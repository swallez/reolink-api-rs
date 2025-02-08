use std::fmt::Debug;
use std::ops::Deref;
use std::sync::Arc;
use std::time::Duration;
use reqwest::{Method, Url};
use bytes::Bytes;
use serde::Serialize;
use tracing::info;
use crate::api::{AuthenticationType, BinaryEndpoint, JsonEndpoint};
use crate::api::security::login::LoginRequest;
use crate::api::security::logout::LogoutRequest;
use crate::common;
use crate::common::{Credentials, Token};

/// A blocking client for the Reolink API.
///
/// Can be cloned cheaply and sent across threads.
#[derive(Clone)]
pub struct ReolinkClient {
    inner: Arc<InnerClient>,
}

impl Debug for ReolinkClient {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ReolinkClient")
            .field("url", &self.inner.url)
            .field("login", &self.inner.credentials.login)
            .finish()
    }
}

struct InnerClient {
    client: reqwest::blocking::Client,
    url: Url,
    credentials: Credentials,
}

impl ReolinkClient {
    pub fn new(mut url: &str, login: String, password: String) -> anyhow::Result<Self> {
        if url.ends_with("/") {
            url = &url[..url.len() - 1];
        }

        let mut url = Url::parse(url)?;
        url.path_segments_mut().unwrap().extend(&["cgi-bin", "api.cgi"]);

        Ok(ReolinkClient {
            inner: Arc::new(InnerClient {
                client: reqwest::blocking::Client::new(),
                url,
                credentials: Credentials::new(login, password),
            })
        })
    }

    /// Authenticate and make sure this client has a valid token.
    pub fn login(&self) -> anyhow::Result<()> {
        self.inner.login()
    }

    /// Release the current authentication token, if any.
    pub fn logout(&self) -> anyhow::Result<()> {
        self.inner.logout()
    }

    pub fn exec<Req: JsonEndpoint>(&self, req: &Req) -> anyhow::Result<Req::Response> {
        self.inner.exec::<Req>(req)
    }

    pub fn exec_with_details<Req: JsonEndpoint>(&self, req: &Req) -> anyhow::Result<(Req::Response, Req::Initial, Req::Range)> {
        self.inner.exec_with_details::<Req>(req)
    }

    pub fn download<Req: BinaryEndpoint>(&self, req: &Req) -> anyhow::Result<Bytes> {
        self.inner.download::<Req>(req)
    }
}

impl InnerClient {
    fn logout(&self) -> anyhow::Result<()> {
        let creds = &self.credentials;
        let token = creds.token.read().unwrap();

        match token.deref() {
            Some(t) if !t.is_expired() => {
                drop(token);
                let result = self.exec(&LogoutRequest{});
                // Clear token
                *creds.token.write().unwrap() = None;
                result?;
            }
            _ => ()
        }

        Ok(())
    }

    /// Ensures this client has a valid token.
    fn login(&self) -> anyhow::Result<()> {
        let creds = &self.credentials;
        let token = creds.token.read().unwrap();
        match token.deref() {
            Some(t) if !t.needs_refresh() => {
                Ok(())
            },
            _ => {
                drop(token); // Release read lock
                let mut token = creds.token.write().unwrap();
                // Got the write lock: recheck
                if token.as_ref().map(|t| t.needs_refresh()).unwrap_or(true) {
                    // No risk of deadlock as this endpoint is Auth::LoginPassword,
                    // so we won't call this function when using it.
                    let resp = self.exec(&LoginRequest::new(&creds.login, &creds.password))?;
                    *token = Some(Token::new(resp.token.name, Duration::from_secs(resp.token.lease_time as u64)));
                }
                Ok(())
            }
        }
    }

    fn ensure_token_if_needed(&self, auth: AuthenticationType) -> anyhow::Result<()> {
        if auth == AuthenticationType::Token {
            self.login()
        } else {
            Ok(())
        }
    }

    fn exec<Req: JsonEndpoint>(&self, req: &Req) -> anyhow::Result<Req::Response> {
        self.ensure_token_if_needed(Req::AUTH)?;
        let request = common::prepare_json_request(&self.client, &self.url, req, &self.credentials, false)?;

        let response = self.client.execute(request)?
            .error_for_status()?
            .bytes()?;
        common::parse_json_response::<Req>(&response)
    }

    fn exec_with_details<Req: JsonEndpoint>(&self, req: &Req) -> anyhow::Result<(Req::Response, Req::Initial, Req::Range)> {
        self.ensure_token_if_needed(Req::AUTH)?;
        let request = common::prepare_json_request(&self.client, &self.url, req, &self.credentials, true)?;

        let response = self.client.execute(request)?
            .error_for_status()?
            .bytes()?;
        common::parse_json_detailed_response::<Req>(&response)
    }

    fn download<Req: BinaryEndpoint>(&self, req: &Req) -> anyhow::Result<Bytes> {
        self.ensure_token_if_needed(Req::AUTH)?;
        let req = common::prepare_download_request(&self.client, &self.url, req, &self.credentials)?;
        let resp = self.client.execute(req)?;
        Ok(resp.bytes()?)
    }
}

impl Drop for InnerClient {
    fn drop(&mut self) {
        self.logout().unwrap_or_else(|err| info!("Logout failed: {:?}", err));
    }
}

impl common::Req for reqwest::blocking::Request {
    fn url_mut(&mut self) -> &mut Url {
        self.url_mut()
    }
}

impl common::HttpClient for reqwest::blocking::Client {
    type Builder = reqwest::blocking::RequestBuilder;
    type Request = reqwest::blocking::Request;
    type Error = reqwest::Error;

    fn new(client: &Self, method: Method, url: Url) -> Self::Builder {
        client.request(method, url)
    }
}

impl common::ReqBuilder for reqwest::blocking::RequestBuilder {
    type Request = reqwest::blocking::Request;
    type Error = reqwest::Error;

    fn query<T: Serialize + ?Sized>(self, query: &T) -> Self {
        self.query(query)
    }

    fn json<T: Serialize + ?Sized>(self, json: &T) -> Self {
        self.json(json)
    }

    fn build(self) -> Result<Self::Request, Self::Error> {
        self.build()
    }
}

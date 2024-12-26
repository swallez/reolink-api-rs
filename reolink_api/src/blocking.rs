use reqwest::{Method, Url};
use bytes::Bytes;
use serde::Serialize;
use crate::api::{BinaryEndpoint, JsonEndpoint};
use crate::api::security::login::LoginRequest;
use crate::api::security::logout::LogoutRequest;
use crate::common;
use crate::common::Credentials;

/// A blocking client for the Reolink API.
pub struct ReolinkClient {
    client: reqwest::blocking::Client,
    url: Url,
    login: String,
    password: String,
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
            client: reqwest::blocking::Client::new(),
            url,
            login: login.clone(),
            password: password.clone(),
            credentials: Credentials::LoginPass { login, password },
        })
    }

    pub fn use_token(&mut self) -> anyhow::Result<()> {
        match &self.credentials {
            Credentials::LoginPass { .. } => {
                let resp = self.exec(&LoginRequest::new(&self.login, &self.password))?;
                self.credentials = Credentials::Token { token: resp.token.name };
            },
            Credentials::Token { .. } => {},
        }

        Ok(())
    }

    pub fn logout(self) -> anyhow::Result<()> {
        self.exec(&LogoutRequest{})?;
        Ok(())
    }

    pub fn exec<Req: JsonEndpoint>(&self, req: &Req) -> anyhow::Result<Req::Response> {
        let request = common::prepare_json_request(&self.client, &self.url, req, &self.credentials, false)?;

        let response = self.client.execute(request)?
            .error_for_status()?
            .bytes()?;
        common::parse_json_response::<Req>(&response)
    }

    pub fn exec_with_details<Req: JsonEndpoint>(&self, req: &Req) -> anyhow::Result<(Req::Response, Req::Initial, Req::Range)> {
        let request = common::prepare_json_request(&self.client, &self.url, req, &self.credentials, true)?;

        let response = self.client.execute(request)?
            .error_for_status()?
            .bytes()?;
        common::parse_json_detailed_response::<Req>(&response)
    }

    pub fn download<Req: BinaryEndpoint>(&self, req: &Req) -> anyhow::Result<Bytes> {
        let req = common::prepare_download_request(&self.client, &self.url, req, &self.credentials)?;
        let resp = self.client.execute(req)?;
        Ok(resp.bytes()?)
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

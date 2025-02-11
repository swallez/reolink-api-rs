use std::sync::RwLock;
use std::time::{Duration, Instant};
use bytes::Bytes;
use reqwest::Url;
use serde::{Deserialize, Deserializer, Serialize};
use serde::de::{DeserializeOwned, Error};
use crate::api::ApiError;
use serde_json::Value as JsonValue;
use crate::api::{AuthenticationType, BinaryEndpoint, JsonEndpoint};

mod url;

pub struct Credentials {
    pub login: String,
    pub password: String,
    pub token: RwLock<Option<Token>>,
}

impl Credentials {
    pub fn new(login: String, password: String) -> Self {
        Credentials { login, password, token: RwLock::new(None) }
    }
}

#[derive(Clone)]
pub struct Token {
    pub value: String,
    pub expires: Instant,
}

impl Token {
    pub fn new(value: String, lease_time: Duration) -> Self {
        Self {
            value,
            expires: Instant::now() + lease_time,
        }
    }

    /// Returns true if this token is expired or about to expire with a grace period.
    pub fn needs_refresh(&self) -> bool {
        // Lease duration is normally 1 hour, remove a 30 secs margin
        self.expires - Duration::from_secs(30) < Instant::now()
    }

    /// Returns true if this token is expired
    pub fn is_expired(&self) -> bool {
        self.expires < Instant::now()
    }
}

/// The base operations that this library expects from an http client. It is modelled after
/// `reqwest`'s client, which provides different (but somewhat similar) types for its
/// blocking and async clients.
pub trait HttpClient {
    type RequestBuilder: ReqBuilder<Request = Self::Request, Error = Self::Error>;
    type Request: Req;
    type Error: std::error::Error + Send + Sync + 'static;
    fn request(client: &Self, method: reqwest::Method, url: reqwest::Url) -> Self::RequestBuilder;
    // Note: no 'fn execute()' here as it can be blocking or async
}

/// The base operations that this request expects from an http client. It is modelled after
/// `reqwest`'s request builder, which provides different (but somewhat similar) types for its
/// blocking and async clients.
pub trait ReqBuilder {
    type Request: Req;
    type Error: std::error::Error + Send + Sync + 'static;
    fn query<T: Serialize + ?Sized>(self, query: &T) -> Self;
    fn timeout(self, timeout: Duration) -> Self;
    fn json<T: Serialize + ?Sized>(self, json: &T) -> Self;
    fn build(self) -> Result<Self::Request, Self::Error>;
}

/// An http request. We need mutable access to the url to tweak the query string encoding.
pub trait Req {
    fn url_mut(&mut self) -> &mut reqwest::Url;
}

pub fn get_api_url(mut url: &str) -> anyhow::Result<Url> {
    if url.ends_with("/") {
        url = &url[..url.len() - 1];
    }

    let mut url = Url::parse(url)?;
    url.path_segments_mut().unwrap().extend(&["cgi-bin", "api.cgi"]);
    Ok(url)
}

// Section independent of the request type (limit code bloat)
fn prepare_request<HC:HttpClient>(
    client: &HC, url: reqwest::Url, cmd: &str, auth: AuthenticationType, creds: &Credentials
) -> anyhow::Result<HC::RequestBuilder> {
    let mut req = HC::request(client, reqwest::Method::POST, url);
    req = req.query(&[("cmd", cmd)]);
    match auth {
        AuthenticationType::None => (),

        AuthenticationType::LoginPassword => {
            req = req.query(&[("user", &creds.login), ("password", &creds.password)]);
        },

        AuthenticationType::Any => {
            // Prefer token if available and not expired
            match creds.token.read().unwrap().as_ref() {
                Some(token) if !token.needs_refresh() => {
                    req = req.query(&[("token", &token.value)]);
                },
                _ => {
                    req = req.query(&[("user", &creds.login), ("password", &creds.password)]);
                }
            }
        },

        AuthenticationType::Token => {
            match creds.token.read().unwrap().as_ref() {
                Some(token) if !token.needs_refresh() => {
                    req = req.query(&[("token", &token.value)]);
                },
                Some(_) => {
                    return Err(anyhow::anyhow!("Token has expired but is required for the '{}' API", cmd))
                }
                None => {
                    return Err(anyhow::anyhow!("A token is required for the '{}' API", cmd))
                }
            }
        }
    }
    Ok(req)
}

fn finalize_request<RB: ReqBuilder>(builder: RB) -> Result<RB::Request, RB::Error> {
    let mut req = builder.build()?;
    let url = req.url_mut();
    // eprintln!("URL = {}", url.to_string());
    url::tweak_url(url);
    Ok(req)
}

/// Prepare a request for an endpoint that returns JSON data. If an auth token is needed,
/// it must be available in `creds`. This function does no token creation or refresh.
pub fn prepare_json_request<HC: HttpClient, APIReq: JsonEndpoint>(
    client: &HC, url: &reqwest::Url, api_req: &APIReq, creds: &Credentials, details: bool
) -> anyhow::Result<HC::Request> {

    let rb = prepare_request(client, url.clone(), APIReq::CMD, APIReq::AUTH, creds)?;

    // Requests are a single object in an array.
    let body = [ApiRequestEnvelope {
        cmd: APIReq::CMD,
        action: if details { Some(1) } else { None },
        param: &api_req
    }];

    //eprintln!("Request body = {}", serde_json::to_string(&body).unwrap());

    let rb = rb.json(&body);
    Ok(finalize_request(rb)?)
}

/// Prepare a request for an endpoint that returns binary data. If an auth token is needed,
/// it must be available in `creds`. This function does no token creation or refresh.
pub fn prepare_download_request<HC: HttpClient, Req: BinaryEndpoint>(
    client: &HC, url: &reqwest::Url, req: &Req, creds: &Credentials
) -> anyhow::Result<HC::Request> {
    let rb = prepare_request(client, url.clone(), Req::CMD, Req::AUTH, creds)?;
    let rb = rb.timeout(Duration::from_secs(5*60));
    let rb = rb.query(req);
    Ok(finalize_request(rb)?)
}

#[derive(Debug, Serialize)]
struct ApiRequestEnvelope<'a, Req: Serialize> {
    cmd: &'static str,
    #[serde(skip_serializing_if = "Option::is_none")]
    action: Option<u8>,
    param: &'a Req,
}

//-------------------------------------------------------------------------------------------------
// Response

pub (crate) fn parse_json_response<APIReq: JsonEndpoint>(bytes: &Bytes) -> anyhow::Result<APIReq::Response> {
    //eprintln!("Response body {}", std::str::from_utf8(bytes).unwrap());
    // Responses are a single object in an array.
    let [result] = serde_json::from_slice::<[ApiResponse<ApiResponseValue<_>>;1]>(bytes)?;
    match result {
        ApiResponse::Success(v) => Ok(v.value),
        ApiResponse::Error(v) => Err(v.into()),
    }
}

pub (crate) fn parse_json_detailed_response<APIReq: JsonEndpoint>(bytes: &Bytes) -> anyhow::Result<(APIReq::Response, APIReq::Initial, APIReq::Range)> {
    //eprintln!("Response body {}", std::str::from_utf8(bytes).unwrap());
    // Responses are a single object in an array.
    let [result] = serde_json::from_slice::<[ApiResponse<ApiResponseValueInitialRange<_, _, _>>;1]>(bytes)?;
    match result {
        ApiResponse::Success(v) => Ok((v.value, v.initial, v.range)),
        ApiResponse::Error(v) => Err(v.into()),
    }
}

#[derive(Debug)]
enum ApiResponse<Value> {
    Success(Value),
    Error(ApiError),
}

#[derive(Debug, Deserialize)]
struct ApiResponseValue<Value> {
    pub value: Value,
}

#[derive(Debug, Deserialize)]
struct ApiResponseValueInitialRange<Value, Initial, Range> {
    pub value: Value,
    pub initial: Initial,
    pub range: Range,
}

impl <'de, T: DeserializeOwned> Deserialize<'de> for ApiResponse<T> {

    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        // Serde currently doesn't handle internal tags that aren't strings, so we have to do it "by hand"
        // https://github.com/serde-rs/serde/issues/745
        // We could use serde's efficient Content API, but it's private. So use serde-json::Value
        // https://github.com/serde-rs/serde/issues/741

        // Section independent of the response type (reduce monomorphization)
        fn get_code_and_json<'de0, D0: Deserializer<'de0>>(deserializer: D0) -> Result<(i64, JsonValue), D0::Error> {
            let mut json = JsonValue::deserialize(deserializer)?;

            let Some(obj) = json.as_object_mut() else {
                return Err(Error::custom("Expecting an object"));
            };
            let Some(_cmd) = obj.remove("cmd") else {
                return Err(Error::missing_field("cmd"));
            };
            let Some(code) = obj.get("code") else {
                return Err(Error::missing_field("code"));
            };
            let Some(code) = code.as_i64() else {
                return Err(Error::custom("'code' is not an integer"));
            };

            // Only keep 'code' if we don't have a successful response
            if code == 0 {
                obj.remove("code");
            }

            Ok((code, json))
        }

        let (code, json) = get_code_and_json(deserializer)?;
        if code == 0 {
            let result = T::deserialize(json).map_err(Error::custom)?;
            Ok(ApiResponse::Success(result))
        } else {
            Ok(ApiResponse::Error(ApiError::deserialize(json).map_err(Error::custom)?))
        }
    }
}


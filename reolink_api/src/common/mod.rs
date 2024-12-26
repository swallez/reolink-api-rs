use bytes::Bytes;
use serde::{Deserialize, Deserializer, Serialize};
use serde::de::{DeserializeOwned, Error};
use crate::api::ApiError;
use serde_json::Value as JsonValue;
use crate::api::{AuthenticationType, BinaryEndpoint, JsonEndpoint};

mod url;

pub enum Credentials {
    LoginPass { login: String, password: String },
    Token { token: String },
}

pub trait HttpClient {
    type Builder: ReqBuilder<Request = Self::Request, Error = Self::Error>;
    type Request: Req;
    type Error: std::error::Error + Send + Sync + 'static;
    fn new(client: &Self, method: reqwest::Method, url: reqwest::Url) -> Self::Builder;
    // Note: no 'fn execute()' here as it can be blocking or async
}

pub trait ReqBuilder {
    type Request: Req;
    type Error: std::error::Error + Send + Sync + 'static;
    fn query<T: Serialize + ?Sized>(self, query: &T) -> Self;
    fn json<T: Serialize + ?Sized>(self, json: &T) -> Self;
    fn build(self) -> Result<Self::Request, Self::Error>;
}

pub trait Req {
    fn url_mut(&mut self) -> &mut reqwest::Url;
}

// Section independent of the request type (limit code bloat)
fn prepare_request_base<HC:HttpClient>(
    client: &HC, url: reqwest::Url, cmd: &str, auth: AuthenticationType, creds: &Credentials
) -> anyhow::Result<HC::Builder> {
    let mut req = HC::new(client, reqwest::Method::POST, url);
    req = req.query(&[("cmd", cmd)]);
    if auth != AuthenticationType::None {
        match creds {
            Credentials::Token { token } => {
                req = req.query(&[("token", token)]);
            }
            Credentials::LoginPass { .. } if auth == AuthenticationType::Token => {
                return Err(anyhow::anyhow!("A token is required for the '{}' API", cmd))
            }
            Credentials::LoginPass { login, password } => {
                req = req.query(&[("user", login), ("password", password)]);
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

pub (crate) fn prepare_json_request<HC: HttpClient, APIReq: JsonEndpoint>(
    client: &HC, url: &reqwest::Url, api_req: &APIReq, creds: &Credentials, details: bool
) -> anyhow::Result<HC::Request> {

    let rb = prepare_request_base(client, url.clone(), APIReq::CMD, APIReq::AUTH, creds)?;

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

pub fn prepare_download_request<HC: HttpClient, Req: BinaryEndpoint>(client: &HC, url: &reqwest::Url, req: &Req, creds: &Credentials) -> anyhow::Result<HC::Request> {
    let rb = prepare_request_base(client, url.clone(), Req::CMD, Req::AUTH, creds)?;
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

        // Non-monomorphized section
        fn get_code<'de0, D0: Deserializer<'de0>>(json: &mut JsonValue) -> Result<i64, D0::Error> {
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

            Ok(code)
        }

        let mut json = JsonValue::deserialize(deserializer)?;
        let code = get_code::<D>(&mut json)?;
        if code == 0 {
            let result = T::deserialize(json).map_err(Error::custom)?;
            Ok(ApiResponse::Success(result))
        } else {
            Ok(ApiResponse::Error(ApiError::deserialize(json).map_err(Error::custom)?))
        }
    }
}


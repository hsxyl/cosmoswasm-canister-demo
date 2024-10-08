use crate::*;
use base64::engine::general_purpose;
use candid::Nat;
use error::RouteError;
use ic_cdk::api::management_canister::http_request::{self, http_request, CanisterHttpRequestArgument, HttpResponse};

/// JSON-RPC ID: request-specific identifier
#[derive(Clone, Debug, Deserialize, Serialize, Eq, PartialEq, Ord, PartialOrd)]
#[serde(untagged)]
pub enum Id {
    /// Numerical JSON ID
    Num(i64),
    /// String JSON ID
    Str(String),
    /// null JSON ID
    None,
}

impl Id {
    /// Create a JSON-RPC ID containing a UUID v4 (i.e. random)
    pub fn uuid_v4() -> Self {
        Self::Str(uuid_str())
    }
}

pub fn uuid_str() -> String {
    // let bytes: [u8; 16] = rand::thread_rng().gen();
    // todo use icp native random number generator
    let bytes: [u8; 16] = [1; 16];
    let uuid = uuid::Builder::from_random_bytes(bytes).into_uuid();
    uuid.to_string()
}

pub async fn http_request_with_status_check(
    request: CanisterHttpRequestArgument,
) ->Result<HttpResponse> {
    let response =  http_request(request.clone(), 100_000_000_000)
            .await
            .map_err(|(code, message)| {
                RouteError::HttpOutCallError(format!("{:?}", code).to_string(), message)
            })?.0;
    if response.status != Nat::from(200u64) {
        return Err(RouteError::HttpStatusError(response.status.clone(), request.url.clone(), String::from_utf8_lossy(&response.body).to_string()));
    }
    Ok(response)
}

pub fn bytes_to_base64(bytes: &[u8]) -> String {
    base64::Engine::encode(&general_purpose::STANDARD, bytes)
}
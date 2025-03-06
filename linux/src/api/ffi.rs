use flutter_rust_bridge::ZeroCopyBuffer;
use serde::{Deserialize, Serialize};

use flutter_rust_bridge::frb;

#[frb(unignore)]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AuthRequestData {
    pub domain: String,
}

pub fn web_authentication_login(
    request: AuthRequestData,
) -> flutter_rust_bridge::ZeroCopyBuffer<Vec<u8>> {
    println!("{:?}", request);
    let response = vec![];
    ZeroCopyBuffer(response)
}

pub fn web_authentication_logout(
    _domain: String,
    _client_id: String,
    _return_to: Option<String>,
) -> flutter_rust_bridge::ZeroCopyBuffer<Vec<u8>> {
    let response = vec![];
    ZeroCopyBuffer(response)
}

use flutter_rust_bridge::ZeroCopyBuffer;
use serde::{Deserialize, Serialize};

use flutter_rust_bridge::frb;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AuthRequestData {
    pub domain: String,
}

#[frb(unignore)]
pub fn web_authentication_login(
    request: AuthRequestData,
) -> Vec<u8> {
    println!("{:?}", request);
    let response = vec![];
    ZeroCopyBuffer(response)
}

pub fn web_authentication_logout(
    _domain: String,
    _client_id: String,
    _return_to: Option<String>,
) -> Vec<u8> {
    let response = vec![];
    ZeroCopyBuffer(response)
}

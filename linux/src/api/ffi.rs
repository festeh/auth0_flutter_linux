use flutter_rust_bridge::ZeroCopyBuffer;
use serde::{Deserialize, Serialize};

use flutter_rust_bridge::frb;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AuthRequestData {}

#[frb]
pub fn web_authentication_login(
    request: AuthRequestData,
) -> flutter_rust_bridge::ZeroCopyBuffer<Vec<u8>> {
    println!("{:?}", request);
    let response = vec![];
    ZeroCopyBuffer(response)
}

#[frb]
pub fn web_authentication_logout(
    domain: String,
    client_id: String,
    return_to: Option<String>,
) -> flutter_rust_bridge::ZeroCopyBuffer<Vec<u8>> {
    let response = vec![];
    ZeroCopyBuffer(response)
}

use flutter_rust_bridge::ZeroCopyBuffer;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AuthRequestData {}

pub fn web_authentication_login(request: AuthRequestData) -> ZeroCopyBuffer<Vec<u8>> {
    println!("{:?}", request);
    let response = vec![];
    ZeroCopyBuffer(response)
}

pub fn web_authentication_logout(
    domain: String,
    client_id: String,
    return_to: Option<String>,
) -> ZeroCopyBuffer<Vec<u8>> {
    let response = vec![];
    ZeroCopyBuffer(response)
}

use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AuthRequestData {
    pub domain: String,
}

pub fn web_authentication_login(request: AuthRequestData) -> Vec<u8> {
    println!("Request {:?}", request);
    let response = vec![];
    response
}

pub fn web_authentication_logout(
    _domain: String,
    _client_id: String,
    _return_to: Option<String>,
) -> Vec<u8> {
    let response = vec![];
    response
}

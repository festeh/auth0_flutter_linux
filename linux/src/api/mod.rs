mod ffi;
mod web_auth;

use flutter_rust_bridge::frb;

#[frb]
pub fn web_authentication_login(
    request: ffi::AuthRequestData,
) -> flutter_rust_bridge::ZeroCopyBuffer<Vec<u8>> {
    ffi::web_authentication_login(request)
}

#[frb]
pub fn web_authentication_logout(
    domain: String,
    client_id: String,
    return_to: Option<String>,
) -> flutter_rust_bridge::ZeroCopyBuffer<Vec<u8>> {
    ffi::web_authentication_logout(domain, client_id, return_to)
}

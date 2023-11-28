service_sdk::macros::use_my_http_server!();

use my_http_server::HttpContext;

const AUTH_HEADER: &str = "authorization";
const API_KEY_HEADER: &str = "x-api-key";

pub trait GetSessionToken {
    fn get_session_token(&self) -> Option<&str>;
}

pub trait GetSessionApiKey {
    fn get_session_api_key(&self) -> Option<&str>;
}

impl GetSessionToken for HttpContext {
    fn get_session_token(&self) -> Option<&str> {
        let auth_header = self
            .request
            .get_headers()
            .try_get_case_insensitive(AUTH_HEADER)?;

        let token = extract_token(auth_header.value)?;

        match std::str::from_utf8(token) {
            Ok(result) => Some(result),
            Err(_) => None,
        }
    }
}

impl GetSessionApiKey for HttpContext {
    fn get_session_api_key(&self) -> Option<&str> {
        let auth_header = self
            .request
            .get_headers()
            .try_get_case_insensitive(API_KEY_HEADER)?;

        match std::str::from_utf8(auth_header.value) {
            Ok(result) => Some(result),
            Err(_) => None,
        }
    }
}

fn extract_token(src: &[u8]) -> Option<&[u8]> {
    if src.len() < 7 {
        return None;
    }
    if src[6] == b' ' {
        return Some(&src[7..]);
    }
    Some(src)
}

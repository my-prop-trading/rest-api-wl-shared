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
        let auth_header = self.request.get_header(AUTH_HEADER)?;

        let bytes = auth_header.as_bytes();
        let token = extract_token(bytes)?;

        match std::str::from_utf8(token) {
            Ok(result) => {
                Some(result)
            },
            Err(_) => None,
        }
    }
}

impl GetSessionApiKey for HttpContext {
    fn get_session_api_key(&self) -> Option<&str> {
        let auth_header = self.request.get_header(API_KEY_HEADER)?;

        let bytes = auth_header.as_bytes();

        match std::str::from_utf8(bytes) {
            Ok(result) => {
                Some(result)
            },
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

#[cfg(test)]
mod tests {
    use service_sdk::{
        flurl::hyper::{header::HeaderValue, Body, Request},
        my_http_server::{HttpContext, HttpRequest},
    };
    use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};

    use crate::middlewares::GetSessionApiKey;

    use super::GetSessionToken;

    #[test]
    fn test_get_session_token() {
        let body = Body::empty();
        let mut body = Request::<Body>::new(body);
        body.headers_mut().append(
            super::AUTH_HEADER,
            HeaderValue::from_str("Bearer 1234567890").unwrap(),
        );
        let addr = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(0, 0, 0, 0), 5000));
        let http_req = HttpRequest::new(body, addr);
        let http_ctx = HttpContext::new(http_req);
        let session_token = http_ctx.get_session_token().unwrap();
        
        assert_eq!("1234567890", session_token);
    }

    #[test]
    fn test_get_session_api_key() {
        let body = Body::empty();
        let mut body = Request::<Body>::new(body);
        body.headers_mut().append(
            super::API_KEY_HEADER,
            HeaderValue::from_str("1234567890").unwrap(),
        );
        let addr = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(0, 0, 0, 0), 5000));
        let http_req = HttpRequest::new(body, addr);
        let http_ctx = HttpContext::new(http_req);
        let session_token = http_ctx.get_session_api_key().unwrap();
        
        assert_eq!("1234567890", session_token);
    }
}

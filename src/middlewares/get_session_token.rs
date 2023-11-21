service_sdk::macros::use_my_http_server!();

use my_http_server::HttpContext;

const AUTH_HEADER: &str = "authorization";

#[derive(Debug, PartialEq)]
pub enum AuthScheme {
    Bearer,
    ApiKey,
}

pub trait GetSessionToken {
    fn get_session_token(&self) -> Option<(&str, AuthScheme)>;
}

impl GetSessionToken for HttpContext {
    fn get_session_token(&self) -> Option<(&str, AuthScheme)> {
        let auth_header = self.request.get_header(AUTH_HEADER)?;

        let bytes = auth_header.as_bytes();
        let token = extract_token(bytes)?;

        match std::str::from_utf8(token) {
            Ok(result) => {
                let scheme = std::str::from_utf8(&bytes[0..7]).unwrap_or("bearer");
                let scheme = match scheme {
                    "bearer" => AuthScheme::Bearer,
                    "apikey" => AuthScheme::ApiKey, 
                    _ => AuthScheme::Bearer,
                };

                Some((result, scheme))
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

    use crate::middlewares::AuthScheme;

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
        
        assert_eq!(AuthScheme::Bearer, session_token.1);
        assert_eq!("1234567890", session_token.0);
    }
}

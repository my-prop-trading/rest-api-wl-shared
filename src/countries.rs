use service_sdk::my_http_server::{HttpContext, HttpFailResult};

pub fn validate_country(_ctx: &HttpContext, value: &str) -> Result<(), HttpFailResult> {;
    let valid_code = rust_common::country_code::CountryCode::parse(value);

    match valid_code {
        Ok(_) => {return Ok(());}
        Err(_) => {
            return Err(HttpFailResult::as_validation_error(
                "Invalid country".to_string(),
            ));
        }
    }
}

pub fn validate_country_optional(
    _ctx: &HttpContext,
    value: &Option<String>,
) -> Result<(), HttpFailResult> {
    let Some(value) = value else {
        return Ok(());
    };

    validate_country(_ctx, value)
}

/* #[cfg(test)]
mod test {
    use super::validate_country;
    use service_sdk::{
        flurl::hyper::{self, Method},
        my_http_server::{HttpContext, HttpFailResult, HttpPath, HttpRequest, RequestData},
    };
    use std::{
        collections::HashMap,
        net::{Ipv4Addr, SocketAddrV4},
    };

    #[test]
    pub fn test_validate_country() {
        let req: HttpRequest = HttpRequest {
            data: RequestData:: ,
            addr: std::net::SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 8080)),
            content_type_header: None,
            method: Method::GET,
            http_path: HttpPath::from_str("/"),
            key_values: None,
        };
        let mut ctx = HttpContext::new(req);

        let result = validate_country(&ctx, &"United States of Bitcoin".to_string());
        assert!(!result.is_ok());
    }
}
 */
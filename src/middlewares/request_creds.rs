service_sdk::macros::use_my_http_server!();

use std::sync::Arc;

use my_http_server::{RequestCredentials, RequestClaim};
use service_sdk::rust_extensions::date_time::DateTimeAsMicroseconds;

use super::SessionEntityTrait;

pub struct TradingPlatformRequestCredentials {
    pub session_entity: Arc<dyn SessionEntityTrait + Send + Sync>,
}

impl TradingPlatformRequestCredentials {
    pub fn new(session_entity: Arc<dyn SessionEntityTrait + Send + Sync>) -> Self {
        Self { session_entity }
    }
}

impl RequestCredentials for TradingPlatformRequestCredentials {
    fn get_id(&self) -> &str {
        &self.session_entity.get_id()
    }

    fn get_claims<'s>(&'s self) -> Option<Vec<my_http_server::RequestClaim<'s>>> {
        let claims = self.session_entity.get_claims();
        return if claims.is_empty() {
            None
        } else {
            let mapped: Vec<RequestClaim> = claims
                .iter()
                .map(|c| {
                    let expires = DateTimeAsMicroseconds {
                        unix_microseconds: c.expires * 1000,
                    };

                    RequestClaim {
                        allowed_ips: None,
                        expires,
                        id: &c.id,
                    }
                })
                .collect();

            Some(mapped)
        };
    }
}

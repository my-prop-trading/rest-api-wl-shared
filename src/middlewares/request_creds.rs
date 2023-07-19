use std::sync::Arc;

use my_http_server::{RequestCredentials, RequestClaim};
use rust_extensions::date_time::DateTimeAsMicroseconds;

use super::SessionEntity;

pub struct TradingPlatformRequestCredentials {
    pub session_entity: Arc<SessionEntity>,
}

impl TradingPlatformRequestCredentials {
    pub fn new(session_entity: Arc<SessionEntity>) -> Self {
        Self { session_entity }
    }
}

impl RequestCredentials for TradingPlatformRequestCredentials {
    fn get_id(&self) -> &str {
        &self.session_entity.trader_id
    }

    fn get_claims<'s>(&'s self) -> Option<Vec<my_http_server::RequestClaim<'s>>> {
        return if self.session_entity.claims.is_empty() {
            None
        } else {
            let mapped: Vec<RequestClaim> = self
                .session_entity
                .claims
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

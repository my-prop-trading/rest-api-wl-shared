use std::sync::Arc;

use my_http_server::RequestCredentials;

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
        None
    }
}

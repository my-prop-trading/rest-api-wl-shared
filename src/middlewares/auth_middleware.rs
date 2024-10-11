service_sdk::macros::use_my_http_server!();
use std::sync::Arc;

use my_http_server::{HttpContext, HttpFailResult, HttpOkResult, HttpServerMiddleware};
use service_sdk::my_no_sql_sdk::reader::MyNoSqlDataReaderTcp;

use crate::KV_BRAND_ID;

use super::{
    GetSessionApiKey, GetSessionToken, OpenApiKeyEntity, SessionEntity,
    TradingPlatformRequestCredentials,
};

pub struct AuthSessionMiddleware {
    sessions_reader: Arc<MyNoSqlDataReaderTcp<SessionEntity>>,
}

pub struct AuthApiKeyMiddleware {
    api_key_reader: Arc<MyNoSqlDataReaderTcp<OpenApiKeyEntity>>,
}

impl AuthSessionMiddleware {
    pub fn new(sessions_reader: Arc<MyNoSqlDataReaderTcp<SessionEntity>>) -> Self {
        Self { sessions_reader }
    }
}

impl AuthApiKeyMiddleware {
    pub fn new(api_key_reader: Arc<MyNoSqlDataReaderTcp<OpenApiKeyEntity>>) -> Self {
        Self { api_key_reader }
    }
}

#[async_trait::async_trait]
impl HttpServerMiddleware for AuthSessionMiddleware {
    async fn handle_request(
        &self,
        ctx: &mut HttpContext,
    ) -> Option<Result<HttpOkResult, HttpFailResult>> {
        let session_token = ctx.get_session_token();

        if session_token.is_none() {
            return None;
        }

        let session_id = session_token.unwrap();

        let token_entity = self
            .sessions_reader
            .get_entity(&SessionEntity::get_pk(), session_id)
            .await;

        if token_entity.is_none() {
            return None;
        }

        let token_entity = token_entity.unwrap();

        let brand_id = token_entity.brand_id.clone();
        ctx.request
            .set_key_value(KV_BRAND_ID.to_string(), brand_id.into_bytes());

        ctx.credentials = Some(Box::new(TradingPlatformRequestCredentials::new(
            token_entity,
        )));

        None
    }
}

#[async_trait::async_trait]
impl HttpServerMiddleware for AuthApiKeyMiddleware {
    async fn handle_request(
        &self,
        ctx: &mut HttpContext,
    ) -> Option<Result<HttpOkResult, HttpFailResult>> {
        let session_token = ctx.get_session_api_key();

        if session_token.is_none() {
            return None;
        }

        let session_id = session_token.unwrap();

        let token_entity = self
            .api_key_reader
            .get_entity(&OpenApiKeyEntity::get_pk(), session_id)
            .await;

        if token_entity.is_none() {
            return None;
        }

        let token_entity = token_entity.unwrap();

        let brand_id = token_entity.brand_id.clone();
        ctx.request
            .set_key_value(KV_BRAND_ID.to_string(), brand_id.into_bytes());

        ctx.credentials = Some(Box::new(TradingPlatformRequestCredentials::new(
            token_entity,
        )));

        None
    }
}

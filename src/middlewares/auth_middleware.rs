service_sdk::macros::use_my_http_server!();
use std::sync::Arc;

use my_http_server::{
    HttpContext, HttpFailResult, HttpOkResult, HttpServerMiddleware, HttpServerRequestFlow,
};
use service_sdk::my_no_sql_sdk::reader::MyNoSqlDataReader;

use crate::KV_BRAND_ID;

use super::{
    AuthScheme, GetSessionToken, OpenApiKeyEntity, SessionEntity, TradingPlatformRequestCredentials,
};

pub struct AuthSessionMiddleware {
    sessions_reader: Arc<dyn MyNoSqlDataReader<SessionEntity> + Send + Sync + 'static>,
}

pub struct AuthApiKeyMiddleware {
    api_key_reader: Arc<dyn MyNoSqlDataReader<OpenApiKeyEntity> + Send + Sync + 'static>,
}

impl AuthSessionMiddleware {
    pub fn new(
        sessions_reader: Arc<dyn MyNoSqlDataReader<SessionEntity> + Send + Sync + 'static>,
    ) -> Self {
        Self {
            sessions_reader,
        }
    }
}


impl AuthApiKeyMiddleware {
    pub fn new(
        api_key_reader: Arc<dyn MyNoSqlDataReader<OpenApiKeyEntity> + Send + Sync + 'static>,
    ) -> Self {
        Self {
            api_key_reader,
        }
    }
}

#[async_trait::async_trait]
impl HttpServerMiddleware for AuthSessionMiddleware {
    async fn handle_request(
        &self,
        ctx: &mut HttpContext,
        get_next: &mut HttpServerRequestFlow,
    ) -> Result<HttpOkResult, HttpFailResult> {
        let session_token = ctx.get_session_token();

        if session_token.is_none() {
            return get_next.next(ctx).await;
        }

        let (session_id, auth_scheme) = session_token.unwrap();

        //Only process bearer reqquests!
        match auth_scheme {
            AuthScheme::Bearer => {}
            _ => return get_next.next(ctx).await,
        }

        let token_entity = self
            .sessions_reader
            .get_entity(&SessionEntity::get_pk(), session_id)
            .await;

        if token_entity.is_none() {
            return get_next.next(ctx).await;
        }

        let token_entity = token_entity.unwrap();

        let brand_id = token_entity.brand_id.clone();
        ctx.request
            .set_key_value(KV_BRAND_ID.to_string(), brand_id.into_bytes());

        ctx.credentials = Some(Box::new(TradingPlatformRequestCredentials::new(
            token_entity,
        )));

        get_next.next(ctx).await
    }
}

#[async_trait::async_trait]
impl HttpServerMiddleware for AuthApiKeyMiddleware {
    async fn handle_request(
        &self,
        ctx: &mut HttpContext,
        get_next: &mut HttpServerRequestFlow,
    ) -> Result<HttpOkResult, HttpFailResult> {
        let session_token = ctx.get_session_token();

        if session_token.is_none() {
            return get_next.next(ctx).await;
        }

        let (session_id, auth_scheme) = session_token.unwrap();

        //Only process apikeys requests!
        match auth_scheme {
            AuthScheme::ApiKey => {}
            _ => return get_next.next(ctx).await,
        }

        let token_entity = self
            .api_key_reader
            .get_entity(&OpenApiKeyEntity::get_pk(), session_id)
            .await;

        if token_entity.is_none() {
            return get_next.next(ctx).await;
        }

        let token_entity = token_entity.unwrap();

        let brand_id = token_entity.brand_id.clone();
        ctx.request
            .set_key_value(KV_BRAND_ID.to_string(), brand_id.into_bytes());

        ctx.credentials = Some(Box::new(TradingPlatformRequestCredentials::new(
            token_entity,
        )));

        get_next.next(ctx).await
    }
}
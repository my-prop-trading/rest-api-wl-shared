service_sdk::macros::use_my_http_server!();

use serde::{Deserialize, Serialize};
use service_sdk::my_no_sql_sdk::{self, abstractions::Timestamp};

pub const SESSION_PARTITION_KEY_VALUE: &str = "t";

//write trait that unites SessionEntity and OpenApiKeyEntity
pub trait SessionEntityTrait {
    fn get_id(&self) -> &str;

    fn get_brand_id(&self) -> &str;

    fn get_claims(&self) -> &Vec<AccessClaim>;
}

#[service_sdk::my_no_sql_sdk::macros::my_no_sql_entity("sessionsentites")]
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SessionEntity {
    #[serde(rename = "TraderId")]
    pub trader_id: String,

    #[serde(rename = "Expires")]
    pub expires: Timestamp,

    #[serde(rename = "BrandId")]
    pub brand_id: String,

    #[serde(rename = "Ip")]
    pub ip: String,

    #[serde(rename = "Claims")]
    pub claims: Vec<AccessClaim>,
}

#[service_sdk::my_no_sql_sdk::macros::my_no_sql_entity("open-api-keys")]
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OpenApiKeyEntity {
    #[serde(rename = "ApiKey")]
    pub api_key_id: String,

    #[serde(rename = "Expires")]
    pub expires: Timestamp,

    #[serde(rename = "BrandId")]
    pub brand_id: String,

    #[serde(rename = "Ip")]
    pub ip: String,

    #[serde(rename = "Claims")]
    pub claims: Vec<AccessClaim>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct AccessClaim {
    #[serde(rename = "Id")]
    pub id: String,

    #[serde(rename = "Expires")]
    pub expires: Timestamp,
}

impl SessionEntity {
    pub fn get_pk() -> String {
        SESSION_PARTITION_KEY_VALUE.to_string()
    }

    pub fn get_session_token(&self) -> &str {
        &self.row_key
    }
}

impl OpenApiKeyEntity {
    pub fn get_pk() -> String {
        SESSION_PARTITION_KEY_VALUE.to_string()
    }

    pub fn get_session_token(&self) -> &str {
        &self.row_key
    }
}

impl SessionEntityTrait for SessionEntity {
    fn get_id(&self) -> &str {
        &self.trader_id
    }

    fn get_brand_id(&self) -> &str {
        &self.brand_id
    }

    fn get_claims(&self) -> &Vec<AccessClaim> {
        self.claims.as_ref()
    }
}

impl SessionEntityTrait for OpenApiKeyEntity {
    fn get_id(&self) -> &str {
        &self.api_key_id
    }

    fn get_brand_id(&self) -> &str {
        &self.brand_id
    }

    fn get_claims(&self) -> &Vec<AccessClaim> {
        self.claims.as_ref()
    }
}

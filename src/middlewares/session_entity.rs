use serde::{Deserialize, Serialize};

pub const SESSION_PARTITION_KEY_VALUE: &str = "t";

#[my_no_sql_macros::my_no_sql_entity("sessionsentites")]
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SessionEntity {
    #[serde(rename = "TraderId")]
    pub trader_id: String,
    #[serde(rename = "Expires")]
    pub expires: String,
}

impl SessionEntity {
    pub fn get_pk() -> String {
        SESSION_PARTITION_KEY_VALUE.to_string()
    }

    pub fn get_session_token(&self) -> &str {
        &self.row_key
    }
}

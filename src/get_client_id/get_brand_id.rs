service_sdk::macros::use_my_http_server!();
use my_http_server::{HttpContext, HttpFailResult};

pub const KV_BRAND_ID: &str = "BRAND_ID";

pub trait GetBrandId {
    fn get_brand_id(&self) -> Result<&str, HttpFailResult>;
}

impl GetBrandId for HttpContext {
    fn get_brand_id(&self) -> Result<&str, HttpFailResult> {
        if let Some(client) = self.request.get_key_value(KV_BRAND_ID) {
            let brand_id = std::str::from_utf8(client).unwrap();
            return Ok(brand_id);
        }

        return Err(HttpFailResult::as_unauthorized(Some(
            "Can not get brand id Looks like request is unauthorized".to_string(),
        )));
    }
}

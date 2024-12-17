service_sdk::macros::use_my_http_server!();
use my_http_server::HttpContext;

pub const KV_CLICK_ID: &str = "click_id";
pub const KV_CNV_ID: &str = "cnv_id";
pub const KV_UNKNOWN: &str = "unknown";
pub const COOKIE_CLICK_ID: &str = "clickid";
pub const COOKIE_CNV_ID: &str = "cnv_id";

pub trait GetCookieClickParameter {
    fn get_cookie_click_id(&self) -> Option<String>;
    fn get_cookie_cnv_id(&self) -> Option<String>;
    fn get_kv_from_cookie(&self, name: &str) -> &str;
}
impl GetCookieClickParameter for HttpContext {
    fn get_cookie_click_id(&self) -> Option<String> {
        self.request
            .get_cookies()
            .get(COOKIE_CLICK_ID)
            .map(|value| value.to_string())
    }

    fn get_cookie_cnv_id(&self) -> Option<String> {
        self.request
            .get_cookies()
            .get(COOKIE_CNV_ID)
            .map(|value| value.to_string())
    }

    fn get_kv_from_cookie(&self, name: &str) -> &str {
        match name {
            COOKIE_CLICK_ID => KV_CLICK_ID,
            COOKIE_CNV_ID => KV_CNV_ID,
            _ => KV_UNKNOWN,
        }
    }
}

service_sdk::macros::use_my_http_server!();
use my_http_server::HttpContext;

pub const COOKIE_CLICK_ID: &str = "clickid";
pub const COOKIE_CNV_ID: &str = "cnv_id";

pub trait GetCookieClickParameter {
    fn get_cookie_click_id(&self) -> Option<String>;
    fn get_cookie_cnv_id(&self) -> Option<String>;
}
impl GetCookieClickParameter for HttpContext {
    fn get_cookie_click_id(&self) -> Option<String> {
        self.request
            .get_cookies()
            .get(COOKIE_CLICK_ID)
            .map(|language| language.to_string())
    }

    fn get_cookie_cnv_id(&self) -> Option<String> {
        self.request
            .get_cookies()
            .get(COOKIE_CNV_ID)
            .map(|language| language.to_string())
    }
}

service_sdk::macros::use_my_http_server!();
use my_http_server::HttpContext;

pub trait GetCookieClickParameter {
    fn get_cookie(&self, name: &str) -> Option<String>;
}
impl GetCookieClickParameter for HttpContext {
    fn get_cookie(&self, name: &str) -> Option<String> {
        self.request
            .get_cookies()
            .get(name)
            .map(|value| value.to_string())
    }
}

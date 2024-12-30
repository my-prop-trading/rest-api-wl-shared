service_sdk::macros::use_my_http_server!();
use my_http_server::HttpContext;

pub trait GetCookie {
    fn get_cookie(&self, name: &str) -> Option<String>;
}
impl GetCookie for HttpContext {
    fn get_cookie(&self, name: &str) -> Option<String> {
        self.request
            .get_cookies()
            .get(name)
            .map(|value| value.to_string())
    }
}

impl GetCookie for &mut HttpContext {
    fn get_cookie(&self, name: &str) -> Option<String> {
        (**self).get_cookie(name)
    }
}
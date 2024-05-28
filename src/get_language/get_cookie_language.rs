service_sdk::macros::use_my_http_server!();
use my_http_server::HttpContext;

pub const COOKIE_LANGUAGE: &str = "lang";

pub trait GetCookieLanguage {
    fn get_cookie_language(&self) -> Option<String>;
}
impl GetCookieLanguage for HttpContext {
    fn get_cookie_language(&self) -> Option<String> {
        self.request
            .get_cookies()
            .get(COOKIE_LANGUAGE)
            .map(|language| language.to_string())
    }
}

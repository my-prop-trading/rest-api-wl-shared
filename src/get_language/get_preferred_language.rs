service_sdk::macros::use_my_http_server!();
use my_http_server::HttpContext;
use crate::{GetCookieLanguage, GetAcceptLanguage};

pub const DEFAULT_LANGUAGE: &str = "en";

pub fn get_preferred_language(supported_languages: Vec<&str>, http_context: HttpContext) -> String {
    // check cookies
    if let Some(language) = http_context.get_cookie_language() {
        if supported_languages.contains(&language.as_str()) {
            return language.clone();
        }
        else {
            return DEFAULT_LANGUAGE.to_string();
        }
    }
    // check accept-languages
    if let Some(languages) = http_context.get_accept_language() {
        for language in languages.iter() {
            if supported_languages.contains(&language.as_str()) {
                return language.clone();
            }
        }
    }

    DEFAULT_LANGUAGE.to_string()
}

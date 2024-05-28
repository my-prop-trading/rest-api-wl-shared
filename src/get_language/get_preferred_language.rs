service_sdk::macros::use_my_http_server!();
use my_http_server::HttpContext;
use crate::{GetCookieLanguage, GetAcceptLanguage};

pub const DEFAULT_LANGUAGE: &str = "en";

pub trait GetPreferredLanguage {
    fn get_preferred_language(&self, supported_languages: Vec<&str>) -> String;
}
impl GetPreferredLanguage for HttpContext {
    fn get_preferred_language(&self, supported_languages: Vec<&str>) -> String {
        
        // check cookies
        if let Some(language) = self.get_cookie_language() {
            if supported_languages.contains(&language.as_str()) {
                return language.clone();
            }
            else {
                return DEFAULT_LANGUAGE.to_string();
            }
        }

        // check accept-languages
        if let Some(languages) = self.get_accept_language() {
            for language in languages.iter() {
                if supported_languages.contains(&language.as_str()) {
                    return language.clone();
                }
            }
        }
    
        DEFAULT_LANGUAGE.to_string()
    }
}

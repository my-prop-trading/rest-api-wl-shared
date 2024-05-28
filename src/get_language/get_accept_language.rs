service_sdk::macros::use_my_http_server!();
use my_http_server::HttpContext;

pub const ACCEPT_LANGUAGE: &str = "Accept-Language";

pub trait GetAcceptLanguage {
    fn get_accept_language(&self) -> Option<Vec<String>>;
}
impl GetAcceptLanguage for HttpContext {
    fn get_accept_language(&self) -> Option<Vec<String>> {
        let accept_language_header = self
            .request
            .get_headers()
            .try_get_case_insensitive(ACCEPT_LANGUAGE)?;

        if let Ok(languages) = std::str::from_utf8(accept_language_header.value) {
            let ordered_iso2_languages = get_ordered_iso2_languages(&languages);
            return Some(ordered_iso2_languages);
        }

        None
    }
}

// Function to get the ordered list of ISO2 codes from the header
fn get_ordered_iso2_languages(header: &str) -> Vec<String> {
    parse_accept_language(header)
        .into_iter()
        .map(|(lang, _)| {
            let parts: Vec<&str> = lang.split('-').collect();
            parts[0].to_string()
        })
        .collect()
}

// Function to parse the Accept-Language header
fn parse_accept_language(header: &str) -> Vec<(&str, f32)> {
    let mut languages: Vec<(&str, f32)> = header
        .split(',')
        .map(|lang| {
            let parts: Vec<&str> = lang.split(';').collect();
            let language = parts[0].trim();
            let quality = if parts.len() > 1 && parts[1].starts_with("q=") {
                parts[1][2..].parse::<f32>().unwrap_or(1.0)
            } else {
                1.0
            };
            (language, quality)
        })
        .collect();
    
    // Sort languages by quality in descending order
    languages.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
    languages
}


#[cfg(test)]
mod test {
    use crate::get_language::get_accept_language::get_ordered_iso2_languages;

    #[test]
    pub fn test_accept_language() {
        let languages = "en-US,en;q=0.9,fr;q=0.8,de;q=0.7";
        let ordered_iso2_languages = get_ordered_iso2_languages(&languages);

        assert_eq!(ordered_iso2_languages, vec!["en", "en", "fr", "de"]);
    }
}

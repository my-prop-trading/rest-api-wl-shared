pub const HEADER_CF_IP_COUNTRY: &str = "CF-IPCountry";
pub const HEADER_SITE_VER: &str = "Sitever";
pub const HEADER_USER_AGENT: &str = "User-Agent";

service_sdk::macros::use_my_http_server!();
use my_http_server::{HttpContext, HttpFailResult};
use service_sdk::my_telemetry;

pub trait GetHeader {
    fn get_header(&self, header_name: &'static str) -> Result<String, String>;
    fn get_header_as_http_fail(&self, header_name: &'static str) -> Result<String, HttpFailResult>;
}

impl GetHeader for HttpContext {
    fn get_header(&self, header_name: &'static str) -> Result<String, String> {
        let header = self
            .request
            .get_headers()
            .try_get_case_insensitive(header_name)
            .ok_or_else(|| format!("Required header [{}] is missing", header_name))?;

        let result = std::str::from_utf8(header.value).map_err(|_| {
            format!(
                "Cannot parse header value in '{}'",
                header_name
            )
        })?;

        Ok(result.to_string())
    }

    fn get_header_as_http_fail(&self, header_name: &'static  str) -> Result<String, HttpFailResult> {
        let header = self
            .request
            .get_headers()
            .try_get_case_insensitive(header_name)
            .ok_or_else(|| {
                required_header_is_missing(header_name.to_string())
    }       )?;

        let result = std::str::from_utf8(header.value).map_err(|_| {
            HttpFailResult::invalid_value_to_parse(format!(
                "Cannot parse header value in '{}'",
                header_name
            ))
        })?;

        Ok(result.to_string())
    }

}

fn required_header_is_missing(param_name: String) -> HttpFailResult {
    HttpFailResult {
        content_type: WebContentType::Text,
        content: format!( "Required header [{}] is missing", param_name)
        .into_bytes(),
        status_code: 400,
        write_telemetry: true,
        write_to_log: false,
        add_telemetry_tags: my_telemetry::TelemetryEventTagsBuilder::new(),
    }
}

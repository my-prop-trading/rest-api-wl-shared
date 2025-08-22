service_sdk::macros::use_my_http_server!();
use my_http_server::{HttpContext, HttpFailResult, WebContentType};

pub trait GetClientId {
    fn get_client_id(&self) -> Result<&str, HttpFailResult>;
}

impl GetClientId for HttpContext {
    fn get_client_id(&self) -> Result<&str, HttpFailResult> {
        if let Some(credentials) = self.credentials.as_ref() {
            Ok(credentials.get_id())
        } else {
            let output = HttpOutput::from_builder()
                .set_status_code(400)
                .set_content_type(WebContentType::Text)
                .set_content("Unauthenticated".as_bytes().to_vec())
                .build();

            Err(HttpFailResult::new(
                output,
                false,
                false,
            ))
        }
    }
}

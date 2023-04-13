use my_http_server::{HttpContext, HttpFailResult, WebContentType};

pub trait GetClientId {
    fn get_client_id(&self) -> Result<&str, HttpFailResult>;
}

impl GetClientId for HttpContext {
    fn get_client_id(&self) -> Result<&str, HttpFailResult> {
        if let Some(credentials) = self.credentials.as_ref() {
            Ok(credentials.get_id())
        } else {
            Err(HttpFailResult {
                content_type: WebContentType::Text,
                status_code: 401,
                content: "Unauthenticated".as_bytes().to_vec(),
                write_telemetry: false,
                write_to_log: false,
            })
        }
    }
}

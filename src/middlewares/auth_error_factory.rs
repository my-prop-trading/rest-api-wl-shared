service_sdk::macros::use_my_http_server!();

use my_http_server::controllers::{documentation::DataTypeProvider, AuthErrorFactory};
use my_http_server::WebContentType;
use serde::Serialize;
use service_sdk::my_http_server::macros::MyHttpObjectStructure;

use crate::{ApiHttpResult, ApiResultStatus};

pub struct AuthErrorFactoryWl;
#[derive(Serialize, MyHttpObjectStructure)]
pub struct AccessClaimRequired {
    pub result: ApiResultStatus,
    pub data: String,
}

impl AuthErrorFactory for AuthErrorFactoryWl {
    fn get_not_authenticated(&self) -> my_http_server::HttpFailResult {
        ApiResultStatus::AccessTokenInvalid.into()
    }

    fn get_not_authorized(&self, claim_name: String) -> my_http_server::HttpFailResult {
        let _content = AccessClaimRequired {
            result: ApiResultStatus::AccessClaimRequired,
            data: claim_name,
        };
        my_http_server::HttpFailResult::new(
            WebContentType::Text,
            401,
            "Unauthenticated".as_bytes().to_vec(),
            false,
            false,
        )
    }

    fn get_global_http_fail_result_types(
        &self,
    ) -> Option<Vec<my_http_server::controllers::documentation::out_results::HttpResult>> {
        use my_http_server::controllers::documentation::out_results::HttpResult;
        vec![
            HttpResult {
                http_code: 401,
                nullable: false,
                description: "Unauthenticated access".to_string(),
                data_type: ApiHttpResult::get_data_type(),
            },
            HttpResult {
                http_code: 403,
                nullable: false,
                description: "Unauthorized access".to_string(),
                data_type: AccessClaimRequired::get_data_type(),
            },
        ]
        .into()
    }
}

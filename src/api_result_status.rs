use my_http_server::HttpFailResult;
use my_http_server_controllers::controllers::documentation::DataTypeProvider;
use my_http_server_swagger::{MyHttpIntegerEnum, MyHttpObjectStructure};
use serde::Serialize;
use serde_repr::*;

#[derive(Serialize_repr, Deserialize_repr, MyHttpIntegerEnum, Debug, Clone, Copy)]
#[repr(i16)]
pub enum ApiResultStatus {
    #[http_enum_case(id="0"; description="Operations was successful")]
    Ok,

    #[http_enum_case(id="-1"; description="Invalid username or password")]
    InvalidUserNameOrPassword = -1,

    #[http_enum_case(id="-2"; description="User exists")]
    UserExists = -2,

    #[http_enum_case(id="-3"; description="User not found")]
    UserNotFound = -3,

    #[http_enum_case(id="-4"; description="Old password is wrong")]
    OldPasswordIsWrong = -4,

    #[http_enum_case(id="-5"; description="Wrong file extension")]
    WrongFileExtension = -5,

    #[http_enum_case(id="-6"; description="File not found")]
    FileNotFound = -6,

    #[http_enum_case(id="-7"; description="Personal data is not valid")]
    PersonalDataNotValid = -7,

    #[http_enum_case(id="-8"; description="System error")]
    SystemError = -8,

    #[http_enum_case(id="-9"; description="AccessTokenExpired")]
    AccessTokenExpired = -9,

    #[http_enum_case(id="-10"; description="TechnicalError")]
    TechnicalError = -10,

    #[http_enum_case(id="-11"; description="CountryRestriction")]
    CountryIsRestricted = -11,

    #[http_enum_case(id="-17"; description="AccessTokenInvalid")]
    AccessTokenInvalid = -17,

    #[http_enum_case(id="-18"; description="AccessClaimRequired")]
    AccessClaimRequired = -18,

    #[http_enum_case(id="-19"; description="TraderPackageNotFound")]
    TraderPackageNotFound = -19,

    #[http_enum_case(id="-20"; description="OrderNotFound")]
    OrderNotFound = -20,

    #[http_enum_case(id="-21"; description="OrderNotPaid")]
    OrderNotPaid = -21,

    #[http_enum_case(id="-30"; description="InvalidCodeEntered")]
    InvalidCodeEntered = -30,

    #[http_enum_case(id="-999"; description="Force Update required")]
    ForceUpdateIsRequired = -999,
}

impl ApiResultStatus {
    pub fn get_status_code(&self) -> u16 {
        match self {
            ApiResultStatus::Ok => 200,
            ApiResultStatus::InvalidUserNameOrPassword => 200,
            ApiResultStatus::UserExists => 200,
            ApiResultStatus::UserNotFound => 200,
            ApiResultStatus::OldPasswordIsWrong => 200,
            ApiResultStatus::WrongFileExtension => 200,
            ApiResultStatus::FileNotFound => 200,
            ApiResultStatus::PersonalDataNotValid => 200,
            ApiResultStatus::SystemError => 200,
            ApiResultStatus::AccessTokenExpired => 401,
            ApiResultStatus::TechnicalError => 200,
            ApiResultStatus::CountryIsRestricted => 200,
            ApiResultStatus::AccessTokenInvalid => 401,
            ApiResultStatus::AccessClaimRequired => 403,
            ApiResultStatus::ForceUpdateIsRequired => 200,
            ApiResultStatus::TraderPackageNotFound => 400,
            ApiResultStatus::OrderNotFound => 400,
            ApiResultStatus::OrderNotPaid => 400,
            ApiResultStatus::InvalidCodeEntered => 400,
        }
    }
}

#[derive(Serialize, MyHttpObjectStructure)]
pub struct ApiHttpResult {
    pub result: ApiResultStatus,
}

impl Into<HttpFailResult> for ApiHttpResult {
    fn into(self) -> HttpFailResult {
        self.result.into()
    }
}

impl Into<HttpFailResult> for ApiResultStatus {
    fn into(self) -> HttpFailResult {
        let status_code = self.get_status_code();
        let result = ApiHttpResult { result: self };

        HttpFailResult {
            content_type: my_http_server::WebContentType::Json,
            status_code,
            content: serde_json::to_vec(&result).unwrap(),
            write_telemetry: false,
            write_to_log: false,
        }
    }
}

#[derive(Serialize, MyHttpObjectStructure)]
pub struct ApiHttpResultWithData<TData: Serialize + DataTypeProvider> {
    pub result: ApiResultStatus,
    pub data: Option<TData>,
}

impl<TData: Serialize + DataTypeProvider> Into<HttpFailResult> for ApiHttpResultWithData<TData> {
    fn into(self) -> HttpFailResult {
        let status_code = self.result.get_status_code();

        HttpFailResult {
            content_type: my_http_server::WebContentType::Json,
            status_code,
            content: serde_json::to_vec(&self).unwrap(),
            write_telemetry: false,
            write_to_log: false,
        }
    }
}

#[cfg(test)]
mod test {
    use super::ApiResultStatus;
    use serde::Serialize;
    #[derive(Serialize, Debug)]
    pub struct TestStruct {
        result: ApiResultStatus,
    }

    #[test]
    pub fn test_result_deserialization() {
        let test_struct = TestStruct {
            result: ApiResultStatus::AccessTokenExpired,
        };

        let result = serde_json::to_string(&test_struct).unwrap();

        println!("{}", result);
    }
}

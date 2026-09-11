service_sdk::macros::use_my_http_server!();

use my_http_server::HttpFailResult;
use my_http_server::controllers::documentation::DataTypeProvider;
use serde::Serialize;
use serde_repr::*;
use service_sdk::my_http_server::macros::MyHttpObjectStructure;

// MyHttpIntegerEnum (my-http-server 0.9.0) generates its own serde impls that write the case
// name as a string; that would change `result` from an integer to text for every API consumer.
// serde_repr keeps the integer contract, and the impls below replace what the derive gave us.
#[derive(Serialize_repr, Deserialize_repr, Debug, Clone, Copy)]
#[repr(i16)]
pub enum ApiResultStatus {
    Ok,

    InvalidUserNameOrPassword = -1,

    UserExists = -2,

    UserNotFound = -3,

    OldPasswordIsWrong = -4,

    WrongFileExtension = -5,

    FileNotFound = -6,

    PersonalDataNotValid = -7,

    SystemError = -8,

    AccessTokenExpired = -9,

    TechnicalError = -10,

    CountryIsRestricted = -11,

    AccessTokenInvalid = -17,

    AccessClaimRequired = -18,

    TraderPackageNotFound = -19,

    OrderNotFound = -20,

    OrderNotPaid = -21,

    PasswordWasUsedBefore = -22,

    InvalidCodeEntered = -30,

    NotEnoughBalance = -40,

    NotAuthorized = -50,

    RefreshTokenExpired = -51,

    PayoutIsBlocked = -60,

    TraderIsNotVerified = -70,

    TraderIsAlreadyVerified = -71,

    InvalidCode = -72,

    InvalidDiscountCode = -73,

    DiscountCodeUsageExceeded = -74,

    DiscountCodeExpired = -75,

    DiscountCodeForAnotherPackage = -76,

    DiscountCodeOnlyForFirstPayin= -77,

    DiscountCodeOnlyForNextPayin = -78,

    InvalidSiteLanguage = -80,

    CompetitionNotFound = -90,

    CompetitionEnded = -91,

    CompetitionRegistrationClosed = -92,

    CompetitionFull = -93,

    CompetitionAlreadyEnrolled = -94,

    CompetitionPlatformNotAllowed = -95,

    UserHasOpenPositions = -100,

    RequestIsNoValid = -200,

    AmountExceedsMax = -201,

    AmountLessThanMin = -202,

    TradingPlatformIsNotValid = -300,

    RecaptchaVerificationIsFailed = -800,

    RecaptchaIsRequired = -801,

    /// An anti-abuse rule is holding this client's IP; `data` carries `retryAfterHours`.
    RegistrationIsRestricted = -802,

    BrandIsNotSetUp = -900,

    ForceUpdateIsRequired = -999,
}

impl my_http_utils::schema::data_types::DataTypeProvider for ApiResultStatus {
    fn get_data_type() -> my_http_utils::schema::data_types::HttpDataType {
        use my_http_utils::schema::data_types::{EnumType, HttpEnumCase, HttpEnumStructure};

        HttpEnumStructure {
            struct_id: "ApiResultStatus",
            enum_type: EnumType::Integer,
            cases: vec![
                HttpEnumCase {
                    id: Self::Ok as i16,
                    value: "Ok",
                    description: "Operations was successful",
                },
                HttpEnumCase {
                    id: Self::InvalidUserNameOrPassword as i16,
                    value: "InvalidUserNameOrPassword",
                    description: "Invalid username or password",
                },
                HttpEnumCase {
                    id: Self::UserExists as i16,
                    value: "UserExists",
                    description: "User exists",
                },
                HttpEnumCase {
                    id: Self::UserNotFound as i16,
                    value: "UserNotFound",
                    description: "User not found",
                },
                HttpEnumCase {
                    id: Self::OldPasswordIsWrong as i16,
                    value: "OldPasswordIsWrong",
                    description: "Old password is wrong",
                },
                HttpEnumCase {
                    id: Self::WrongFileExtension as i16,
                    value: "WrongFileExtension",
                    description: "Wrong file extension",
                },
                HttpEnumCase {
                    id: Self::FileNotFound as i16,
                    value: "FileNotFound",
                    description: "File not found",
                },
                HttpEnumCase {
                    id: Self::PersonalDataNotValid as i16,
                    value: "PersonalDataNotValid",
                    description: "Personal data is not valid",
                },
                HttpEnumCase {
                    id: Self::SystemError as i16,
                    value: "SystemError",
                    description: "System error",
                },
                HttpEnumCase {
                    id: Self::AccessTokenExpired as i16,
                    value: "AccessTokenExpired",
                    description: "AccessTokenExpired",
                },
                HttpEnumCase {
                    id: Self::TechnicalError as i16,
                    value: "TechnicalError",
                    description: "TechnicalError",
                },
                HttpEnumCase {
                    id: Self::CountryIsRestricted as i16,
                    value: "CountryIsRestricted",
                    description: "CountryRestriction",
                },
                HttpEnumCase {
                    id: Self::AccessTokenInvalid as i16,
                    value: "AccessTokenInvalid",
                    description: "AccessTokenInvalid",
                },
                HttpEnumCase {
                    id: Self::AccessClaimRequired as i16,
                    value: "AccessClaimRequired",
                    description: "AccessClaimRequired",
                },
                HttpEnumCase {
                    id: Self::TraderPackageNotFound as i16,
                    value: "TraderPackageNotFound",
                    description: "TraderPackageNotFound",
                },
                HttpEnumCase {
                    id: Self::OrderNotFound as i16,
                    value: "OrderNotFound",
                    description: "OrderNotFound",
                },
                HttpEnumCase {
                    id: Self::OrderNotPaid as i16,
                    value: "OrderNotPaid",
                    description: "OrderNotPaid",
                },
                HttpEnumCase {
                    id: Self::PasswordWasUsedBefore as i16,
                    value: "PasswordWasUsedBefore",
                    description: "Password was used before",
                },
                HttpEnumCase {
                    id: Self::InvalidCodeEntered as i16,
                    value: "InvalidCodeEntered",
                    description: "InvalidCodeEntered",
                },
                HttpEnumCase {
                    id: Self::NotEnoughBalance as i16,
                    value: "NotEnoughBalance",
                    description: "NotEnoughBalance",
                },
                HttpEnumCase {
                    id: Self::NotAuthorized as i16,
                    value: "NotAuthorized",
                    description: "NotAuthorized",
                },
                HttpEnumCase {
                    id: Self::RefreshTokenExpired as i16,
                    value: "RefreshTokenExpired",
                    description: "RefreshTokenExpired",
                },
                HttpEnumCase {
                    id: Self::PayoutIsBlocked as i16,
                    value: "PayoutIsBlocked",
                    description: "PayoutIsBlocked",
                },
                HttpEnumCase {
                    id: Self::TraderIsNotVerified as i16,
                    value: "TraderIsNotVerified",
                    description: "TraderIsNotVerified",
                },
                HttpEnumCase {
                    id: Self::TraderIsAlreadyVerified as i16,
                    value: "TraderIsAlreadyVerified",
                    description: "TraderIsAlreadyVerified",
                },
                HttpEnumCase {
                    id: Self::InvalidCode as i16,
                    value: "InvalidCode",
                    description: "InvalidCode",
                },
                HttpEnumCase {
                    id: Self::InvalidDiscountCode as i16,
                    value: "InvalidDiscountCode",
                    description: "InvalidDiscountCode",
                },
                HttpEnumCase {
                    id: Self::DiscountCodeUsageExceeded as i16,
                    value: "DiscountCodeUsageExceeded",
                    description: "DiscountCodeUsageExceeded",
                },
                HttpEnumCase {
                    id: Self::DiscountCodeExpired as i16,
                    value: "DiscountCodeExpired",
                    description: "DiscountCodeExpired",
                },
                HttpEnumCase {
                    id: Self::DiscountCodeForAnotherPackage as i16,
                    value: "DiscountCodeForAnotherPackage",
                    description: "DiscountCodeForAnotherPackage",
                },
                HttpEnumCase {
                    id: Self::DiscountCodeOnlyForFirstPayin as i16,
                    value: "DiscountCodeOnlyForFirstPayin",
                    description: "DiscountCodeOnlyForFirstPayin",
                },
                HttpEnumCase {
                    id: Self::DiscountCodeOnlyForNextPayin as i16,
                    value: "DiscountCodeOnlyForNextPayin",
                    description: "DiscountCodeOnlyForNextPayin",
                },
                HttpEnumCase {
                    id: Self::InvalidSiteLanguage as i16,
                    value: "InvalidSiteLanguage",
                    description: "InvalidSiteLanguage",
                },
                HttpEnumCase {
                    id: Self::CompetitionNotFound as i16,
                    value: "CompetitionNotFound",
                    description: "Competition not found",
                },
                HttpEnumCase {
                    id: Self::CompetitionEnded as i16,
                    value: "CompetitionEnded",
                    description: "Competition has ended",
                },
                HttpEnumCase {
                    id: Self::CompetitionRegistrationClosed as i16,
                    value: "CompetitionRegistrationClosed",
                    description: "Competition registration is closed",
                },
                HttpEnumCase {
                    id: Self::CompetitionFull as i16,
                    value: "CompetitionFull",
                    description: "Competition is full",
                },
                HttpEnumCase {
                    id: Self::CompetitionAlreadyEnrolled as i16,
                    value: "CompetitionAlreadyEnrolled",
                    description: "Already enrolled in competition",
                },
                HttpEnumCase {
                    id: Self::CompetitionPlatformNotAllowed as i16,
                    value: "CompetitionPlatformNotAllowed",
                    description: "Platform not allowed for this competition",
                },
                HttpEnumCase {
                    id: Self::UserHasOpenPositions as i16,
                    value: "UserHasOpenPositions",
                    description: "UserHasOpenPositions",
                },
                HttpEnumCase {
                    id: Self::RequestIsNoValid as i16,
                    value: "RequestIsNoValid",
                    description: "RequestIsNoValid",
                },
                HttpEnumCase {
                    id: Self::AmountExceedsMax as i16,
                    value: "AmountExceedsMax",
                    description: "AmountExceedsMax",
                },
                HttpEnumCase {
                    id: Self::AmountLessThanMin as i16,
                    value: "AmountLessThanMin",
                    description: "AmountLessThanMin",
                },
                HttpEnumCase {
                    id: Self::TradingPlatformIsNotValid as i16,
                    value: "TradingPlatformIsNotValid",
                    description: "TradingPlatformIsNotValid",
                },
                HttpEnumCase {
                    id: Self::RecaptchaVerificationIsFailed as i16,
                    value: "RecaptchaVerificationIsFailed",
                    description: "Google recaptcha failed: too many requests",
                },
                HttpEnumCase {
                    id: Self::RecaptchaIsRequired as i16,
                    value: "RecaptchaIsRequired",
                    description: "Google recaptcha is required",
                },
                HttpEnumCase {
                    id: Self::RegistrationIsRestricted as i16,
                    value: "RegistrationIsRestricted",
                    description: "Too many registrations from this IP",
                },
                HttpEnumCase {
                    id: Self::BrandIsNotSetUp as i16,
                    value: "BrandIsNotSetUp",
                    description: "BrandIsNotSetUp",
                },
                HttpEnumCase {
                    id: Self::ForceUpdateIsRequired as i16,
                    value: "ForceUpdateIsRequired",
                    description: "Force Update required",
                },
            ],
        }
        .into_http_data_type_object()
    }

    fn get_generic_type() -> Option<String> {
        None
    }
}

// Nested-in-object writing goes through my_json, not serde, so it must agree with Serialize_repr.
impl my_http_utils::my_json::json_writer::JsonValueWriter for ApiResultStatus {
    const IS_ARRAY: bool = false;

    fn write(&self, dest: &mut String) {
        my_http_utils::my_json::json_writer::JsonValueWriter::write(&(*self as i16), dest);
    }
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
            ApiResultStatus::NotEnoughBalance => 400,
            ApiResultStatus::NotAuthorized => 401,
            ApiResultStatus::UserHasOpenPositions => 400,
            ApiResultStatus::BrandIsNotSetUp => 500,
            ApiResultStatus::PasswordWasUsedBefore => 400,
            ApiResultStatus::RefreshTokenExpired => 400,
            ApiResultStatus::PayoutIsBlocked => 400,
            ApiResultStatus::TraderIsNotVerified => 403,
            ApiResultStatus::TraderIsAlreadyVerified => 400,
            ApiResultStatus::InvalidCode => 400,
            ApiResultStatus::RequestIsNoValid => 400,
            ApiResultStatus::AmountExceedsMax => 400,
            ApiResultStatus::AmountLessThanMin => 400,
            ApiResultStatus::InvalidDiscountCode => 200,
            ApiResultStatus::DiscountCodeUsageExceeded => 400,
            ApiResultStatus::DiscountCodeExpired => 400,
            ApiResultStatus::DiscountCodeForAnotherPackage => 200,
            ApiResultStatus::InvalidSiteLanguage => 400,
            ApiResultStatus::RecaptchaIsRequired => 200,
            ApiResultStatus::RecaptchaVerificationIsFailed => 200,
            ApiResultStatus::RegistrationIsRestricted => 200,
            ApiResultStatus::TradingPlatformIsNotValid => 400,
            ApiResultStatus::DiscountCodeOnlyForFirstPayin => 200,
            ApiResultStatus::DiscountCodeOnlyForNextPayin => 200,
            ApiResultStatus::CompetitionNotFound => 400,
            ApiResultStatus::CompetitionEnded => 400,
            ApiResultStatus::CompetitionRegistrationClosed => 400,
            ApiResultStatus::CompetitionFull => 400,
            ApiResultStatus::CompetitionAlreadyEnrolled => 400,
            ApiResultStatus::CompetitionPlatformNotAllowed => 400,
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
        let output = HttpOutput::from_builder()
            .set_status_code(status_code)
            .set_content_type(WebContentType::Json)
            .set_content(serde_json::to_vec(&result).unwrap(),
            )
            .build();

        HttpFailResult::new(output,
            false,
            false)
    }
}

#[derive(MyHttpObjectStructure)]
pub struct ApiHttpResultWithData<TData: Serialize + DataTypeProvider> {
    pub result: ApiResultStatus,
    pub data: Option<TData>,
}

// Hand-written to keep `skip_serializing_if = "Option::is_none"`: the derive now rejects serde
// attributes, but dropping the behaviour would add `"data": null` to every serde-rendered
// response (all HttpFailResult bodies).
impl<TData: Serialize + DataTypeProvider> Serialize for ApiHttpResultWithData<TData> {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;

        let fields = if self.data.is_some() { 2 } else { 1 };
        let mut state = serializer.serialize_struct("ApiHttpResultWithData", fields)?;
        state.serialize_field("result", &self.result)?;

        if let Some(data) = &self.data {
            state.serialize_field("data", data)?;
        }

        state.end()
    }
}

impl<TData: Serialize + DataTypeProvider> Into<HttpFailResult> for ApiHttpResultWithData<TData> {
    fn into(self) -> HttpFailResult {
        let status_code = self.result.get_status_code();
        let output = HttpOutput::from_builder()
            .set_status_code(status_code)
            .set_content_type(WebContentType::Json)
            .set_content(serde_json::to_vec(&self).unwrap(),
            )
            .build();
        
        HttpFailResult::new(output,
            false,
            false) 
    }
}

#[cfg(test)]
mod test {
    use super::{ApiHttpResultWithData, ApiResultStatus};
    use serde::Serialize;

    #[derive(Serialize, Debug)]
    pub struct TestStruct {
        result: ApiResultStatus,
    }

    // These assert the wire contract every rest-api consumer depends on. They exist because
    // my-http-server 0.9.0 can silently turn the integer into a case-name string.
    #[test]
    pub fn status_serializes_as_integer() {
        let test_struct = TestStruct {
            result: ApiResultStatus::AccessTokenExpired,
        };

        assert_eq!(
            serde_json::to_string(&test_struct).unwrap(),
            r#"{"result":-9}"#
        );
    }

    #[test]
    pub fn status_writes_as_integer_through_my_json() {
        use my_http_utils::my_json::json_writer::JsonValueWriter;

        let mut dest = String::new();
        ApiResultStatus::AccessTokenExpired.write(&mut dest);

        assert_eq!(dest, "-9");
    }

    #[test]
    pub fn data_is_omitted_when_none() {
        let result: ApiHttpResultWithData<String> = ApiHttpResultWithData {
            result: ApiResultStatus::Ok,
            data: None,
        };

        assert_eq!(serde_json::to_string(&result).unwrap(), r#"{"result":0}"#);
    }

    #[test]
    pub fn data_is_written_when_some() {
        let result = ApiHttpResultWithData {
            result: ApiResultStatus::Ok,
            data: Some("payload".to_string()),
        };

        assert_eq!(
            serde_json::to_string(&result).unwrap(),
            r#"{"result":0,"data":"payload"}"#
        );
    }
}

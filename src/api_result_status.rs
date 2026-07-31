service_sdk::macros::use_my_http_server!();

use my_http_server::HttpFailResult;
use my_http_server::controllers::documentation::DataTypeProvider;
use serde::Serialize;
use serde_repr::*;
use service_sdk::my_http_server::macros::MyHttpObjectStructure;

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

    BrandIsNotSetUp = -900,

    ForceUpdateIsRequired = -999,
}

// MyHttpIntegerEnum (derive) now generates its own Serialize/Deserialize that writes the case as
// a *string* (`self.as_str()`), conflicting with the manual Serialize_repr/Deserialize_repr above
// which keep this on the wire as a number — the derive was dropped so the numeric contract
// doesn't silently change. DataTypeProvider/JsonValueWriter are what the derive would otherwise
// have generated, written by hand to preserve the same numeric `id`/`value` (the enum's Rust
// discriminant, not the `http_enum_case(id=...)` attribute — those two disagree for
// RecaptchaIsRequired, and the discriminant is what actually goes on the wire).
use my_http_utils::schema::data_types::{HttpDataType, HttpEnumCase, HttpEnumStructure};

impl DataTypeProvider for ApiResultStatus {
    fn get_data_type() -> HttpDataType {
        let mut __es = HttpEnumStructure {
            struct_id: "ApiResultStatus",
            enum_type: my_http_utils::schema::data_types::EnumType::Integer,
            cases: vec![],
        };

        __es.cases.push(HttpEnumCase { id: Self::Ok as i16, value: "0", description: "Operations was successful" });
        __es.cases.push(HttpEnumCase { id: Self::InvalidUserNameOrPassword as i16, value: "-1", description: "Invalid username or password" });
        __es.cases.push(HttpEnumCase { id: Self::UserExists as i16, value: "-2", description: "User exists" });
        __es.cases.push(HttpEnumCase { id: Self::UserNotFound as i16, value: "-3", description: "User not found" });
        __es.cases.push(HttpEnumCase { id: Self::OldPasswordIsWrong as i16, value: "-4", description: "Old password is wrong" });
        __es.cases.push(HttpEnumCase { id: Self::WrongFileExtension as i16, value: "-5", description: "Wrong file extension" });
        __es.cases.push(HttpEnumCase { id: Self::FileNotFound as i16, value: "-6", description: "File not found" });
        __es.cases.push(HttpEnumCase { id: Self::PersonalDataNotValid as i16, value: "-7", description: "Personal data is not valid" });
        __es.cases.push(HttpEnumCase { id: Self::SystemError as i16, value: "-8", description: "System error" });
        __es.cases.push(HttpEnumCase { id: Self::AccessTokenExpired as i16, value: "-9", description: "AccessTokenExpired" });
        __es.cases.push(HttpEnumCase { id: Self::TechnicalError as i16, value: "-10", description: "TechnicalError" });
        __es.cases.push(HttpEnumCase { id: Self::CountryIsRestricted as i16, value: "-11", description: "CountryRestriction" });
        __es.cases.push(HttpEnumCase { id: Self::AccessTokenInvalid as i16, value: "-17", description: "AccessTokenInvalid" });
        __es.cases.push(HttpEnumCase { id: Self::AccessClaimRequired as i16, value: "-18", description: "AccessClaimRequired" });
        __es.cases.push(HttpEnumCase { id: Self::TraderPackageNotFound as i16, value: "-19", description: "TraderPackageNotFound" });
        __es.cases.push(HttpEnumCase { id: Self::OrderNotFound as i16, value: "-20", description: "OrderNotFound" });
        __es.cases.push(HttpEnumCase { id: Self::OrderNotPaid as i16, value: "-21", description: "OrderNotPaid" });
        __es.cases.push(HttpEnumCase { id: Self::PasswordWasUsedBefore as i16, value: "-22", description: "Password was used before" });
        __es.cases.push(HttpEnumCase { id: Self::InvalidCodeEntered as i16, value: "-30", description: "InvalidCodeEntered" });
        __es.cases.push(HttpEnumCase { id: Self::NotEnoughBalance as i16, value: "-40", description: "NotEnoughBalance" });
        __es.cases.push(HttpEnumCase { id: Self::NotAuthorized as i16, value: "-50", description: "NotAuthorized" });
        __es.cases.push(HttpEnumCase { id: Self::RefreshTokenExpired as i16, value: "-51", description: "RefreshTokenExpired" });
        __es.cases.push(HttpEnumCase { id: Self::PayoutIsBlocked as i16, value: "-60", description: "PayoutIsBlocked" });
        __es.cases.push(HttpEnumCase { id: Self::TraderIsNotVerified as i16, value: "-70", description: "TraderIsNotVerified" });
        __es.cases.push(HttpEnumCase { id: Self::TraderIsAlreadyVerified as i16, value: "-71", description: "TraderIsAlreadyVerified" });
        __es.cases.push(HttpEnumCase { id: Self::InvalidCode as i16, value: "-72", description: "InvalidCode" });
        __es.cases.push(HttpEnumCase { id: Self::InvalidDiscountCode as i16, value: "-73", description: "InvalidDiscountCode" });
        __es.cases.push(HttpEnumCase { id: Self::DiscountCodeUsageExceeded as i16, value: "-74", description: "DiscountCodeUsageExceeded" });
        __es.cases.push(HttpEnumCase { id: Self::DiscountCodeExpired as i16, value: "-75", description: "DiscountCodeExpired" });
        __es.cases.push(HttpEnumCase { id: Self::DiscountCodeForAnotherPackage as i16, value: "-76", description: "DiscountCodeForAnotherPackage" });
        __es.cases.push(HttpEnumCase { id: Self::DiscountCodeOnlyForFirstPayin as i16, value: "-77", description: "DiscountCodeOnlyForFirstPayin" });
        __es.cases.push(HttpEnumCase { id: Self::DiscountCodeOnlyForNextPayin as i16, value: "-78", description: "DiscountCodeOnlyForNextPayin" });
        __es.cases.push(HttpEnumCase { id: Self::InvalidSiteLanguage as i16, value: "-80", description: "InvalidSiteLanguage" });
        __es.cases.push(HttpEnumCase { id: Self::CompetitionNotFound as i16, value: "-90", description: "Competition not found" });
        __es.cases.push(HttpEnumCase { id: Self::CompetitionEnded as i16, value: "-91", description: "Competition has ended" });
        __es.cases.push(HttpEnumCase { id: Self::CompetitionRegistrationClosed as i16, value: "-92", description: "Competition registration is closed" });
        __es.cases.push(HttpEnumCase { id: Self::CompetitionFull as i16, value: "-93", description: "Competition is full" });
        __es.cases.push(HttpEnumCase { id: Self::CompetitionAlreadyEnrolled as i16, value: "-94", description: "Already enrolled in competition" });
        __es.cases.push(HttpEnumCase { id: Self::CompetitionPlatformNotAllowed as i16, value: "-95", description: "Platform not allowed for this competition" });
        __es.cases.push(HttpEnumCase { id: Self::UserHasOpenPositions as i16, value: "-100", description: "UserHasOpenPositions" });
        __es.cases.push(HttpEnumCase { id: Self::RequestIsNoValid as i16, value: "-200", description: "RequestIsNoValid" });
        __es.cases.push(HttpEnumCase { id: Self::AmountExceedsMax as i16, value: "-201", description: "AmountExceedsMax" });
        __es.cases.push(HttpEnumCase { id: Self::AmountLessThanMin as i16, value: "-202", description: "AmountLessThanMin" });
        __es.cases.push(HttpEnumCase { id: Self::TradingPlatformIsNotValid as i16, value: "-300", description: "TradingPlatformIsNotValid" });
        __es.cases.push(HttpEnumCase { id: Self::RecaptchaVerificationIsFailed as i16, value: "-800", description: "Google recaptcha failed: too many requests" });
        __es.cases.push(HttpEnumCase { id: Self::RecaptchaIsRequired as i16, value: "-801", description: "Google recaptcha is required" });
        __es.cases.push(HttpEnumCase { id: Self::BrandIsNotSetUp as i16, value: "-900", description: "BrandIsNotSetUp" });
        __es.cases.push(HttpEnumCase { id: Self::ForceUpdateIsRequired as i16, value: "-999", description: "Force Update required" });

        __es.into_http_data_type_object()
    }

    fn get_generic_type() -> Option<String> {
        None
    }
}

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

// `MyHttpObjectStructure` (0.9.0+) rejects `#[serde(skip_serializing_if)]` on any field, since
// its own my-json read/write path never consults serde. `data`'s "omit the key entirely when
// None" behavior is part of the wire contract several services already depend on, so `Serialize`
// is implemented by hand here (kept out of the derive list) rather than accepting `"data":null`.
#[derive(MyHttpObjectStructure)]
pub struct ApiHttpResultWithData<TData: Serialize + DataTypeProvider> {
    pub result: ApiResultStatus,
    pub data: Option<TData>,
}

impl<TData: Serialize + DataTypeProvider> Serialize for ApiHttpResultWithData<TData> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let field_count = if self.data.is_some() { 2 } else { 1 };
        let mut state = serializer.serialize_struct("ApiHttpResultWithData", field_count)?;
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

        assert_eq!(result, r#"{"result":-9}"#);
    }

    #[test]
    pub fn test_result_with_data_omits_data_key_when_none() {
        use super::{ApiHttpResultWithData, ApiResultStatus};

        let with_data = ApiHttpResultWithData::<String> {
            result: ApiResultStatus::Ok,
            data: Some("hello".to_owned()),
        };
        assert_eq!(
            serde_json::to_string(&with_data).unwrap(),
            r#"{"result":0,"data":"hello"}"#
        );

        let without_data = ApiHttpResultWithData::<String> {
            result: ApiResultStatus::UserNotFound,
            data: None,
        };
        assert_eq!(
            serde_json::to_string(&without_data).unwrap(),
            r#"{"result":-3}"#
        );
    }
}

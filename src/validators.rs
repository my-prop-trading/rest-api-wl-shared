use std::str::FromStr;

use phonenumber::PhoneNumber;
use service_sdk::{
    my_http_server::{HttpContext, HttpFailResult},
    rust_extensions::date_time::DateTimeAsMicroseconds,
};

use crate::{ApiHttpResultWithData, ApiResultStatus};

pub fn validate_non_empty(_ctx: &HttpContext, value: &str) -> Result<(), HttpFailResult> {
    if validate_non_empty_text(value) {
        return Ok(());
    }

    Err(create_fail_http_result("Should not be empty"))
}

pub fn validate_email(_ctx: &HttpContext, value: &str) -> Result<(), HttpFailResult> {
    if !validate_email_text(value) {
        return Err(create_fail_http_result("Invalid Email format"));
    }

    if !validate_max(value, 64) {
        return Err(create_fail_http_result("Max length is 64 symbols"));
    }

    if !validate_no_trimm_spaces(value) {
        return Err(create_fail_http_result("Should not start or end with space"));
    }

    if !validate_no_cyrillic(value) {
        return Err(create_fail_http_result("No cyrillic letters are allowed"));
    }

    return Ok(());
}

pub fn validate_email_optional(
    _ctx: &HttpContext,
    value: &Option<String>,
) -> Result<(), HttpFailResult> {
    match value {
        Some(value) => {
            return validate_email(_ctx, value);
        }
        None => Ok(()),
    }
}

pub fn validate_password(_ctx: &HttpContext, value: &str) -> Result<(), HttpFailResult> {
    match validate_password_conditions(value) {
        Ok(_) => Ok(()),
        Err(err_msg) => Err(HttpFailResult::as_validation_error(err_msg)),
    }
}

pub fn validate_password_conditions(value: &str) -> Result<(), String> {

    let checks: &[(fn(&str) -> bool, &str)] = &[
        (|v| validate_min(v, 8), "Min length is 8 symbols"),
        (|v| validate_max(v, 50), "Max length is 50 symbols"),
        (validate_no_trimm_spaces, "Should not start or end with space"),
        (validate_no_cyrillic, "No cyrillic letters are allowed"),
        (contains_upper_letter, "Must contain upper letter"),
        (contains_no_space_characters, "Password must contain no space characters"),
        (contains_special_symbol, "Password must contain at least one special symbol"),
    ];

    for (check, message) in checks {
        if !check(value) {
            return Err(message.to_string());
        }
    }

    Ok(())
}

pub fn validate_phone(_ctx: &HttpContext, value: &str) -> Result<(), HttpFailResult> {
    if !validate_non_empty_text(value) {
        return Err(create_fail_http_result("Phone: Should not be empty"));
    }

    if !validate_no_trimm_spaces(value) {
        return Err(create_fail_http_result("Phone: Should not start or end with space"));
    }

    if validate_phone_text(value)
    {
        return Ok(());
    }

    Err(create_fail_http_result("Phone is not valid!"))
}

pub fn validate_phone_optional(
    _ctx: &HttpContext,
    value: &Option<String>,
) -> Result<(), HttpFailResult> {
    match value {
        Some(value) => {
            return validate_phone(_ctx, value);
        }
        None => Ok(()),
    }
}

fn validate_phone_text(value: &str) -> bool {
    let number = PhoneNumber::from_str(value);

    match number {
        Ok(number) => {
            return number.is_valid();
        }
        Err(_) => {
            return false;
        }
        
    }
}

pub fn validate_name(_ctx: &HttpContext, value: &str) -> Result<(), HttpFailResult> {
    if !validate_max(value, 32) {
        return Err(create_fail_http_result("Name: Max length is 32 symbols"));
    }

    if !validate_no_trimm_spaces(value) {
        return Err(create_fail_http_result("Should not start or end with space"));
    }
    
    if !validate_latin_letters_with_spaces(value) {
        return Err(create_fail_http_result("Name: Only latin letters are allowed"));
    }

    return Ok(());
}

pub fn validate_name_optional(
    _ctx: &HttpContext,
    value: &Option<String>,
) -> Result<(), HttpFailResult> {
    match value {
        Some(value) => {
            return validate_name(_ctx, value);
        }
        None => Ok(()),
    }
}

pub fn validate_name_with_spaces(_ctx: &HttpContext, value: &str) -> Result<(), HttpFailResult> {
    if !validate_latin_letters_with_spaces(value) {
        return Err(create_fail_http_result("Name: Only latin letters and spaces are allowed"));
    }

    if !validate_max(value, 32) {
        return Err(create_fail_http_result("Name: Max length is 32 symbols"));
    }

    return Ok(());
}

pub fn validate_name_with_spaces_optional(
    _ctx: &HttpContext,
    value: &Option<String>,
) -> Result<(), HttpFailResult> {
    match value {
        Some(value) => {
            return validate_name_with_spaces(_ctx, value);
        }
        None => Ok(()),
    }
}

pub fn validate_date_of_birth(
    _ctx: &HttpContext,
    value: &str,
) -> Result<(), HttpFailResult> {
    let value = match DateTimeAsMicroseconds::from_str(value) {
        Some(x) => x,
        None => return Err(create_fail_http_result("DateOfBirth: Not a valid date!")),
    };

    let now = DateTimeAsMicroseconds::now();
    let diff = now.duration_since(value);

    match diff {
        service_sdk::rust_extensions::date_time::DateTimeDuration::Positive(x) => {
            // turn secunds to years
            let x = x.as_secs() / 60 / 60 / 24 / 365;
            if x < 18 {
                return Err(create_fail_http_result("DateOfBirth: Should be older than 18"));
            }

            return Ok(());
        }
        service_sdk::rust_extensions::date_time::DateTimeDuration::Negative(_) => {}
        service_sdk::rust_extensions::date_time::DateTimeDuration::Zero => {}
    }

    return Err(create_fail_http_result("DateOfBirth: Should be older than 18"));
}

pub fn validate_date_of_birth_optional(
    _ctx: &HttpContext,
    value: &Option<String>,
) -> Result<(), HttpFailResult> {
    match value {
        Some(value) => {
            return validate_date_of_birth(_ctx, value);
        }
        None => Ok(()),
    }
}

pub fn validate_address(
    _ctx: &HttpContext,
    value: &str,
) -> Result<(), HttpFailResult> {
    if !validate_max(value, 50) {
        return Err(create_fail_http_result("Address: Max length is 50 symbols"));
    }

    if !validate_non_empty_text(value) {
        return Err(create_fail_http_result("Address: Should not be empty"));
    }

    if !validate_no_trimm_spaces(value) {
        return Err(create_fail_http_result("Address: Should not start or end with space"));
    }

    if !validate_no_cyrillic(value) {
        return Err(create_fail_http_result("Address: No cyrillic letters are allowed"));
    }

    return Ok(());
}

pub fn validate_address_optional(
    ctx: &HttpContext,
    value: &Option<String>,
) -> Result<(), HttpFailResult> {
    let Some(value) = value else {
        return Ok(());
    };

    return validate_address(ctx, value);
}

pub fn validate_city(
    _ctx: &HttpContext,
    value: &str,
) -> Result<(), HttpFailResult> {
    if !validate_max(value, 50) {
        return Err(create_fail_http_result("City: Max length is 50 symbols"));
    }

    if !validate_non_empty_text(value) {
        return Err(create_fail_http_result("City: Should not be empty"));
    }

    if !validate_no_trimm_spaces(value) {
        return Err(create_fail_http_result("City: Should not start or end with space"));
    }

    if !validate_no_cyrillic(value) {
        return Err(create_fail_http_result("City: No cyrillic letters are allowed"));
    }

    return Ok(());
}

pub fn validate_city_optional(
    ctx: &HttpContext,
    value: &Option<String>,
) -> Result<(), HttpFailResult> {
    let Some(value) = value else {
        return Ok(());
    };

    return validate_city(ctx, value);
}

pub fn validate_zip_code(
    _ctx: &HttpContext,
    value: &str,
) -> Result<(), HttpFailResult> {
    if !validate_max(value, 10) {
        return Err(create_fail_http_result("ZipCode: Max length is 10 symbols"));
    }

    if !validate_non_empty_text(value) {
        return Err(create_fail_http_result("ZipCode: Should not be empty"));
    }

    if !validate_no_trimm_spaces(value) {
        return Err(create_fail_http_result("ZipCode: Should not start or end with space"));
    }

    if !validate_no_cyrillic(value) {
        return Err(create_fail_http_result("ZipCode: No cyrillic letters are allowed"));
    }

    return Ok(());
}

pub fn validate_zip_code_optional(
    ctx: &HttpContext,
    value: &Option<String>,
) -> Result<(), HttpFailResult> {
    let Some(value) = value else {
        return Ok(());
    };

    return validate_zip_code(ctx, value);
}

pub fn validate_latin_letters_only(src: &str) -> bool {
    regex::Regex::new(r"^[a-zA-Z\-]*$").unwrap().is_match(src)
}

pub fn validate_latin_letters_with_spaces(src: &str) -> bool {
    regex::Regex::new(r"^[a-zA-Z\-]+(\s+[a-zA-Z\-]+)*$").unwrap().is_match(src)
}

pub fn validate_no_cyrillic(src: &str) -> bool {
    src.chars().all(|c| !is_cyrillic(c))
}

pub fn contains_upper_letter(src: &str) -> bool {
    src.chars().any(|c| c.is_uppercase())
}

fn is_cyrillic(c: char) -> bool {
    ('\u{0400}'..='\u{04FF}').contains(&c) || 
    ('\u{0500}'..='\u{052F}').contains(&c) ||
    ('\u{2DE0}'..='\u{2DFF}').contains(&c) ||
    ('\u{A640}'..='\u{A69F}').contains(&c)
}

pub fn validate_no_trimm_spaces(src: &str) -> bool {
    !src.starts_with(" ") && !src.ends_with(" ")
}

pub fn validate_non_empty_text(src: &str) -> bool {
    !src.is_empty()
}

pub fn validate_max(src: &str, max: usize) -> bool {
    src.len() <= max
}

pub fn validate_min(src: &str, min: usize) -> bool {
    src.len() >= min
}

fn validate_email_text(src: &str) -> bool {
    regex::Regex::new(r"^[^\s@]+@[^\s@]+\.[^\s@]+$")
        .unwrap()
        .is_match(src)
}

const SPECIAL_SYMBOLS: [char; 13] = [
    '!', '@', '#', '$', '%', '^', '&', '*', '(', ')', '-', '+', '=',
];


pub fn contains_no_space_characters(value: &str) -> bool {
    !value.as_bytes().iter().any(|b| *b <= 32)
}

pub fn contains_special_symbol(value: &str) -> bool {
    value
        .as_bytes()
        .iter()
        .any(|b| SPECIAL_SYMBOLS.iter().any(|c| *c as u8 == *b))
}

pub fn create_fail_http_result(error: &str) -> HttpFailResult {
    HttpFailResult::new(
        service_sdk::my_http_server::WebContentType::Json,
        400,
        serde_json::to_vec(&ApiHttpResultWithData::<String> {
            result: ApiResultStatus::RequestIsNoValid,
            data: Some(error.to_string()),
        },).unwrap(),
        true,
        true,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate_email_is_correct() {
        assert!(validate_email_text("test@test.tt"));

        assert_eq!(false, validate_email_text("@test.tt"));

        assert_eq!(false, validate_email_text("test.tt@"));

        assert_eq!(false, validate_email_text(" test.tt@sss.tr"));
    }

    #[test]
    fn validate_phone_is_correct() {
        assert_eq!(true, validate_phone_text("+1-202-555-0173"));

        assert_eq!(false, validate_phone_text("+359111"));

        assert_eq!(false, validate_phone_text("1"));
        assert_eq!(true, validate_phone_text("+55-99-7115-675-1"));
        assert_eq!(false, validate_phone_text("+55-99-7115-675"));
    }

    #[test]
    fn validate_name_with_spaces_correct() {
        assert!(validate_latin_letters_with_spaces("Jhon Do  Doo"));
    }

    #[test]
    fn validate_name_with_spaces_failed() {
        assert!(!validate_latin_letters_with_spaces("Jhon Doo  "));
    }

    #[test]
    fn valid_password_passes_all_checks() {
        let password = "Valid123!";
        let result = validate_password_conditions(password);
        assert!(result.is_ok(), "Expected OK, got: {:?}", result);
    }

    #[test]
    fn fails_on_too_short_password() {
        let password = "V1!";
        let result = validate_password_conditions(password);
        assert_eq!(result, Err("Min length is 8 symbols".to_string()));
    }

    #[test]
    fn fails_on_too_long_password() {
        let password = "A".repeat(51) + "!";
        let result = validate_password_conditions(&password);
        assert_eq!(result, Err("Max length is 50 symbols".to_string()));
    }

    #[test]
    fn fails_on_leading_or_trailing_space() {
        let password = " Valid123!";
        let result = validate_password_conditions(password);
        assert_eq!(
            result,
            Err("Should not start or end with space".to_string())
        );

        let password = "Valid123! ";
        let result = validate_password_conditions(password);
        assert_eq!(
            result,
            Err("Should not start or end with space".to_string())
        );
    }

    #[test]
    fn fails_on_internal_space_character() {
        let password = "Valid 123!";
        let result = validate_password_conditions(password);
        assert_eq!(
            result,
            Err("Password must contain no space characters".to_string())
        );
    }

    #[test]
    fn fails_on_cyrillic_letters() {
        let password = "Вalid123!";
        let result = validate_password_conditions(password);
        assert_eq!(
            result,
            Err("No cyrillic letters are allowed".to_string())
        );
    }

    #[test]
    fn fails_without_uppercase_letter() {
        let password = "valid123!";
        let result = validate_password_conditions(password);
        assert_eq!(
            result,
            Err("Must contain upper letter".to_string())
        );
    }

    #[test]
    fn fails_without_special_symbol() {
        let password = "Valid123";
        let result = validate_password_conditions(password);
        assert_eq!(
            result,
            Err("Password must contain at least one special symbol".to_string())
        );
    }
}

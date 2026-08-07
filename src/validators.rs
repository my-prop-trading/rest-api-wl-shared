use crate::{ApiHttpResultWithData, ApiResultStatus};
use phonenumber::PhoneNumber;
use service_sdk::my_http_server::{HttpFailResult, HttpOutput, WebContentType};
use service_sdk::rust_extensions::date_time::DateTimeAsMicroseconds;
use std::str::FromStr;

// my-http-server 0.9.0 derives call a validator as `f(value) -> Result<(), impl Display>` and wrap
// the error into a 400 themselves. Optional fields are unwrapped by the derive, so `*_optional`
// wrappers are gone — point the attribute at the plain validator.

pub fn validate_non_empty(value: &str) -> Result<(), String> {
    if validate_non_empty_text(value) {
        return Ok(());
    }

    Err("Should not be empty".to_string())
}

pub fn validate_email(value: &str) -> Result<(), String> {
    if !validate_email_text(value) {
        return Err("Invalid Email format".to_string());
    }

    if !validate_max(value, 64) {
        return Err("Max length is 64 symbols".to_string());
    }

    if !validate_no_trimm_spaces(value) {
        return Err("Should not start or end with space".to_string());
    }

    if !validate_no_cyrillic(value) {
        return Err("No cyrillic letters are allowed".to_string());
    }

    Ok(())
}

pub fn validate_password(value: &str) -> Result<(), String> {
    validate_password_conditions(value)
}

pub fn validate_password_conditions(value: &str) -> Result<(), String> {
    let checks: &[(fn(&str) -> bool, &str)] = &[
        (|v| validate_min(v, 8), "Min length is 8 symbols"),
        (|v| validate_max(v, 50), "Max length is 50 symbols"),
        (
            validate_no_trimm_spaces,
            "Should not start or end with space",
        ),
        (validate_no_cyrillic, "No cyrillic letters are allowed"),
        (contains_upper_letter, "Must contain upper letter"),
        (
            contains_no_space_characters,
            "Password must contain no space characters",
        ),
        (
            contains_special_symbol,
            "Password must contain at least one special symbol",
        ),
    ];

    for (check, message) in checks {
        if !check(value) {
            return Err(message.to_string());
        }
    }

    Ok(())
}

pub fn validate_phone(value: &str) -> Result<(), String> {
    if !validate_non_empty_text(value) {
        return Err("Phone: Should not be empty".to_string());
    }

    if !validate_no_trimm_spaces(value) {
        return Err("Phone: Should not start or end with space".to_string());
    }

    if validate_phone_text(value) {
        return Ok(());
    }

    Err("Phone is not valid!".to_string())
}

fn validate_phone_text(value: &str) -> bool {
    let number = PhoneNumber::from_str(value);

    match number {
        Ok(number) => number.is_valid(),
        Err(_) => false,
    }
}

pub fn validate_name(value: &str) -> Result<(), String> {
    if !validate_max(value, 32) {
        return Err("Name: Max length is 32 symbols".to_string());
    }

    if !validate_no_trimm_spaces(value) {
        return Err("Should not start or end with space".to_string());
    }

    if !validate_latin_letters_with_spaces(value) {
        return Err("Name: Only latin letters are allowed".to_string());
    }

    Ok(())
}

pub fn validate_name_with_spaces(value: &str) -> Result<(), String> {
    if !validate_latin_letters_with_spaces(value) {
        return Err("Name: Only latin letters and spaces are allowed".to_string());
    }

    if !validate_max(value, 32) {
        return Err("Name: Max length is 32 symbols".to_string());
    }

    Ok(())
}

pub fn validate_date_of_birth(value: &str) -> Result<(), String> {
    const ADULT_AGE_YEARS: u64 = 18;
    const SECS_IN_YEAR: u64 = 60 * 60 * 24 * 365;

    let value = match DateTimeAsMicroseconds::from_str(value) {
        Some(x) => x,
        None => return Err("DateOfBirth: Not a valid date!".to_string()),
    };

    let now = DateTimeAsMicroseconds::now();

    if let service_sdk::rust_extensions::date_time::DateTimeDuration::Positive(duration) =
        now.duration_since(value)
    {
        if duration.as_secs() / SECS_IN_YEAR >= ADULT_AGE_YEARS {
            return Ok(());
        }
    }

    Err("DateOfBirth: Should be older than 18".to_string())
}

pub fn validate_address(value: &str) -> Result<(), String> {
    if !validate_max(value, 50) {
        return Err("Address: Max length is 50 symbols".to_string());
    }

    if !validate_non_empty_text(value) {
        return Err("Address: Should not be empty".to_string());
    }

    if !validate_no_trimm_spaces(value) {
        return Err("Address: Should not start or end with space".to_string());
    }

    if !validate_no_cyrillic(value) {
        return Err("Address: No cyrillic letters are allowed".to_string());
    }

    Ok(())
}

pub fn validate_city(value: &str) -> Result<(), String> {
    if !validate_max(value, 50) {
        return Err("City: Max length is 50 symbols".to_string());
    }

    if !validate_non_empty_text(value) {
        return Err("City: Should not be empty".to_string());
    }

    if !validate_no_trimm_spaces(value) {
        return Err("City: Should not start or end with space".to_string());
    }

    if !validate_no_cyrillic(value) {
        return Err("City: No cyrillic letters are allowed".to_string());
    }

    Ok(())
}

pub fn validate_zip_code(value: &str) -> Result<(), String> {
    if !validate_max(value, 10) {
        return Err("ZipCode: Max length is 10 symbols".to_string());
    }

    if !validate_non_empty_text(value) {
        return Err("ZipCode: Should not be empty".to_string());
    }

    if !validate_no_trimm_spaces(value) {
        return Err("ZipCode: Should not start or end with space".to_string());
    }

    if !validate_no_cyrillic(value) {
        return Err("ZipCode: No cyrillic letters are allowed".to_string());
    }

    Ok(())
}

pub fn validate_latin_letters_only(src: &str) -> bool {
    regex::Regex::new(r"^[a-zA-Z\-]*$").unwrap().is_match(src)
}

pub fn validate_latin_letters_with_spaces(src: &str) -> bool {
    regex::Regex::new(r"^[a-zA-Z\-]+(\s+[a-zA-Z\-]+)*$")
        .unwrap()
        .is_match(src)
}

pub fn validate_no_cyrillic(src: &str) -> bool {
    src.chars().all(|c| !is_cyrillic(c))
}

pub fn contains_upper_letter(src: &str) -> bool {
    src.chars().any(|c| c.is_uppercase())
}

fn is_cyrillic(c: char) -> bool {
    ('\u{0400}'..='\u{04FF}').contains(&c)
        || ('\u{0500}'..='\u{052F}').contains(&c)
        || ('\u{2DE0}'..='\u{2DFF}').contains(&c)
        || ('\u{A640}'..='\u{A69F}').contains(&c)
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

/// Builds the legacy `{result, data}` 400 payload for handlers that validate inline.
pub fn create_fail_http_result(error: &str) -> HttpFailResult {
    let output = HttpOutput::from_builder()
        .set_status_code(400)
        .set_content_type(WebContentType::Json)
        .set_content(
            serde_json::to_vec(&ApiHttpResultWithData::<String> {
                result: ApiResultStatus::RequestIsNoValid,
                data: Some(error.to_string()),
            })
            .unwrap(),
        )
        .build();

    HttpFailResult::new(output, true, true)
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
        assert_eq!(result, Err("No cyrillic letters are allowed".to_string()));
    }

    #[test]
    fn fails_without_uppercase_letter() {
        let password = "valid123!";
        let result = validate_password_conditions(password);
        assert_eq!(result, Err("Must contain upper letter".to_string()));
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

    #[test]
    fn date_of_birth_rejects_underage() {
        let mut recent = DateTimeAsMicroseconds::now();
        recent.add_days(-365);

        assert!(validate_date_of_birth(&recent.to_rfc3339()).is_err());
    }
}

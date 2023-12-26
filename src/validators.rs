use service_sdk::{
    my_http_server::{HttpContext, HttpFailResult},
    rust_extensions::date_time::DateTimeAsMicroseconds,
};

pub fn validate_non_empty(_ctx: &HttpContext, value: &str) -> Result<(), HttpFailResult> {
    if validate_non_empty_text(value) {
        return Ok(());
    }

    Err(HttpFailResult::as_validation_error(
        "Should not be empty".to_string(),
    ))
}

pub fn validate_email(_ctx: &HttpContext, value: &str) -> Result<(), HttpFailResult> {
    if !validate_email_text(value) {
        return Err(HttpFailResult::as_validation_error(
            "Invalid Email format".to_string(),
        ))
    }

    if !validate_max(value, 64) {
        return Err(HttpFailResult::as_validation_error(
            "Max length is 32 symbols".to_string(),
        ));
    }

    if !validate_no_trimm_spaces(value) {
        return Err(HttpFailResult::as_validation_error(
            "Should not start or end with space".to_string(),
        ));
    }

    if !validate_no_cyrillic(value) {
        return Err(HttpFailResult::as_validation_error(
            "No cyrillic letters are allowed".to_string(),
        ));
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
    match validate_password_text(value) {
        Ok(_) => {
            if !validate_min(value, 8) {
                return Err(HttpFailResult::as_validation_error(
                    "Min length is 8 symbols".to_string(),
                ));
            }

            if !validate_max(value, 50) {
                return Err(HttpFailResult::as_validation_error(
                    "Max length is 32 symbols".to_string(),
                ));
            }

            if !validate_no_trimm_spaces(value) {
                return Err(HttpFailResult::as_validation_error(
                    "Should not start or end with space".to_string(),
                ));
            }

            if !validate_no_cyrillic(value) {
                return Err(HttpFailResult::as_validation_error(
                    "No cyrillic letters are allowed".to_string(),
                ));
            }
            Ok(())
        }
        Err(err_text) => Err(HttpFailResult::as_validation_error(err_text)),
    }
}

pub fn validate_phone(_ctx: &HttpContext, value: &str) -> Result<(), HttpFailResult> {
    if regex::Regex::new(
        r"^\+?(\d{1,3})?[-. (]?(\d{1,4})?[)-. ]?(\d{1,4})[-. ]?(\d{1,4})[-. ]?(\d{1,9})$",
    )
    .unwrap()
    .is_match(value)
    {
        return Ok(());
    }

    Err(HttpFailResult::as_validation_error(
        "Phone is not valid!".to_string(),
    ))
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

pub fn validate_name(_ctx: &HttpContext, value: &str) -> Result<(), HttpFailResult> {
    if !validate_latin_letters_only(value) {
        return Err(HttpFailResult::as_validation_error(
            "Name: Only latin letters are allowed".to_string(),
        ));
    }

    if !validate_max(value, 32) {
        return Err(HttpFailResult::as_validation_error(
            "Name: Max length is 32 symbols".to_string(),
        ));
    }

    return Ok(());
}

pub fn validate_name_with_spaces(_ctx: &HttpContext, value: &str) -> Result<(), HttpFailResult> {
    if !validate_latin_letters_with_spaces(value) {
        return Err(HttpFailResult::as_validation_error(
            "Name: Only latin letters and spaces are allowed".to_string(),
        ));
    }

    if !validate_max(value, 32) {
        return Err(HttpFailResult::as_validation_error(
            "Name: Max length is 32 symbols".to_string(),
        ));
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

pub fn validate_date_of_birth(
    _ctx: &HttpContext,
    value: &DateTimeAsMicroseconds,
) -> Result<(), HttpFailResult> {
    let now = DateTimeAsMicroseconds::now();
    let diff = now.duration_since(*value);

    match diff {
        service_sdk::rust_extensions::date_time::DateTimeDuration::Positive(x) => {
            // turn secunds to years
            let x = x.as_secs() / 60 / 60 / 24 / 365;
            if x < 18 {
                return Err(HttpFailResult::as_validation_error(
                    "DateOfBirth: Should be older than 18".to_string(),
                ));
            }

            return Ok(());
        }
        service_sdk::rust_extensions::date_time::DateTimeDuration::Negative(_) => {}
        service_sdk::rust_extensions::date_time::DateTimeDuration::Zero => {}
    }

    return Err(HttpFailResult::as_validation_error(
        "DateOfBirth: Should be older than 18".to_string(),
    ));
}

pub fn validate_date_of_birth_optional(
    _ctx: &HttpContext,
    value: &Option<DateTimeAsMicroseconds>,
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
        return Err(HttpFailResult::as_validation_error(
            "Address: Max length is 50 symbols".to_string(),
        ));
    }

    if !validate_non_empty_text(value) {
        return Err(HttpFailResult::as_validation_error(
            "Address: Should not be empty".to_string(),
        ));
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
        return Err(HttpFailResult::as_validation_error(
            "City: Max length is 50 symbols".to_string(),
        ));
    }

    if !validate_non_empty_text(value) {
        return Err(HttpFailResult::as_validation_error(
            "City: Should not be empty".to_string(),
        ));
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
        return Err(HttpFailResult::as_validation_error(
            "ZipCode: Max length is 10 symbols".to_string(),
        ));
    }

    if !validate_non_empty_text(value) {
        return Err(HttpFailResult::as_validation_error(
            "ZipCode: Should not be empty".to_string(),
        ));
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

fn validate_latin_letters_only(src: &str) -> bool {
    regex::Regex::new(r"^[a-zA-Z\-]*$").unwrap().is_match(src)
}

fn validate_latin_letters_with_spaces(src: &str) -> bool {
    regex::Regex::new(r"^[a-zA-Z\-]+(\s+[a-zA-Z\-]+)*$").unwrap().is_match(src)
}

fn validate_no_cyrillic(src: &str) -> bool {
    src.chars().all(|c| !is_cyrillic(c))
}

fn is_cyrillic(c: char) -> bool {
    ('\u{0400}'..='\u{04FF}').contains(&c) || 
    ('\u{0500}'..='\u{052F}').contains(&c) ||
    ('\u{2DE0}'..='\u{2DFF}').contains(&c) ||
    ('\u{A640}'..='\u{A69F}').contains(&c)
}

fn validate_no_trimm_spaces(src: &str) -> bool {
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

fn validate_password_text(value: &str) -> Result<(), String> {
    let mut amount_of_special_symbols = 0;

    let mut amount_of_spaces = 0;

    for v in value.as_bytes() {
        if *v <= 32 {
            amount_of_spaces += 1;
        }
        let found_it = SPECIAL_SYMBOLS.iter().find(|c| **c as u8 == *v);

        if found_it.is_some() {
            amount_of_special_symbols += 1;
        }
    }

    if amount_of_spaces > 0 {
        return Err(format!("Password must contain no space characters"));
    }

    if amount_of_special_symbols == 0 {
        return Err(format!(
            "Password must contain at least 1 special symbol such as {:?}",
            SPECIAL_SYMBOLS
        ));
    }

    Ok(())
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
    fn validate_name_with_spaces_correct() {
        assert!(validate_latin_letters_with_spaces("Jhon Do  Doo"));
    }

    #[test]
    fn validate_name_with_spaces_failed() {
        assert!(!validate_latin_letters_with_spaces("Jhon Doo  "));
    }
}

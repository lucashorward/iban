use std::error::Error;

#[derive(Debug)]
struct Iban {
    raw_iban: String,
    machine_iban: String,
    pretty_iban: String,
    country_code: String,
    check_digits: String,
    check_digits_int: i8,
    bban: String,
    is_valid: bool,
}

fn remove_spaces(iban: &String) -> String {
    if !iban.contains(' ') {
        return iban.to_string();
    }
    return iban.split_whitespace().collect();
}

fn get_country_code(iban: &String) -> String {
    return iban.chars().take(2).collect();
}

fn get_check_digits(iban: &String) -> String {
    return iban.chars().skip(2).take(2).collect();
}

fn validate_length(iban: &String) -> bool {
    if iban.len() > 34 || iban.len() < 15 {
        return false;
    }
    return true;
}

fn is_country_code_valid(country_code: String) -> bool {
    return true;
}

fn parse_iban(iban_string: String) -> Result<Iban, String> {
    let sanitised = remove_spaces(&iban_string);
    let iban = Iban {
        raw_iban: iban_string,
        machine_iban: sanitised,
    };

    if !iban.is_valid {
        return Err(String::from("Yikes"));
    }
    Ok(iban)
}

fn is_valid_iban_string(iban: String) -> bool {
    let sanitised_iban = remove_spaces(&iban);

    if !validate_length(&sanitised_iban) {
        return false;
    }

    let country_code = get_country_code(&sanitised_iban);
    if !is_country_code_valid(country_code) {
        return false;
    }

    return true;
    // Remove spaces
    // Length
    // Country code in list
    // Check digits
    // Create checksum
    // Validate checksum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn country_code_works() {
        let input = String::from("GB82 WEST 1234 5698 7654 32");
        let result = get_country_code(&input);
        assert_eq!(result, "GB");
    }

    #[test]
    fn check_digits_works() {
        let input = String::from("GB82 WEST 1234 5698 7654 32");
        let result = get_check_digits(&input);
        assert_eq!(result, "82");
    }

    #[test]
    fn remove_spaces_works() {
        let input = String::from("GB82 WEST 1234 5698 7654 32");
        let result = remove_spaces(&input);
        assert_eq!(result, "GB82WEST12345698765432");
    }

    #[test]
    fn validate_length_works() {
        let input = String::from("GB82 WEST 1234 5698 7654 32");
        let too_long = String::from("GB82 WEST 1234 5698 7654 32 2222 222");
        let too_short = String::from("GB82 WEST 1234");
        let result = validate_length(&input);
        let result_long = validate_length(&too_long);
        let result_short = validate_length(&too_short);
        assert_eq!(result, true);
        assert_eq!(result_long, false);
        assert_eq!(result_short, false);
    }
}
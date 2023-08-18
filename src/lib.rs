#[derive(Debug)]
pub struct Iban {
    pub raw_iban: String,
    pub machine_iban: String,
    // pretty_iban: String,
    pub country_code: String,
    pub check_digits: String,
    pub check_digits_int: i8,
    pub bban: String,
    pub is_valid: bool,
}

fn sanitise_iban(iban: &str) -> String {
    let uppercased = iban.to_uppercase();
    uppercased.split_whitespace().collect()
}

fn get_country_code(iban: &str) -> String {
    iban.chars().take(2).collect()
}

fn get_check_digits(iban: &str) -> String {
    iban.chars().skip(2).take(2).collect()
}

fn get_bban(iban: &str) -> String {
    iban.chars().skip(4).take(iban.len() - 4).collect()
}

fn validate_length(iban: &str) -> bool {
    if iban.len() > 34 || iban.len() < 15 {
        return false;
    }
    true
}

fn is_country_code_valid(_country_code: &str) -> bool {
    true
}

pub fn parse_iban(iban_string: String) -> Iban {
    let sanitised = sanitise_iban(&iban_string);
    let check_digits = get_check_digits(&sanitised);
    let mut iban = Iban {
        raw_iban: iban_string,
        machine_iban: sanitised.clone(),
        is_valid: false,
        country_code: get_country_code(&sanitised),
        check_digits: check_digits.clone(),
        check_digits_int: check_digits.parse().unwrap(),
        bban: get_bban(&sanitised)
    };

    let is_valid = is_valid_iban(&iban);
    iban.is_valid = is_valid;

    iban
}

fn is_valid_iban(iban: &Iban) -> bool {
    if !validate_length(&iban.machine_iban) {
        return false;
    }
    if !is_country_code_valid(&iban.country_code) {
        return false;
    }
    true
}

pub fn is_valid_iban_string(iban: String) -> bool {
    let sanitised_iban = sanitise_iban(&iban);

    if !validate_length(&sanitised_iban) {
        return false;
    }

    let country_code = get_country_code(&sanitised_iban);
    if !is_country_code_valid(&country_code) {
        return false;
    }

    true
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
        let result = sanitise_iban(&input);
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
        assert!(result);
        assert!(!result_long);
        assert!(!result_short);
    }
}

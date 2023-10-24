#![doc=include_str!("../README.md")]

mod core;
mod country;

#[derive(Debug, PartialEq)]
pub struct Iban {
    /// IBAN as provided
    pub raw_iban: String,
    /// Sanitised IBAN, without whitespaces and all uppercased. Will be filled even if the IBAN is invalid.
    pub machine_iban: String,
    // TODO prettify machine IBAN to a string like GB82 WEST 1234 5698 7654 32.
    // pretty_iban: String,
    /// The country code of the IBAN, such as GB or NL.
    pub country_code: String,
    /// Check digits as provided, which are the 2 digits that follow the country code, such as 82.
    pub check_digits: String,
    /// Same as check_digits, but as an int.
    pub check_digits_int: i8,
    /// BBAN is a substring of the IBAN, being the country-specific alphanumeric characters that follow after the country code and check digits.
    pub bban: String,
    /// Whether this is a valid iban or not.
    pub is_valid: bool,
}

/// Parses a string into an `IBAN` struct, containing a sanitised IBAN, is_valid, country code, BBAN and check digits
pub fn parse_iban(iban_string: &str) -> Iban {
    let sanitised = core::sanitise_iban(iban_string);
    let mut chars = sanitised.chars();
    let country_code: String = chars.by_ref().take(2).collect();
    let check_digits: String = chars.by_ref().take(2).collect();
    let bban: String = chars.by_ref().take(40).collect();

    let is_valid = core::is_valid_checksum(&country_code, &check_digits, &bban) && country::is_country_code_valid(&country_code) && core::validate_length(&sanitised);
 
    Iban {
        raw_iban: iban_string.to_string(),
        bban,
        machine_iban: sanitised,
        is_valid,
        country_code,
        check_digits_int: check_digits.clone().parse().unwrap(),
        check_digits
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parser_works() {
        let input = "GB82 WEST 1234 5698 7654 32";

        let result = parse_iban(input);

        assert_eq!(result, Iban {
            country_code: String::from("GB"),
            check_digits: String::from("82"),
            check_digits_int: 82,
            bban: String::from("WEST12345698765432"),
            is_valid: true,
            raw_iban: input.to_string(),
            machine_iban: String::from("GB82WEST12345698765432")
        })
    }

    #[test]
    fn country_code_validation() {
        let good = "GB82 WEST 1234 5698 7654 32";
        let bad = "BB82 WEST 1234 5698 7654 32";

        let good_result = parse_iban(good);
        let bad_result = parse_iban(bad);

        assert!(good_result.is_valid);
        assert!(!bad_result.is_valid);
    }
}

#[cfg(test)]
mod integration_test;
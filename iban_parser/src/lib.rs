#![doc=include_str!("../README.md")]

#![feature(test)]
extern crate test;

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

/// Removes whitespaces, and uppercases the string.
fn sanitise_iban(iban: &str) -> String {
    let uppercased = iban.to_uppercase();
    uppercased.split_whitespace().collect()
}

/// Checks if the string is at least 15, and not more than 34 characters long.
fn validate_length(iban: &str) -> bool {
    if iban.len() > 34 || iban.len() < 15 {
        return false;
    }
    true
}

/// Just a wrapper around format, to make it slightly more readable what we're doing.
fn rearrange(bban: &str, country_code: &str, check_digits: &str) -> String {
    format!("{bban}{country_code}{check_digits}")
}

/// Converts a rearranged IBAN to numbers. Character A = 10, B=11, and so on.
/// Don't ask me, that's how the standard works.
fn convert_to_numbers(rearranged_iban: &str) -> u128 {
    const NUMBER_OFFSET: u8 = 55;
    let chars = rearranged_iban.chars();
    let mut new_string = String::from("");
    for c in chars {
        if c.is_ascii_alphabetic() {
            // Yes this is extremely ugly. I'll probably chop this up in parts later.
            new_string += (c as u8 - NUMBER_OFFSET).to_string().as_str();
        } else {
            new_string += c.to_string().as_str();
        }
    }

    new_string.parse().unwrap()
}

/// Checks whether the generated checksum is valid according to ISO 7064.
fn validate_checksum(checksum: u128) -> bool{
    checksum % 97 == 1
}

fn is_valid_checksum(country_code: &str, check_digits: &str, bban: &str) -> bool {
    let rearranged = rearrange(bban, country_code, check_digits);

    let checksum = convert_to_numbers(&rearranged);

    validate_checksum(checksum)
}

/// Parses a string into an `IBAN` struct, containing a sanitised IBAN, is_valid, country code, BBAN and check digits
pub fn parse_iban(iban_string: &str) -> Iban {
    let sanitised = sanitise_iban(iban_string);
    let mut chars = sanitised.chars();
    let country_code: String = chars.by_ref().take(2).collect();
    let check_digits: String = chars.by_ref().take(2).collect();
    let bban: String = chars.by_ref().take(40).collect();

    let is_valid = is_valid_checksum(&country_code, &check_digits, &bban) && country::is_country_code_valid(&country_code) && validate_length(&sanitised);
 
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
    fn remove_spaces_works() {
        let input = "GB82 WEST 1234 5698 7654 32";
        let result = sanitise_iban(&input);
        assert_eq!(result, "GB82WEST12345698765432");
    }

    #[test]
    fn validate_length_works() {
        let input = "GB82 WEST 1234 5698 7654 32";
        let too_long ="GB82 WEST 1234 5698 7654 32 2222 222";
        let too_short = "GB82 WEST 1234";
        let result = validate_length(input);
        let result_long = validate_length(too_long);
        let result_short = validate_length(too_short);
        assert!(result);
        assert!(!result_long);
        assert!(!result_short);
    }

    #[test]
    fn validate_checksum_works() {
        let good = validate_checksum(3214282912345698765432161182);
        let bad= validate_checksum(3214282912345698765432161181);

        assert!(good);
        assert!(!bad);
    }

    #[test]
    fn convert_to_numbers_works() {
        let input = "WEST12345698765432GB82";

        let result = convert_to_numbers(input);

        assert_eq!(result, 3214282912345698765432161182)
    }

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

    #[bench]
    fn bench_parse_iban(b: &mut test::Bencher) {
        b.iter(|| parse_iban("GB82 WEST 1234 5698 7654 32"));
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
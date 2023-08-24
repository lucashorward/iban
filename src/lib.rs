#![feature(test)]
extern crate test;

#[derive(Debug)]
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

/// This is a stub for now while I figure out a good way to do this.
fn is_country_code_valid(_country_code: &str) -> bool {
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
pub fn parse_iban(iban_string: &String) -> Iban {
    let sanitised = sanitise_iban(iban_string);
    let mut chars = sanitised.chars();
    let country_code: String = chars.by_ref().take(2).collect();
    let check_digits: String = chars.by_ref().take(2).collect();
    let bban: String = chars.by_ref().take(40).collect();
    println!("{country_code}, {check_digits}");
    let is_valid = is_valid_checksum(&country_code, &check_digits, &bban);
 
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

/// Validates an IBAN string.
/// 
/// Checks for length and the basic mod-97 operation (ISO 7064).
/// 
/// Country code and country-specific validations are not done.
pub fn is_valid_iban_string(iban: &str) -> bool {
    let sanitised_iban = sanitise_iban(iban);

    // Check that it's not too long or too short
    if !validate_length(&sanitised_iban) {
        return false;
    }

    // This does nothing for now, while I think of a good solution for this that doesn't involve a very long enum
    let mut chars = sanitised_iban.chars();
    let country_code: String = chars.by_ref().take(2).collect();
    let check_digits: String = chars.by_ref().take(2).collect();
    let bban: String = chars.by_ref().take(40).collect();
    if !is_country_code_valid(&country_code) {
        return false;
    }

    is_valid_checksum(&country_code, &check_digits, &bban)
}

#[cfg(test)]
mod tests {
    use super::*;

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

    #[test]
    fn validate_checksum_works() {
        let good = validate_checksum(3214282912345698765432161182);
        let bad= validate_checksum(3214282912345698765432161181);

        assert!(good);
        assert!(!bad);
    }

    #[test]
    fn convert_to_numbers_works() {
        let input = String::from("WEST12345698765432GB82");

        let result = convert_to_numbers(&input);

        assert_eq!(result, 3214282912345698765432161182)
    }

    #[test]
    fn is_valid_iban_string_works() {
        let good = String::from("GB82 WEST 1234 5698 7654 32");
        let bad = String::from("GB82 WEST 1234 5698 7654 34");

        let good_result = is_valid_iban_string(&good);
        let bad_result = is_valid_iban_string(&bad);

        assert!(good_result);
        assert!(!bad_result);
    }

    #[bench]
    fn bench_parse_iban(b: &mut test::Bencher) {
        b.iter(|| parse_iban(&String::from("GB82 WEST 1234 5698 7654 32")));
    }

    #[bench]
    fn bench_validate_iban(b: &mut test::Bencher) {
        b.iter(|| is_valid_iban_string(&String::from("GB82 WEST 1234 5698 7654 32")));
    }
}

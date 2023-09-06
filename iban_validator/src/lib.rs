#![doc=include_str!("../README.md")]
#![feature(test)]

use country::{get_rule_from_country_code, CheckingAlgorithms};

mod country;

extern crate test;

/// Removes whitespaces, and uppercases the string.
fn sanitise_iban(iban: &str) -> String {
    let uppercased = iban.to_uppercase();
    uppercased.split_whitespace().collect()
}

/// Checks if the string is at least 15, and not more than the max_length
fn validate_length(iban: &str, max_length: usize) -> bool {
    if iban.len() > max_length || iban.len() < 15 {
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
fn validate_checksum(checksum: u128, modelo: u128) -> bool{
    checksum % modelo == 1
}

fn is_valid_checksum(country_code: &str, check_digits: &str, bban: &str, modelo: u128) -> bool {    let rearranged = rearrange(bban, country_code, check_digits);

    let checksum = convert_to_numbers(&rearranged);

    validate_checksum(checksum, modelo)
}

/// Validates an IBAN string.
/// 
/// Checks for length and the basic mod-97 operation (ISO 7064).
/// 
/// Country code and country-specific validations are not done.
pub fn is_valid_iban_string(iban: &str) -> bool {
    let sanitised_iban = sanitise_iban(iban);


    // This does nothing for now, while I think of a good solution for this that doesn't involve a very long enum
    let mut chars = sanitised_iban.chars();
    let country_code: String = chars.by_ref().take(2).collect();
    let rules_opt = get_rule_from_country_code(&country_code);
    if rules_opt.is_none() {
        return false;
    }
    let rules = rules_opt.unwrap();

    // Check that it's not too long or too short
    if !validate_length(&sanitised_iban, rules.max_length.into()) {
        return false;
    }


    let check_digits: String = chars.by_ref().take(2).collect();
    let bban: String = chars.by_ref().take(40).collect();

    for modelo in rules.checking_algorithms {
        let result = is_valid_checksum(&country_code, &check_digits, &bban, country::algorithm_to_mod(modelo));
        if !result{
            return false;
        }
    }
    true

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
        let result = validate_length(&input, 34);
        let result_long = validate_length(&too_long, 34);
        let result_short = validate_length(&too_short, 34);
        assert!(result);
        assert!(!result_long);
        assert!(!result_short);
    }

    #[test]
    fn validate_checksum_works() {
        let good = validate_checksum(3214282912345698765432161182, 97);
        let bad= validate_checksum(3214282912345698765432161181, 97);

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
    fn bench_validate_iban(b: &mut test::Bencher) {
        b.iter(|| is_valid_iban_string(&String::from("GB82 WEST 1234 5698 7654 32")));
    }
}

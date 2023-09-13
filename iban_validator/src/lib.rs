#![doc=include_str!("../README.md")]
#![feature(test)]
extern crate test;

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
    println!("{new_string}");
    new_string.parse::<u128>().unwrap()
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
        let input = "GB82 WEST 1234 5698 7654 32";
        let result = sanitise_iban(input);
        assert_eq!(result, "GB82WEST12345698765432");
    }

    #[test]
    fn validate_length_works() {
        let input = "GB82 WEST 1234 5698 7654 32";
        let too_long = "GB82 WEST 1234 5698 7654 32 2222 222";
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
    fn is_valid_iban_string_works() {
        let good = "GB82 WEST 1234 5698 7654 32";
        let bulgarian = "BG59STSA93003897385696";
        let bad = "GB82 WEST 1234 5698 7654 34";

        let good_result = is_valid_iban_string(good);
        let bulgarian_result = is_valid_iban_string(bulgarian);
        let bad_result = is_valid_iban_string(bad);

        assert!(good_result);
        assert!(bulgarian_result);
        assert!(!bad_result);
    }

    #[bench]
    fn bench_validate_iban(b: &mut test::Bencher) {
        b.iter(|| is_valid_iban_string("GB82 WEST 1234 5698 7654 32"));
    }
}

#[cfg(test)]
mod integration_test;

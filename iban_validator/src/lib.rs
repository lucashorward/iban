#![doc=include_str!("../README.md")]
#![feature(test)]
extern crate test;

mod core;
mod country;

/// Validates an IBAN string.
/// 
/// Checks for length and the basic mod-97 operation (ISO 7064).
/// 
/// Country code and country-specific validations are not done.
pub fn is_valid_iban_string(iban: &str) -> bool {
    let sanitised_iban = core::sanitise_iban(iban);

    // Check that it's not too long or too short
    if !core::validate_length(&sanitised_iban) {
        return false;
    }

    // This does nothing for now, while I think of a good solution for this that doesn't involve a very long enum
    let mut chars = sanitised_iban.chars();
    let country_code: String = chars.by_ref().take(2).collect();
    let check_digits: String = chars.by_ref().take(2).collect();
    let bban: String = chars.by_ref().take(40).collect();
    if !country::is_country_code_valid(&country_code) {
        println!("Country code");
        return false;
    }

    core::is_valid_checksum(&country_code, &check_digits, &bban)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_valid_iban_string_works() {
        let good = "GB82 WEST 1234 5698 7654 32";
        let bad = "GB82 WEST 1234 5698 7654 34";

        let good_result = is_valid_iban_string(good);
        let bad_result = is_valid_iban_string(bad);

        assert!(good_result);
        assert!(!bad_result);
    }

    #[test]
    fn country_code_validation() {
        let good = "GB82 WEST 1234 5698 7654 32";
        let bad = "BB82 WEST 1234 5698 7654 32";

        let good_result = is_valid_iban_string(good);
        let bad_result = is_valid_iban_string(bad);

        assert!(good_result);
        assert!(!bad_result);
    }

    #[bench]
    fn bench_validate_iban(b: &mut test::Bencher) {
        b.iter(|| is_valid_iban_string("GB82 WEST 1234 5698 7654 32"));
    }
}

#[cfg(test)]
mod integration_test;

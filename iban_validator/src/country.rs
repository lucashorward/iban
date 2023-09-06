pub struct CountryRules<'a> {
    pub country_code: &'a str,
    pub max_length: u8,
    pub checking_algorithms: Vec<CheckingAlgorithms>
}

pub enum CheckingAlgorithms {
    MOD97 = 97,
}

// This will probably need to change at some point as
// some specific countries use stuff like MOD11 with their own custom weights
pub fn algorithm_to_mod(algorithm: CheckingAlgorithms) -> u128 {
    match algorithm {
        CheckingAlgorithms::MOD97 => 97,
    }
}

/// A very ugly but fast way of checking the country code and getting max_length and algorithms per country
pub fn get_rule_from_country_code(country_code: &str) -> Option<CountryRules> {
    match country_code {
        "AL" => Some(CountryRules {
            country_code: country_code,
            max_length: 28,
            checking_algorithms: vec![CheckingAlgorithms::MOD97],
        }),
        "AD" => Some(CountryRules {
            country_code: country_code,
            max_length: 24,
            checking_algorithms: vec![CheckingAlgorithms::MOD97],
        }),
        "AT" => Some(CountryRules {
            country_code: country_code,
            max_length: 20,
            checking_algorithms: vec![CheckingAlgorithms::MOD97],
        }),
        "AZ" => Some(CountryRules {
            country_code: country_code,
            max_length: 28,
            checking_algorithms: vec![CheckingAlgorithms::MOD97],
        }),
        "BH" => Some(CountryRules {
            country_code: country_code,
            max_length: 22,
            checking_algorithms: vec![CheckingAlgorithms::MOD97],
        }),
        "BY" => Some(CountryRules {
            country_code: country_code,
            max_length: 28,
            checking_algorithms: vec![CheckingAlgorithms::MOD97],
        }),
        "BE" => Some(CountryRules {
            country_code: country_code,
            max_length: 16,
            checking_algorithms: vec![CheckingAlgorithms::MOD97],
        }),
        "BA" => Some(CountryRules {
            country_code: country_code,
            max_length: 20,
            checking_algorithms: vec![CheckingAlgorithms::MOD97],
        }),
        "BR" => Some(CountryRules {
            country_code: country_code,
            max_length: 29,
            checking_algorithms: vec![CheckingAlgorithms::MOD97],
        }),
        "BG" => Some(CountryRules {
            country_code: country_code,
            max_length: 22,
            checking_algorithms: vec![CheckingAlgorithms::MOD97],
        }),
        "BI" => Some(CountryRules {
            country_code: country_code,
            max_length: 27,
            checking_algorithms: vec![CheckingAlgorithms::MOD97],
        }),
        "CR" => Some(CountryRules {
            country_code: country_code,
            max_length: 22,
            checking_algorithms: vec![CheckingAlgorithms::MOD97],
        }),
        "HR" => Some(CountryRules {
            country_code: country_code,
            max_length: 21,
            checking_algorithms: vec![CheckingAlgorithms::MOD97],
        }),
        "CY" => Some(CountryRules {
            country_code: country_code,
            max_length: 28,
            checking_algorithms: vec![CheckingAlgorithms::MOD97],
        }),
        "CZ" => Some(CountryRules {
            country_code: country_code,
            max_length: 24,
            checking_algorithms: vec![CheckingAlgorithms::MOD97],
        }),
        "DK" => Some(CountryRules {
            country_code: country_code,
            max_length: 18,
            checking_algorithms: vec![CheckingAlgorithms::MOD97],
        }),
        "DJ" => Some(CountryRules {
            country_code: country_code,
            max_length: 27,
            checking_algorithms: vec![CheckingAlgorithms::MOD97],
        }),
        "DO" => Some(CountryRules {
            country_code: country_code,
            max_length: 28,
            checking_algorithms: vec![CheckingAlgorithms::MOD97],
        }),
        "EG" => Some(CountryRules {
            country_code: country_code,
            max_length: 29,
            checking_algorithms: vec![CheckingAlgorithms::MOD97],
        }),
        "SV" => Some(CountryRules {
            country_code: country_code,
            max_length: 28,
            checking_algorithms: vec![CheckingAlgorithms::MOD97],
        }),
        "EE" => Some(CountryRules {
            country_code: country_code,
            max_length: 20,
            checking_algorithms: vec![CheckingAlgorithms::MOD97],
        }),
        "FK" => Some(CountryRules {
            country_code: country_code,
            max_length: 18,
            checking_algorithms: vec![CheckingAlgorithms::MOD97],
        }),
        "FO" => Some(CountryRules {
            country_code: country_code,
            max_length: 18,
            checking_algorithms: vec![CheckingAlgorithms::MOD97],
        }),
        "FI" => Some(CountryRules {
            country_code: country_code,
            max_length: 18,
            checking_algorithms: vec![CheckingAlgorithms::MOD97],
        }),
        "FR" => Some(CountryRules {
            country_code: country_code,
            max_length: 27,
            checking_algorithms: vec![CheckingAlgorithms::MOD97],
        }),
        "GE" => Some(CountryRules {
            country_code: country_code,
            max_length: 22,
            checking_algorithms: vec![CheckingAlgorithms::MOD97],
        }),
        "DE" => Some(CountryRules {
            country_code: country_code,
            max_length: 22,
            checking_algorithms: vec![CheckingAlgorithms::MOD97],
        }),
        "GI" => Some(CountryRules {
            country_code: country_code,
            max_length: 23,
            checking_algorithms: vec![CheckingAlgorithms::MOD97],
        }),
        "GB" => Some(CountryRules {
            country_code: country_code,
            max_length: 22,
            checking_algorithms: vec![CheckingAlgorithms::MOD97],
        }),

        _ => None
    }
}

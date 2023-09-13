extern crate iban_validator;

fn main() {
    println!("Hello, world!");
    iban_validator::is_valid_iban_string("AL35202111090000000001234567");
}

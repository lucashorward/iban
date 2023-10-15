extern crate iban_validator;

fn main() {
    println!("Hello, world!");
    let result = iban_validator::is_valid_iban_string("GB82 WEST 1234 5698 7654 32");
    println!("{result}");
}

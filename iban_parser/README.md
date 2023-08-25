# About

A zero-dependency, lightweight IBAN validator✨

This is a hobby project where I learn both about IBANs and how they are validated, and the Rust language.

As it's a hobby project, I can't guarantee it will work 100% of the time. I'm still learning, about both IBAN validation in general and Rust.

Looking for a just a validator? Check out my iban_validator package!

# Out of scope

Things I haven't taken into account as of the time of writing:

- Checking each and every country code
- Countries where the validation is slightly different
- Countries that have additional validation on top of the generic IBAN validation

Please bear with me until I've done this, or help out and make your own MR :)

# Usage

`parse_iban()` will take the IBAN string and parse it to a `Iban` struct, so you can get information like the country code, BBAN, check digits etc. Will also perform the validity check, and return an `is_valid` property as well.
# About

A zero-dependency, lightweight IBAN validator✨

This is a hobby project where I learn both about IBANs and how they are validated, and the Rust language.

As it's a hobby project, I can't guarantee it will work 100% of the time. I'm still learning, about both IBAN validation in general and Rust.

Looking for a parser? Check out my iban_parser package!

# Out of scope

Things I haven't taken into account as of the time of writing:

- Countries where the validation is slightly different
- Countries that have additional validation on top of the generic IBAN validation
- Changing a machine-readable IBAN into a nice, human-readable one

Please bear with me until I've done this, or help out and make your own MR :)

# Usage

Call the `is_valid_iban_string` function :)
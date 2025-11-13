use std::hash::Hash;

use color_eyre::eyre::{Result, eyre};
use rand::Rng;
use secrecy::{SecretString, ExposeSecret};

#[derive(Clone, Debug)]
pub struct TwoFACode(SecretString);

impl PartialEq for TwoFACode {
    fn eq(&self, other: &Self) -> bool {
        self.0.expose_secret() == other.0.expose_secret()
    }
}

impl Eq for TwoFACode {}

impl Hash for TwoFACode {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.expose_secret().hash(state);
    }
}


impl TwoFACode {
    pub fn parse(code: SecretString) -> Result<Self> {
        // Ensure `code` is a valid 6-digit code
        match is_valid_code(&code.expose_secret()) {
            true => Ok(Self(code)),
            _ => Err(eyre!("Invalid 2FA code")),
        }
    }
}

fn is_valid_code(code: &str) -> bool {
    // Check if the code is exactly 6 characters long.
    if code.len() != 6 {
        return false;
    }
    // Check if all characters are digits.
    for c in code.chars() {
        if !c.is_ascii_digit() {
            return false;
        }
    }
    // If all checks pass, the code is valid.
    true
}

impl Default for TwoFACode {
    fn default() -> Self {
        // Use the `rand` crate to generate a random 2FA code.
        // The code should be 6 digits (ex: 834629)
        Self(generate_code())
    }
}

fn generate_code() -> SecretString {
    let mut rng = rand::rng();
    let mut code = String::new();

    for _ in 0..6 {
        let digit = rng.random_range(0..10); // Generate a random digit between 0 and 9
        code.push_str(&digit.to_string()); // Convert the digit to a string and append to the code
    }

    code.into()
}

// TODO: Implement AsRef<str> for TwoFACode
impl AsRef<SecretString> for TwoFACode {
    fn as_ref(&self) -> &SecretString {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;

    #[test]
    fn test_valid_code() {
        assert_eq!(is_valid_code("123456".into()), true);
        assert_eq!(is_valid_code("987654".into()), true);
        assert_eq!(is_valid_code("000000".into()), true);
    }

    #[test]
    fn test_invalid_code_too_short() {
        assert_eq!(is_valid_code("12345".into()), false);
        assert_eq!(is_valid_code("1234".into()), false);
    }

    #[test]
    fn test_invalid_code_too_long() {
        assert_eq!(is_valid_code("1234567".into()), false);
        assert_eq!(is_valid_code("12345678".into()), false);
    }

    #[test]
    fn test_invalid_code_non_digit() {
        assert_eq!(is_valid_code("123a56".into()), false);
        assert_eq!(is_valid_code("12345-6".into()), false);
        assert_eq!(is_valid_code("12345.6".into()), false);
    }

    #[test]
    fn test_generate_code() {
        let code = generate_code();
        assert_eq!(code.expose_secret().len(), 6);
        for c in code.expose_secret().chars() {
            assert!(c.is_digit(10));
        }
    }

    #[test]
    fn test_generate_multiple_codes() {
        let mut codes = Vec::new();
        for _ in 0..10 {
            codes.push(generate_code());
        }

        // Check if the codes are unique (probabilistic check)
        let mut seen: HashSet<String> = std::collections::HashSet::new();
        for code in &codes {
            assert!(seen.insert(code.expose_secret().to_string()));
            assert!(is_valid_code(code.expose_secret()))
        }
    }
}

use rand::Rng;

#[derive(Clone, Debug, PartialEq)]
pub struct TwoFACode(String);

impl TwoFACode {
    pub fn parse(code: String) -> Result<Self, String> {
        // Ensure `code` is a valid 6-digit code
        match is_valid_code(&code) {
            true => Ok(Self(code)),
            _ => Err("Invalid 2FA code".to_owned()),
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
        if !c.is_digit(10) {
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

fn generate_code() -> String {
    let mut rng = rand::rng();
    let mut code = String::new();

    for _ in 0..6 {
        let digit = rng.random_range(0..10); // Generate a random digit between 0 and 9
        code.push_str(&digit.to_string()); // Convert the digit to a string and append to the code
    }

    code
}

// TODO: Implement AsRef<str> for TwoFACode
impl AsRef<str> for TwoFACode {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_code() {
        assert_eq!(is_valid_code("123456"), true);
        assert_eq!(is_valid_code("987654"), true);
        assert_eq!(is_valid_code("000000"), true);
    }

    #[test]
    fn test_invalid_code_too_short() {
        assert_eq!(is_valid_code("12345"), false);
        assert_eq!(is_valid_code("1234"), false);
    }

    #[test]
    fn test_invalid_code_too_long() {
        assert_eq!(is_valid_code("1234567"), false);
        assert_eq!(is_valid_code("12345678"), false);
    }

    #[test]
    fn test_invalid_code_non_digit() {
        assert_eq!(is_valid_code("123a56"), false);
        assert_eq!(is_valid_code("12345-6"), false);
        assert_eq!(is_valid_code("12345.6"), false);
    }

    #[test]
    fn test_generate_code() {
        let code = generate_code();
        assert_eq!(code.len(), 6);
        for c in code.chars() {
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
        let mut seen = std::collections::HashSet::new();
        for code in &codes {
            assert!(seen.insert(code));
            assert!(is_valid_code(code))
        }
    }
}

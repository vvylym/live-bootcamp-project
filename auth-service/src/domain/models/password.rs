use crate::domain::error::*;

#[derive(Clone, Debug, PartialEq)]
pub struct Password(String);

impl Password {
    /// Parses a string into an Email.
    /// Returns an error if the string is not a valid email address.
    pub fn parse(s: &str) -> Result<Self, AuthAPIError> {
        if validate_password(s) {
            Ok(Password(s.to_string()))
        } else {
            Err(AuthAPIError::InvalidPassword)
        }
    }
}

fn validate_password(s: &str) -> bool {
    s.len() >= 8
}

impl AsRef<str> for Password {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
#[cfg(test)]
mod tests {
    use super::Password;

    use fake::Fake;
    use fake::faker::internet::en::Password as FakePassword;

    #[test]
    fn empty_string_is_rejected() {
        let password = "";
        assert!(Password::parse(password).is_err());
    }
    #[test]
    fn string_less_than_8_characters_is_rejected() {
        let password = "1234567";
        assert!(Password::parse(password).is_err());
    }

    #[derive(Debug, Clone)]
    struct ValidPasswordFixture(pub String);

    impl quickcheck::Arbitrary for ValidPasswordFixture {
        fn arbitrary(_: &mut quickcheck::Gen) -> Self {
            let password = FakePassword(8..30).fake();
            Self(password)
        }
    }
    #[quickcheck_macros::quickcheck]
    fn valid_passwords_are_parsed_successfully(valid_password: ValidPasswordFixture) -> bool {
        Password::parse(&valid_password.0).is_ok()
    }
}

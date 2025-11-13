use color_eyre::eyre::Result;
use secrecy::{ExposeSecret, SecretString};

use crate::domain::error::*;

#[derive(Clone, Debug)]
pub struct Password(SecretString);

impl Password {
    /// Parses a string into an Email.
    /// Returns an error if the string is not a valid email address.
    pub fn parse(s: SecretString) -> Result<Self, AuthAPIError> {
        if validate_password(&s) {
            Ok(Password(s))
        } else {
            Err(AuthAPIError::InvalidPassword)
        }
    }
}

impl PartialEq for Password {
    fn eq(&self, other: &Self) -> bool {
        self.0.expose_secret() == other.0.expose_secret()
    }
}

fn validate_password(s: &SecretString) -> bool {
    s.expose_secret().len() >= 8
}

impl AsRef<SecretString> for Password {
    fn as_ref(&self) -> &SecretString {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::Password;

    use fake::Fake;
    use fake::faker::internet::en::Password as FakePassword;
    use secrecy::SecretString;

    #[test]
    fn empty_string_is_rejected() {
        let password = SecretString::from("".to_string());
        assert!(Password::parse(password).is_err());
    }
    #[test]
    fn string_less_than_8_characters_is_rejected() {
        let password = SecretString::from("1234567");
        assert!(Password::parse(password).is_err());
    }

    #[derive(Debug, Clone)]
    struct ValidPasswordFixture(pub SecretString);

    impl quickcheck::Arbitrary for ValidPasswordFixture {
        fn arbitrary(_: &mut quickcheck::Gen) -> Self {
            let password: String = FakePassword(8..30).fake();
            Self(SecretString::from(password))
        }
    }
    #[quickcheck_macros::quickcheck]
    fn valid_passwords_are_parsed_successfully(valid_password: ValidPasswordFixture) -> bool {
        Password::parse(valid_password.0).is_ok()
    }
}

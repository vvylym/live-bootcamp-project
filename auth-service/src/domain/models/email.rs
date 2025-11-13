use std::hash::Hash;

use crate::domain::error::AuthAPIError;
use secrecy::{ExposeSecret, SecretString};
use validator::ValidateEmail;

/// Email
#[derive(Clone, Debug)]
pub struct Email(SecretString);

impl PartialEq for Email {
    fn eq(&self, other: &Self) -> bool {
        self.0.expose_secret() == other.0.expose_secret()
    }
}

impl Eq for Email {}

impl Hash for Email {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.expose_secret().hash(state);
    }
}



impl Email {
    /// Parses a string into an Email.
    /// Returns an error if the string is not a valid email address.
    pub fn parse(s: SecretString) -> Result<Self, AuthAPIError> {
        if s.expose_secret().validate_email() {
            Ok(Self(s))
        } else {
            Err(AuthAPIError::InvalidEmail)
        }
    }
}

impl AsRef<SecretString> for Email {
    fn as_ref(&self) -> &SecretString {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use fake::Fake;
    use fake::faker::internet::en::SafeEmail;

    #[test]
    fn empty_string_is_rejected() {
        let email = "";
        assert!(Email::parse(email.into()).is_err());
    }
    #[test]
    fn email_missing_at_symbol_is_rejected() {
        let email = "ursuladomain.com";
        assert!(Email::parse(email.into()).is_err());
    }
    #[test]
    fn email_missing_subject_is_rejected() {
        let email = "@domain.com";
        assert!(Email::parse(email.into()).is_err());
    }

    #[derive(Debug, Clone)]
    struct ValidEmailFixture(pub String);

    impl quickcheck::Arbitrary for ValidEmailFixture {
        fn arbitrary(_: &mut quickcheck::Gen) -> Self {
            // Remove use of fake_with_rng due to multiple "rand" version
            let email = SafeEmail().fake();
            Self(email)
        }
    }

    #[quickcheck_macros::quickcheck]
    fn valid_emails_are_parsed_successfully(valid_email: ValidEmailFixture) -> bool {
        Email::parse(SecretString::from(valid_email.0)).is_ok()
    }
}

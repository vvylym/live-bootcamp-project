use crate::domain::error::AuthAPIError;
use validator::ValidateEmail;

/// Email
#[derive(Clone, Debug, PartialEq, Hash, Eq)]
pub struct Email(String);

impl Email {
    /// Parses a string into an Email.
    /// Returns an error if the string is not a valid email address.
    pub fn parse(s: &str) -> Result<Self, AuthAPIError> {
        if s.validate_email() {
            Ok(Self(s.to_string()))
        } else {
            Err(AuthAPIError::InvalidEmail)
        }
    }
}

impl AsRef<str> for Email {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::Email;

    use fake::Fake;
    use fake::faker::internet::en::SafeEmail;

    #[test]
    fn empty_string_is_rejected() {
        let email = "";
        assert!(Email::parse(email).is_err());
    }
    #[test]
    fn email_missing_at_symbol_is_rejected() {
        let email = "ursuladomain.com";
        assert!(Email::parse(email).is_err());
    }
    #[test]
    fn email_missing_subject_is_rejected() {
        let email = "@domain.com";
        assert!(Email::parse(email).is_err());
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
        Email::parse(&valid_email.0).is_ok()
    }
}

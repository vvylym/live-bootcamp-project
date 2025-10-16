use crate::domain::error::AuthAPIError;

#[derive(Clone, Debug, PartialEq)]
pub struct User {
    /// The user's email address.
    pub email: Email,
    /// The user's password.
    pub password: Password,
    /// Indicates if two-factor authentication is required.
    pub requires_2fa: bool,
}

impl User {
    // add a constructor function called `new`
    pub fn new(email: Email, password: Password, requires_2fa: bool) -> Self {
        Self {
            email,
            password,
            requires_2fa,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Hash)]
pub struct Email(String);

impl Email {
    /// Parses a string into an Email.
    /// Returns an error if the string is not a valid email address.
    pub fn parse(s: &str) -> Result<Self, AuthAPIError> {
        if s.is_empty() || !s.contains('@') {
            Err(AuthAPIError::InvalidEmail)
        } else {
            Ok(Email(s.to_string()))
        }
    }
}

impl AsRef<str> for Email {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Password(String);

impl Password {
    /// Parses a string into an Email.
    /// Returns an error if the string is not a valid email address.
    pub fn parse(s: &str) -> Result<Self, AuthAPIError> {
        if s.len() < 8 {
            Err(AuthAPIError::InvalidPassword)
        } else {
            Ok(Password(s.to_string()))
        }
    }
}

impl AsRef<str> for Password {
    fn as_ref(&self) -> &str {
        &self.0
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_email() {
        let email = Email::parse("@");
        assert!(email.is_ok());
    }

    #[test]
    fn test_invalid_email() {
        let email = Email::parse("");
        assert!(email.is_err());
    }

    #[test]
    fn test_valid_password() {
        let pwd = Password::parse("12345678");
        assert!(pwd.is_ok());
    }

    #[test]
    fn test_invalid_password() {
        let pwd = Password::parse("1234567");
        assert!(pwd.is_err());
    }
}
       
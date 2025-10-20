use super::{Email, Password};

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

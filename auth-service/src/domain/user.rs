#[derive(Clone, Debug, PartialEq)]
pub struct User {
    /// The user's email address.
    pub email: String,
    /// The user's password.
    pub password: String,
    /// Indicates if two-factor authentication is required.
    pub requires_2fa: bool,
}

impl User {
    // add a constructor function called `new`
    pub fn new(email: String, password: String, requires_2fa: bool) -> Self {
        Self {
            email,
            password,
            requires_2fa,
        }
    }
}
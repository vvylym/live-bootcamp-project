use std::hash::Hash;

use color_eyre::eyre::{Context, Result};
use secrecy::{ExposeSecret, SecretString};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct LoginAttemptId(SecretString);

impl PartialEq for LoginAttemptId {
    fn eq(&self, other: &Self) -> bool {
        self.0.expose_secret() == other.0.expose_secret()
    }
}

impl Eq for LoginAttemptId {}

impl Hash for LoginAttemptId {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.expose_secret().hash(state);
    }
}

impl LoginAttemptId {
    pub fn parse(id: SecretString) -> Result<Self> {
        // Use the `parse_str` function from the `uuid` crate to ensure `id` is a valid UUID
        let parse_id = Uuid::parse_str(id.expose_secret()).wrap_err("Invalid login attempt id")?;

        Ok(Self(parse_id.to_string().into()))
    }
}

impl Default for LoginAttemptId {
    fn default() -> Self {
        // Use the `uuid` crate to generate a random version 4 UUID
        Self(Uuid::new_v4().to_string().into())
    }
}

impl AsRef<SecretString> for LoginAttemptId {
    fn as_ref(&self) -> &SecretString {
        &self.0
    }
}

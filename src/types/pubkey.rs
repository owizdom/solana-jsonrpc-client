use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Pubkey(String);

impl Pubkey {
    pub fn new(pubkey: String) -> Self {
        Self(pubkey)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<String> for Pubkey {
    fn from(s: String) -> Self {
        Self(s)
    }
}

impl From<&str> for Pubkey {
    fn from(s: &str) -> Self {
        Self(s.to_string())
    }
}

impl fmt::Display for Pubkey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}



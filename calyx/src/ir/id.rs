use crate::errors::Span;
use derivative::Derivative;
use serde::{Deserialize, Serialize};

/// Represents an identifier in a Calyx program
#[derive(Derivative, Clone, PartialOrd, Ord, Deserialize)]
#[derivative(Hash, Eq, Debug)]
#[serde(transparent)]
pub struct Id {
    pub id: String,
    #[derivative(Hash = "ignore")]
    #[derivative(Debug = "ignore")]
    #[serde(skip)]
    span: Option<Span>,
}

impl Id {
    pub fn new<S: ToString>(id: S, span: Option<Span>) -> Self {
        Self {
            id: id.to_string(),
            span,
        }
    }

    pub fn fmt_err(&self, err_msg: &str) -> String {
        match &self.span {
            Some(span) => span.format(err_msg),
            None => err_msg.to_string(),
        }
    }
}

/* =================== Impls for Id to make them easier to use ============== */

impl std::fmt::Display for Id {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self.id)
    }
}

impl AsRef<str> for Id {
    fn as_ref(&self) -> &str {
        &self.id
    }
}

impl From<&str> for Id {
    fn from(s: &str) -> Self {
        Id {
            id: s.to_string(),
            span: None,
        }
    }
}

impl From<String> for Id {
    fn from(s: String) -> Self {
        Id { id: s, span: None }
    }
}

impl PartialEq<str> for Id {
    fn eq(&self, other: &str) -> bool {
        self.id == other
    }
}

impl<S: AsRef<str>> PartialEq<S> for Id {
    fn eq(&self, other: &S) -> bool {
        self.id == other.as_ref()
    }
}

impl Serialize for Id {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.id)
    }
}

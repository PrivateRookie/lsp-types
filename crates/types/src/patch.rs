use super::Integer;
use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum OneOf<T, U> {
    One(T),
    Other(U),
}

#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum OneOf3<T, U, X> {
    One(T),
    Mid(U),
    Other(X)
}

impl<T: Default, U> OneOf<T, U> {
    pub fn one_default() -> Self {
        Self::One(T::default())
    }

    pub fn opt_one_default() -> Option<Self> {
        Some(Self::one_default())
    }
}

impl<T, U: Default> OneOf<T, U> {
    pub fn other_default() -> Self {
        Self::Other(U::default())
    }

    pub fn opt_other_default() -> Option<Self> {
        Some(Self::other_default())
    }
}

pub type ReqId = OneOf<Integer, String>;
pub type ProgressToken = OneOf<Integer, String>;
pub type DiagnosticCode = OneOf<Integer, String>;

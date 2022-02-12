use crate::ResponseMessage;

use super::Integer;
use serde::{Deserialize, Serialize};

#[doc = "empty response data"]
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct Empty {}

#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum OneOf<T, U> {
    One(T),
    Other(U),
}

impl<T, U> OneOf<T, U> {
    pub fn map<X>(self, f: impl FnOnce(T) -> X) -> OneOf<X, U> {
        match self {
            OneOf::One(t) => OneOf::One(f(t)),
            OneOf::Other(u) => OneOf::Other(u),
        }
    }

    pub fn map_or<X>(self, f: impl FnOnce(U) -> X) -> OneOf<T, X> {
        match self {
            OneOf::One(t) => OneOf::One(t),
            OneOf::Other(u) => OneOf::Other(f(u)),
        }
    }

    pub fn unify(self, f: impl FnOnce(U) -> T) -> T {
        match self {
            OneOf::One(t) => t,
            OneOf::Other(u) => f(u),
        }
    }

    pub fn transpose(self) -> OneOf<U, T> {
        match self {
            OneOf::One(t) => OneOf::Other(t),
            OneOf::Other(u) => OneOf::One(u),
        }
    }
}

impl<T, U> OneOf<T, OneOf<T, U>> {
    pub fn flat_or(self) -> OneOf<T, U> {
        match self {
            OneOf::One(t) => OneOf::One(t),
            OneOf::Other(u) => u.map(|x| x),
        }
    }

    pub fn flat_or_map<X>(self, f: impl FnOnce(T) -> X) -> OneOf<X, U> {
        self.flat_or().map(f)
    }

    pub fn flat_or_map_or<X>(self, f: impl FnOnce(U) -> X) -> OneOf<T, X> {
        self.flat_or().map_or(f)
    }
}

impl<T: Default, U> Default for OneOf<T, U> {
    fn default() -> Self {
        OneOf::One(T::default())
    }
}

#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum OneOf3<T, U, X> {
    One(T),
    Mid(U),
    Other(X),
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

#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct ProgressParams {
    #[doc = "The progress token provided by the client or server."]
    pub token: ProgressToken,
    #[doc = "The progress data."]
    pub value: serde_json::Value,
}

impl<T: Default, U, X> Default for OneOf3<T, U, X> {
    fn default() -> Self {
        OneOf3::One(T::default())
    }
}

impl ReqId {
    pub fn ok_resp<T: serde::Serialize, R: Into<Option<T>>>(self, result: R) -> ResponseMessage {
        let result = result.into().map(|v| serde_json::to_value(v).unwrap());
        ResponseMessage {
            error: None,
            id: Some(self),
            jsonrpc: "2.0".to_string(),
            result,
        }
    }
}

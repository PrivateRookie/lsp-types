use crate::ResponseMessage;

use super::Integer;
use serde::{Deserialize, Serialize};

#[doc = "empty data"]
#[derive(Clone, PartialEq, Debug, Default)]
pub struct Empty {}

#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum OneOf<T, O> {
    This(T),
    Other(O),
}

impl<T, O> OneOf<T, O> {
    /// map `OneOf<T, O> -> OneOf<X, O>`
    pub fn map_t<X>(self, f: impl FnOnce(T) -> X) -> OneOf<X, O> {
        match self {
            OneOf::This(t) => OneOf::This(f(t)),
            OneOf::Other(u) => OneOf::Other(u),
        }
    }

    /// map `OneOf<T, O> -> OneOf<T, X>`
    pub fn map_o<X>(self, f: impl FnOnce(O) -> X) -> OneOf<T, X> {
        match self {
            OneOf::This(t) => OneOf::This(t),
            OneOf::Other(u) => OneOf::Other(f(u)),
        }
    }

    /// make `OneOf<T, O>` -> T
    pub fn unify(self, f: impl FnOnce(O) -> T) -> T {
        match self {
            OneOf::This(t) => t,
            OneOf::Other(u) => f(u),
        }
    }

    /// `OneOf<T, O> -> OneOf<O, T>`
    pub fn transpose(self) -> OneOf<O, T> {
        match self {
            OneOf::This(t) => OneOf::Other(t),
            OneOf::Other(u) => OneOf::This(u),
        }
    }
}

impl<T, O> OneOf<T, OneOf<T, O>> {
    /// `OneOf<T, OneOf<T, O>>` -> OneOf<T, O>`
    pub fn flat_o(self) -> OneOf<T, O> {
        match self {
            OneOf::This(t) => OneOf::This(t),
            OneOf::Other(u) => u.map_t(|x| x),
        }
    }

    /// apply flat_o then apply map_t
    pub fn flat_o_map_t<X>(self, f: impl FnOnce(T) -> X) -> OneOf<X, O> {
        self.flat_o().map_t(f)
    }

    /// apply flat_o then apply map_o
    pub fn flat_o_map_o<X>(self, f: impl FnOnce(O) -> X) -> OneOf<T, X> {
        self.flat_o().map_o(f)
    }
}

impl<T: Default, U> Default for OneOf<T, U> {
    fn default() -> Self {
        OneOf::This(T::default())
    }
}

#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum OneOf3<T, A, O> {
    This(T),
    Among(A),
    Other(O),
}

impl<T: Default, O> OneOf<T, O> {
    pub fn default_this() -> Self {
        Self::This(T::default())
    }

    pub fn opt_default_this() -> Option<Self> {
        Some(Self::default_this())
    }
}

impl<T, O: Default> OneOf<T, O> {
    pub fn default_other() -> Self {
        Self::Other(O::default())
    }

    pub fn opt_default_other() -> Option<Self> {
        Some(Self::default_other())
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
        OneOf3::This(T::default())
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

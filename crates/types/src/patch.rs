use crate::{FromNotice, FromReq, ResponseError};

use super::{Integer, NotificationMessage, RequestMessage, ResponseMessage};
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
    /// helper function to construct a ok response with request id
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

impl RequestMessage {
    pub fn with<C>(self, ctx: C) -> ReqWithContext<C> {
        ReqWithContext((self, ctx))
    }
}

pub struct ReqWithContext<C>((RequestMessage, C));

impl<C> ReqWithContext<C> {
    /// passing handler for current request
    pub fn then<R, F, I>(self, f: F) -> OneOf<I, Self>
    where
        C: Clone,
        R: FromReq,
        F: FnOnce(C, ReqId, R) -> I,
    {
        let (req, ctx) = self.0;
        R::from_req(req)
            .map_t(|(req_id, req)| f(ctx.clone(), req_id, req))
            .map_o(|req| Self((req, ctx)))
    }

    pub fn group<F, I>(self, f: F) -> OneOf<I, Self>
    where
        F: FnOnce(OneOf<I, Self>) -> OneOf<I, Self>,
    {
        f(OneOf::Other(self))
    }

    pub fn split(self) -> (RequestMessage, C) {
        self.0
    }
}

impl<I, C> OneOf<I, ReqWithContext<C>> {
    /// if previous handler does not match method field, pass alternative handler
    pub fn or_else<F, R>(self, f: F) -> OneOf<I, ReqWithContext<C>>
    where
        C: Clone,
        R: FromReq,
        F: FnOnce(C, ReqId, R) -> I,
    {
        self.map_o(|req| req.then(f)).flat_o()
    }

    pub fn group<F>(self, f: F) -> Self
    where
        F: FnOnce(Self) -> Self,
    {
        f(self)
    }
}

impl NotificationMessage {
    /// construct ctx with custom data, used for chaining handler functions
    pub fn with<C>(self, ctx: C) -> NoticeWithContext<C> {
        NoticeWithContext((self, ctx))
    }
}

/// wrap notification message with custom context
///
/// should be constructed by [NotificationMessage::with]
pub struct NoticeWithContext<C>((NotificationMessage, C));

impl<C> NoticeWithContext<C> {
    /// passing handler for current request
    pub fn then<N, F, I>(self, f: F) -> OneOf<I, Self>
    where
        C: Clone,
        N: FromNotice,
        F: FnOnce(C, N) -> I,
    {
        let (notice, ctx) = self.0;
        N::from_notice(notice)
            .map_t(|notice| f(ctx.clone(), notice))
            .map_o(|notice| Self((notice, ctx)))
    }

    pub fn split(self) -> (NotificationMessage, C) {
        self.0
    }
}

impl<I, C> OneOf<I, NoticeWithContext<C>> {
    /// if previous handler does not match method field, pass alternative handler
    pub fn or_else<F, N>(self, f: F) -> OneOf<I, NoticeWithContext<C>>
    where
        C: Clone,
        N: FromNotice,
        F: FnOnce(C, N) -> I,
    {
        self.map_o(|notice| notice.then(f)).flat_o()
    }
}

#[cfg(feature = "async")]
mod async_impl {
    use std::future::Future;

    use super::{FromNotice, FromReq, NoticeWithContext, ReqWithContext};
    use crate::{OneOf, ReqId};

    impl<T, O> OneOf<T, O> {
        /// async version of `unify`, allowing pass async handler function
        pub async fn async_unify<F: Future<Output = T>>(self, f: impl FnOnce(O) -> F) -> T {
            match self {
                OneOf::This(t) => t,
                OneOf::Other(u) => {
                    let t = f(u).await;
                    t
                }
            }
        }
    }

    impl<C> ReqWithContext<C> {
        /// async version of `then`, passing async handler
        pub async fn async_then<R, F, I, IF>(self, f: F) -> OneOf<I, Self>
        where
            C: Clone,
            R: FromReq,
            IF: Future<Output = I>,
            F: FnOnce(C, ReqId, R) -> IF,
        {
            let (req, ctx) = self.0;
            match R::from_req(req) {
                OneOf::This((req_id, req)) => {
                    let t = f(ctx.clone(), req_id, req).await;
                    OneOf::This(t)
                }
                OneOf::Other(req) => OneOf::Other(Self((req, ctx))),
            }
        }

        pub async fn async_group<F, I, IFut>(self, f: F) -> OneOf<I, Self>
        where
            IFut: Future<Output = OneOf<I, Self>>,
            F: FnOnce(OneOf<I, Self>) -> IFut,
        {
            f(OneOf::Other(self)).await
        }
    }

    impl<I, C> OneOf<I, ReqWithContext<C>> {
        /// async version of `or_else`, passing async handler
        pub async fn async_or_else<F, R, IF>(self, f: F) -> OneOf<I, ReqWithContext<C>>
        where
            C: Clone,
            R: FromReq,
            IF: Future<Output = I>,
            F: FnOnce(C, ReqId, R) -> IF,
        {
            let ret = match self {
                OneOf::This(t) => OneOf::This(t),
                OneOf::Other(o) => {
                    let o = o.async_then(f).await;
                    OneOf::Other(o)
                }
            };
            ret.flat_o()
        }

        pub async fn async_group<F, Fut>(self, f: F) -> Self
        where
            Fut: Future<Output = Self>,
            F: FnOnce(Self) -> Fut,
        {
            f(self).await
        }
    }

    impl<C> NoticeWithContext<C> {
        /// async version of `then`, passing async handler
        pub async fn async_then<N, F, R, IF>(self, f: F) -> OneOf<R, Self>
        where
            C: Clone,
            N: FromNotice,
            IF: Future<Output = R>,
            F: FnOnce(C, N) -> IF,
        {
            let (req, ctx) = self.0;
            match N::from_notice(req) {
                OneOf::This(req) => {
                    let t = f(ctx.clone(), req).await;
                    OneOf::This(t)
                }
                OneOf::Other(req) => OneOf::Other(Self((req, ctx))),
            }
        }
    }

    impl<I, C> OneOf<I, NoticeWithContext<C>> {
        /// async version of `or_else`, passing async handler
        pub async fn async_or_else<F, N, IF>(self, f: F) -> OneOf<I, NoticeWithContext<C>>
        where
            C: Clone,
            N: FromNotice,
            IF: Future<Output = I>,
            F: FnOnce(C, N) -> IF,
        {
            let ret = match self {
                OneOf::This(t) => OneOf::This(t),
                OneOf::Other(o) => {
                    let o = o.async_then(f).await;
                    OneOf::Other(o)
                }
            };
            ret.flat_o()
        }
    }
}

impl ResponseMessage {
    pub fn err_resp<I: Into<Option<ReqId>>>(id: I, error: ResponseError) -> Self {
        Self {
            id: id.into(),
            jsonrpc: "2.0".to_string(),
            error: Some(error),
            result: None,
        }
    }
}

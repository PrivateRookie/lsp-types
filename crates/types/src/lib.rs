mod part1;
mod part2;
mod part3;
mod patch;

use std::fmt::Debug;

pub use part1::*;
pub use part2::*;
pub use part3::*;
pub use patch::*;

pub trait Request {
    type Resp;
    const METHOD: &'static str;
}

pub trait _Handler {
    const METHOD: &'static str;
    type Params;
    type Result;
    type Context;

    fn register_cap(caps: &mut ServerCapabilities);

    fn handle(
        id: ReqId,
        params: Self::Params,
        ctx: &mut Self::Context,
    ) -> OneOf<Self::Result, ResponseError>;
}

#[test]
fn xx() {}

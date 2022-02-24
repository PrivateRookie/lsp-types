#[cfg(feature = "blocking")]
mod blocking;
#[cfg(feature = "async")]
mod non_blocking;

mod utils;

const BUF_SIZE: usize = 1024 * 4;

#[cfg(feature = "blocking")]
pub use blocking::*;

#[cfg(feature = "async")]
pub use non_blocking::*;

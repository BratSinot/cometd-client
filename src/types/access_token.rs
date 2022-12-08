#[cfg(feature = "basic")]
mod basic;
mod bearer;

#[cfg(feature = "basic")]
pub use basic::*;
pub use bearer::*;
use std::fmt::Debug;

pub trait AccessToken: Debug {
    fn get_authorization_header<'a>(&'a self) -> &[(&'static str, Box<str>)];
}

use crate::consts::msg;
use core::fmt::{Display, Formatter};

pub type Result<T, E = Error> = core::result::Result<T, E>;

#[derive(Clone, Debug, PartialEq)]
pub enum Error {
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "")
    }
}

impl core_error::Error for Error {}

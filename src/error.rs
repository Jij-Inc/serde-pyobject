use serde::ser;
use std::fmt::{self, Display};

#[derive(Debug)]
pub struct Error {}

impl ser::Error for Error {
    fn custom<T: Display>(_msg: T) -> Self {
        Self {}
    }
}

impl Display for Error {
    fn fmt(&self, _formatter: &mut fmt::Formatter) -> fmt::Result {
        Ok(())
    }
}

impl std::error::Error for Error {}

pub type Result<T> = ::std::result::Result<T, Error>;

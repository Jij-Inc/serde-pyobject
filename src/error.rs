use pyo3::PyErr;
use serde::ser;
use std::fmt::{self, Display};

#[derive(Debug)]
pub enum Error {
    Py(PyErr),
}

impl From<PyErr> for Error {
    fn from(err: PyErr) -> Self {
        Error::Py(err)
    }
}

impl ser::Error for Error {
    fn custom<T: Display>(_msg: T) -> Self {
        todo!()
    }
}

impl Display for Error {
    fn fmt(&self, _formatter: &mut fmt::Formatter) -> fmt::Result {
        Ok(())
    }
}

impl std::error::Error for Error {}

pub type Result<T> = ::std::result::Result<T, Error>;

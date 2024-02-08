use pyo3::{exceptions::PyRuntimeError, PyErr};
use serde::{de, ser};
use std::fmt::{self, Display};

/// New-type wrapper of `PyErr` to implement `serde::ser::Error`.
#[derive(Debug)]
pub struct Error(pub PyErr);

impl From<PyErr> for Error {
    fn from(err: PyErr) -> Self {
        Error(err)
    }
}

impl From<Error> for PyErr {
    fn from(err: Error) -> Self {
        err.0
    }
}

impl ser::Error for Error {
    fn custom<T: Display>(msg: T) -> Self {
        Error(PyRuntimeError::new_err(msg.to_string()))
    }
}

impl de::Error for Error {
    fn custom<T: Display>(msg: T) -> Self {
        Error(PyRuntimeError::new_err(msg.to_string()))
    }
}

impl Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(formatter)
    }
}

impl std::error::Error for Error {}

pub type Result<T> = ::std::result::Result<T, Error>;

mod de;
mod error;
mod pydict;
mod ser;

/// Re-export of `pyo3` crate.
pub use pyo3;

pub use de::from_pyobject;
pub use error::Error;
pub use ser::to_pyobject;

#[cfg_attr(doc, doc = include_str!("../README.md"))]
mod readme {}

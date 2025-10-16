//! PyO3's PyAny as a serde data format
//!
//! This crate provides a mapping from [serde data model](https://serde.rs/data-model.html)
//! to Python objects.
//!

mod de;
mod error;
mod pylit;
mod ser;

/// Re-export of `pyo3` crate.
pub use pyo3;

pub use de::from_pyobject;
pub use error::Error;
pub use ser::to_pyobject;

#[cfg_attr(doc, doc = include_str!("../README.md"))]
mod readme {}

#[cfg(any(feature = "dataclass_support", feature = "pydantic_support"))]
mod py_module_cache;

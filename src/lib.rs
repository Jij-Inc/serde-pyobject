//! PyO3's PyAny as a serde data format
//!
//! This crate provides a mapping from [serde data model](https://serde.rs/data-model.html)
//! to Python objects.
//!

mod dataclass;
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

mod py_module_cache;

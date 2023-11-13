mod error;
mod ser;

pub use error::Error;
pub use ser::to_pyobject;

#[cfg_attr(doc, doc = include_str!("../README.md"))]
mod readme {}

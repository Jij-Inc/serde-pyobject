mod error;
mod serializer;

pub use error::Error;
pub use serializer::as_pyobject;

#[cfg_attr(doc, doc = include_str!("../README.md"))]
mod readme {}

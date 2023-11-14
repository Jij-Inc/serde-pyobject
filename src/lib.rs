mod de;
mod error;
mod ser;

pub use de::from_pyobject;
pub use error::Error;
pub use ser::to_pyobject;

#[cfg_attr(doc, doc = include_str!("../README.md"))]
mod readme {}

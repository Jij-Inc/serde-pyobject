use crate::error::{Error, Result};
use pyo3::types::{PyAny, PyLong};
use serde::{
    de::{self, Visitor},
    forward_to_deserialize_any, Deserialize,
};

pub fn from_pyobject<'py, 'de, T: Deserialize<'de>>(any: &'py PyAny) -> Result<T> {
    T::deserialize(Deserializer(any))
}

struct Deserializer<'py>(&'py PyAny);

impl<'de, 'py> de::Deserializer<'de> for Deserializer<'py> {
    type Error = Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        if self.0.is_instance_of::<PyLong>() {
            let i: i64 = self.0.extract()?;
            return visitor.visit_i64(i);
        }
        todo!()
    }

    forward_to_deserialize_any! {
        bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string
        bytes byte_buf option unit unit_struct newtype_struct seq tuple
        tuple_struct map struct enum identifier ignored_any
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use pyo3::prelude::*;

    #[test]
    fn i32_from_pyobject() {
        Python::with_gil(|py| {
            let py_int: Py<PyAny> = 42.into_py(py);
            let i32: i32 = from_pyobject(py_int.into_ref(py)).unwrap();
            assert_eq!(i32, 42);
        });
    }
}

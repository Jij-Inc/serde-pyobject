use crate::error::{Error, Result};
use pyo3::types::*;
use serde::{
    de::{self, MapAccess, Visitor},
    forward_to_deserialize_any, Deserialize,
};

pub fn from_pyobject<'py, 'de, T: Deserialize<'de>>(any: &'py PyAny) -> Result<T> {
    T::deserialize(PyAnyDeserializer(any))
}

struct PyAnyDeserializer<'py>(&'py PyAny);

impl<'de, 'py> de::Deserializer<'de> for PyAnyDeserializer<'py> {
    type Error = Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        if self.0.is_instance_of::<PyDict>() {
            return visitor.visit_map(MapDeserializer::new(self.0.extract()?));
        }
        if self.0.is_instance_of::<PyString>() {
            return visitor.visit_str(self.0.extract()?);
        }
        if self.0.is_instance_of::<PyLong>() {
            return visitor.visit_i64(self.0.extract()?);
        }
        unreachable!("Unsupported type: {}", self.0.get_type());
    }

    forward_to_deserialize_any! {
        bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string
        bytes byte_buf option unit unit_struct newtype_struct seq tuple
        tuple_struct map struct enum identifier ignored_any
    }
}

struct MapDeserializer<'py> {
    keys: Vec<&'py PyAny>,
    values: Vec<&'py PyAny>,
}

impl<'py> MapDeserializer<'py> {
    fn new(dict: &'py PyDict) -> Self {
        let mut keys = Vec::new();
        let mut values = Vec::new();
        for (key, value) in dict.iter() {
            keys.push(key);
            values.push(value);
        }
        Self { keys, values }
    }
}

impl<'de, 'py> MapAccess<'de> for MapDeserializer<'py> {
    type Error = Error;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>>
    where
        K: de::DeserializeSeed<'de>,
    {
        if let Some(key) = self.keys.pop() {
            let key = seed.deserialize(PyAnyDeserializer(key))?;
            Ok(Some(key))
        } else {
            Ok(None)
        }
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value>
    where
        V: de::DeserializeSeed<'de>,
    {
        if let Some(value) = self.values.pop() {
            let value = seed.deserialize(PyAnyDeserializer(value))?;
            Ok(value)
        } else {
            unreachable!()
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::pydict;
    use pyo3::prelude::*;
    use std::collections::BTreeMap;

    #[test]
    fn i32_from_pyobject() {
        Python::with_gil(|py| {
            let py_int: Py<PyAny> = 42.into_py(py);
            let i32: i32 = from_pyobject(py_int.into_ref(py)).unwrap();
            assert_eq!(i32, 42);
        });
    }

    #[test]
    fn btreemap_from_pydict() {
        Python::with_gil(|py| {
            let dict = pydict! {
                "a" => "hom",
                "b" => "test"
            }
            .unwrap();
            let map: BTreeMap<String, String> = from_pyobject(dict.into_ref(py)).unwrap();
            assert_eq!(map.get("a"), Some(&"hom".to_string()));
            assert_eq!(map.get("b"), Some(&"test".to_string()));
        });
    }

    #[derive(Debug, PartialEq, Deserialize)]
    struct A {
        a: i32,
        b: String,
    }

    #[test]
    fn struct_from_pydict() {
        Python::with_gil(|py| {
            let dict = pydict! {
                "a" => 1,
                "b" => "test"
            }
            .unwrap();
            let a: A = from_pyobject(dict.into_ref(py)).unwrap();
            assert_eq!(
                a,
                A {
                    a: 1,
                    b: "test".to_string()
                }
            );
        });
    }

    #[test]
    fn struct_from_nested_pydict() {
        Python::with_gil(|py| {
            let dict = pydict! {
                "A" => pydict! {
                    "a" => 1,
                    "b" => "test"
                }
                .unwrap()
            }
            .unwrap();
            let a: A = from_pyobject(dict.into_ref(py)).unwrap();
            assert_eq!(
                a,
                A {
                    a: 1,
                    b: "test".to_string()
                }
            );
        });
    }
}

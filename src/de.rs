use crate::error::{Error, Result};
use pyo3::types::*;
use serde::{
    de::{self, MapAccess, SeqAccess, Visitor},
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
        if self.0.is_instance_of::<PyList>() {
            return visitor.visit_seq(SeqDeserializer::new(self.0.extract()?));
        }
        if self.0.is_instance_of::<PyString>() {
            return visitor.visit_str(self.0.extract()?);
        }
        if self.0.is_instance_of::<PyLong>() {
            return visitor.visit_i64(self.0.extract()?);
        }
        unreachable!("Unsupported type: {}", self.0.get_type());
    }

    fn deserialize_struct<V: de::Visitor<'de>>(
        self,
        name: &'static str,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value> {
        // Nested dict `{ "A": { "a": 1, "b": 2 } }` is deserialized as `A { a: 1, b: 2 }`
        if self.0.is_instance_of::<PyDict>() {
            let dict: &PyDict = self.0.extract()?;
            if let Some(inner) = dict.get_item(name)? {
                if let Ok(inner) = inner.extract() {
                    return visitor.visit_map(MapDeserializer::new(inner));
                }
            }
        }
        // Default to `any` case
        self.deserialize_any(visitor)
    }

    fn deserialize_newtype_struct<V: de::Visitor<'de>>(
        self,
        name: &'static str,
        visitor: V,
    ) -> Result<V::Value> {
        // Dict `{ "A": 1 }` is deserialized as `A(1)`
        if self.0.is_instance_of::<PyDict>() {
            let dict: &PyDict = self.0.extract()?;
            if let Some(inner) = dict.get_item(name)? {
                // Visitor of `#[derive(Deserialize)] struct A(u8);` requires tuple struct,
                // and thus use 1-element "tuple" here
                return visitor.visit_seq(SeqDeserializer {
                    seq_reversed: vec![inner],
                });
            }
        }
        // Default to `any` case
        self.deserialize_any(visitor)
    }

    forward_to_deserialize_any! {
        bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string
        bytes byte_buf option unit unit_struct seq tuple tuple_struct
        map enum identifier ignored_any
    }
}

struct SeqDeserializer<'py> {
    seq_reversed: Vec<&'py PyAny>,
}

impl<'py> SeqDeserializer<'py> {
    fn new(list: &'py PyList) -> Self {
        let mut seq_reversed = Vec::new();
        for item in list.iter().rev() {
            seq_reversed.push(item);
        }
        Self { seq_reversed }
    }
}

impl<'de, 'py> SeqAccess<'de> for SeqDeserializer<'py> {
    type Error = Error;
    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>>
    where
        T: de::DeserializeSeed<'de>,
    {
        self.seq_reversed.pop().map_or(Ok(None), |value| {
            let value = seed.deserialize(PyAnyDeserializer(value))?;
            Ok(Some(value))
        })
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

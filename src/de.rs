use crate::error::{Error, Result};
use pyo3::{types::*, Bound};
use serde::{
    de::{self, value::StrDeserializer, MapAccess, SeqAccess, Visitor},
    forward_to_deserialize_any, Deserialize, Deserializer,
};

/// Deserialize a Python object into Rust type `T: Deserialize`.
///
/// # Examples
///
/// ## primitive
///
/// ```
/// use pyo3::{Python, Py, PyAny, IntoPy};
/// use serde_pyobject::from_pyobject;
///
/// Python::with_gil(|py| {
///     // integer
///     let any: Py<PyAny> = 42.into_py(py);
///     let i: i32 = from_pyobject(any.into_bound(py)).unwrap();
///     assert_eq!(i, 42);
///
///     // float
///     let any: Py<PyAny> = (0.1).into_py(py);
///     let x: f32 = from_pyobject(any.into_bound(py)).unwrap();
///     assert_eq!(x, 0.1);
///
///     // bool
///     let any: Py<PyAny> = true.into_py(py);
///     let x: bool = from_pyobject(any.into_bound(py)).unwrap();
///     assert_eq!(x, true);
/// });
/// ```
///
/// ## option
///
/// ```
/// use pyo3::{Python, Py, PyAny, IntoPy};
/// use serde_pyobject::from_pyobject;
///
/// Python::with_gil(|py| {
///     let none = py.None();
///     let option: Option<i32> = from_pyobject(none.into_bound(py)).unwrap();
///     assert_eq!(option, None);
///
///     let py_int: Py<PyAny> = 42.into_py(py);
///     let i: Option<i32> = from_pyobject(py_int.into_bound(py)).unwrap();
///     assert_eq!(i, Some(42));
/// })
/// ```
///
/// ## unit
///
/// ```
/// use pyo3::{Python, types::PyTuple};
/// use serde_pyobject::from_pyobject;
///
/// Python::with_gil(|py| {
///     let py_unit = PyTuple::empty(py);
///     let unit: () = from_pyobject(py_unit).unwrap();
///     assert_eq!(unit, ());
/// })
/// ```
///
/// ## unit_struct
///
/// ```
/// use serde::Deserialize;
/// use pyo3::{Python, types::PyTuple};
/// use serde_pyobject::from_pyobject;
///
/// #[derive(Debug, PartialEq, Deserialize)]
/// struct UnitStruct;
///
/// Python::with_gil(|py| {
///     let py_unit = PyTuple::empty(py);
///     let unit: UnitStruct = from_pyobject(py_unit).unwrap();
///     assert_eq!(unit, UnitStruct);
/// })
/// ```
///
/// ## unit variant
///
/// ```
/// use serde::Deserialize;
/// use pyo3::{Python, types::PyString};
/// use serde_pyobject::from_pyobject;
///
/// #[derive(Debug, PartialEq, Deserialize)]
/// enum E {
///     A,
///     B,
/// }
///
/// Python::with_gil(|py| {
///     let any = PyString::new_bound(py, "A");
///     let out: E = from_pyobject(any).unwrap();
///     assert_eq!(out, E::A);
/// })
/// ```
///
/// ## newtype struct
///
/// ```
/// use serde::Deserialize;
/// use pyo3::{Python, Bound, PyAny, IntoPy};
/// use serde_pyobject::from_pyobject;
///
/// #[derive(Debug, PartialEq, Deserialize)]
/// struct NewTypeStruct(u8);
///
/// Python::with_gil(|py| {
///     let any: Bound<PyAny> = 1_u32.into_py(py).into_bound(py);
///     let obj: NewTypeStruct = from_pyobject(any).unwrap();
///     assert_eq!(obj, NewTypeStruct(1));
/// });
/// ```
///
/// ## newtype variant
///
/// ```
/// use serde::Deserialize;
/// use pyo3::Python;
/// use serde_pyobject::{from_pyobject, pydict};
///
/// #[derive(Debug, PartialEq, Deserialize)]
/// enum NewTypeVariant {
///     N(u8),
/// }
///
/// Python::with_gil(|py| {
///     let dict = pydict! { py, "N" => 41 }.unwrap();
///     let obj: NewTypeVariant = from_pyobject(dict).unwrap();
///     assert_eq!(obj, NewTypeVariant::N(41));
/// });
/// ```
///
/// ## seq
///
/// ```
/// use pyo3::Python;
/// use serde_pyobject::{from_pyobject, pylist};
///
/// Python::with_gil(|py| {
///     let list = pylist![py; 1, 2, 3].unwrap();
///     let seq: Vec<i32> = from_pyobject(list).unwrap();
///     assert_eq!(seq, vec![1, 2, 3]);
/// });
/// ```
///
/// ## tuple
///
/// ```
/// use pyo3::{Python, types::PyTuple};
/// use serde_pyobject::from_pyobject;
///
/// Python::with_gil(|py| {
///     let tuple = PyTuple::new_bound(py, &[1, 2, 3]);
///     let tuple: (i32, i32, i32) = from_pyobject(tuple).unwrap();
///     assert_eq!(tuple, (1, 2, 3));
/// });
/// ```
///
/// ## tuple struct
///
/// ```
/// use serde::Deserialize;
/// use pyo3::{Python, IntoPy, types::PyTuple};
/// use serde_pyobject::from_pyobject;
///
/// #[derive(Debug, PartialEq, Deserialize)]
/// struct T(u8, String);
///
/// Python::with_gil(|py| {
///     let tuple = PyTuple::new_bound(py, &[1_u32.into_py(py), "test".into_py(py)]);
///     let obj: T = from_pyobject(tuple).unwrap();
///     assert_eq!(obj, T(1, "test".to_string()));
/// });
/// ```
///
/// ## tuple variant
///
/// ```
/// use serde::Deserialize;
/// use pyo3::Python;
/// use serde_pyobject::{from_pyobject, pydict};
///
/// #[derive(Debug, PartialEq, Deserialize)]
/// enum TupleVariant {
///     T(u8, u8),
/// }
///
/// Python::with_gil(|py| {
///     let dict = pydict! { py, "T" => (1, 2) }.unwrap();
///     let obj: TupleVariant = from_pyobject(dict).unwrap();
///     assert_eq!(obj, TupleVariant::T(1, 2));
/// });
/// ```
///
/// ## map
///
/// ```
/// use pyo3::Python;
/// use serde_pyobject::{from_pyobject, pydict};
/// use std::collections::BTreeMap;
///
/// Python::with_gil(|py| {
///     let dict = pydict! { py,
///         "a" => "hom",
///         "b" => "test"
///     }
///     .unwrap();
///     let map: BTreeMap<String, String> = from_pyobject(dict).unwrap();
///     assert_eq!(map.get("a"), Some(&"hom".to_string()));
///     assert_eq!(map.get("b"), Some(&"test".to_string()));
/// });
/// ```
///
/// ## struct
///
/// ```
/// use serde::Deserialize;
/// use pyo3::Python;
/// use serde_pyobject::{from_pyobject, pydict};
///
/// #[derive(Debug, PartialEq, Deserialize)]
/// struct A {
///     a: i32,
///     b: String,
/// }
///
/// Python::with_gil(|py| {
///     let dict = pydict! {
///         "a" => 1,
///         "b" => "test"
///     }
///     .unwrap();
///     let a: A = from_pyobject(dict.into_bound(py)).unwrap();
///     assert_eq!(
///         a,
///         A {
///             a: 1,
///             b: "test".to_string()
///         }
///     );
/// });
///
/// Python::with_gil(|py| {
///     let dict = pydict! {
///         "A" => pydict! {
///             "a" => 1,
///             "b" => "test"
///         }
///         .unwrap()
///     }
///     .unwrap();
///     let a: A = from_pyobject(dict.into_bound(py)).unwrap();
///     assert_eq!(
///         a,
///         A {
///             a: 1,
///             b: "test".to_string()
///         }
///     );
/// });
/// ```
///
/// ## struct variant
///
/// ```
/// use serde::Deserialize;
/// use pyo3::Python;
/// use serde_pyobject::{from_pyobject, pydict};
///
/// #[derive(Debug, PartialEq, Deserialize)]
/// enum StructVariant {
///     S { r: u8, g: u8, b: u8 },
/// }
///
/// Python::with_gil(|py| {
///     let dict = pydict! {
///         py,
///         "S" => pydict! {
///             "r" => 1,
///             "g" => 2,
///             "b" => 3
///         }.unwrap()
///     }
///     .unwrap();
///     let obj: StructVariant = from_pyobject(dict).unwrap();
///     assert_eq!(obj, StructVariant::S { r: 1, g: 2, b: 3 });
/// });
/// ```
pub fn from_pyobject<'py, 'de, T: Deserialize<'de>, Any>(any: Bound<'py, Any>) -> Result<T> {
    let any = any.into_any();
    T::deserialize(PyAnyDeserializer(any))
}

struct PyAnyDeserializer<'py>(Bound<'py, PyAny>);

impl<'de> de::Deserializer<'de> for PyAnyDeserializer<'_> {
    type Error = Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        if self.0.is_instance_of::<PyDict>() {
            return visitor.visit_map(MapDeserializer::new(self.0.downcast()?));
        }
        if self.0.is_instance_of::<PyList>() {
            return visitor.visit_seq(SeqDeserializer::from_list(self.0.downcast()?));
        }
        if self.0.is_instance_of::<PyTuple>() {
            return visitor.visit_seq(SeqDeserializer::from_tuple(self.0.downcast()?));
        }
        if self.0.is_instance_of::<PyString>() {
            return visitor.visit_str(&self.0.extract::<String>()?);
        }
        if self.0.is_instance_of::<PyBool>() {
            // must be match before PyLong
            return visitor.visit_bool(self.0.extract()?);
        }
        if self.0.is_instance_of::<PyInt>() {
            return visitor.visit_i64(self.0.extract()?);
        }
        if self.0.is_instance_of::<PyFloat>() {
            return visitor.visit_f64(self.0.extract()?);
        }
        #[cfg(feature = "dataclass_support")]
        if crate::py_module_cache::is_dataclass(self.0.py(), &self.0)? {
            // Use dataclasses.asdict(obj) to get the dict representtion of the object
            let dataclasses = PyModule::import(self.0.py(), "dataclasses")?;
            let asdict = dataclasses.getattr("asdict")?;
            let dict = asdict.call1((self.0,))?;
            return visitor.visit_map(MapDeserializer::new(dict.downcast()?));
        }
        #[cfg(feature = "pydantic_support")]
        if crate::py_module_cache::is_pydantic_base_model(self.0.py(), &self.0)? {
            // Use pydantic.BaseModel#model_dump() to get the dict representation of the object
            let model_dump = self.0.getattr("model_dump")?;
            let dict = model_dump.call0()?;
            return visitor.visit_map(MapDeserializer::new(dict.downcast()?));
        }
        if self.0.hasattr("__dict__")? {
            return visitor.visit_map(MapDeserializer::new(
                self.0.getattr("__dict__")?.downcast()?,
            ));
        }
        if self.0.is_none() {
            return visitor.visit_none();
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
            let dict: &Bound<PyDict> = self.0.downcast()?;
            if let Some(inner) = dict.get_item(name)? {
                if let Ok(inner) = inner.downcast() {
                    return visitor.visit_map(MapDeserializer::new(inner));
                }
            }
        }
        // Default to `any` case
        self.deserialize_any(visitor)
    }

    fn deserialize_newtype_struct<V: de::Visitor<'de>>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value> {
        visitor.visit_seq(SeqDeserializer {
            seq_reversed: vec![self.0],
        })
    }

    fn deserialize_option<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        if self.0.is_none() {
            visitor.visit_none()
        } else {
            visitor.visit_some(self)
        }
    }

    fn deserialize_unit<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        if self.0.is(&PyTuple::empty(self.0.py())) {
            visitor.visit_unit()
        } else {
            self.deserialize_any(visitor)
        }
    }

    fn deserialize_unit_struct<V: de::Visitor<'de>>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value> {
        if self.0.is(&PyTuple::empty(self.0.py())) {
            visitor.visit_unit()
        } else {
            self.deserialize_any(visitor)
        }
    }

    fn deserialize_enum<V: de::Visitor<'de>>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value> {
        if self.0.is_instance_of::<PyString>() {
            let variant: String = self.0.extract()?;
            let py = self.0.py();
            let none = py.None().into_bound(py);
            return visitor.visit_enum(EnumDeserializer {
                variant: &variant,
                inner: none,
            });
        }
        if self.0.is_instance_of::<PyDict>() {
            let dict: &Bound<PyDict> = self.0.downcast()?;
            if dict.len() == 1 {
                let key = dict.keys().get_item(0).unwrap();
                let value = dict.values().get_item(0).unwrap();
                if key.is_instance_of::<PyString>() {
                    let variant: String = key.extract()?;
                    return visitor.visit_enum(EnumDeserializer {
                        variant: &variant,
                        inner: value,
                    });
                }
            }
        }
        self.deserialize_any(visitor)
    }

    fn deserialize_tuple_struct<V: de::Visitor<'de>>(
        self,
        name: &'static str,
        _len: usize,
        visitor: V,
    ) -> Result<V::Value> {
        if self.0.is_instance_of::<PyDict>() {
            let dict: &Bound<PyDict> = self.0.downcast()?;
            if let Some(value) = dict.get_item(name)? {
                if value.is_instance_of::<PyTuple>() {
                    let tuple: &Bound<PyTuple> = value.downcast()?;
                    return visitor.visit_seq(SeqDeserializer::from_tuple(tuple));
                }
            }
        }
        self.deserialize_any(visitor)
    }

    forward_to_deserialize_any! {
        bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string
        bytes byte_buf seq tuple
        map identifier ignored_any
    }
}

struct SeqDeserializer<'py> {
    seq_reversed: Vec<Bound<'py, PyAny>>,
}

impl<'py> SeqDeserializer<'py> {
    fn from_list(list: &Bound<'py, PyList>) -> Self {
        let mut seq_reversed = Vec::new();
        for item in list.iter().rev() {
            seq_reversed.push(item);
        }
        Self { seq_reversed }
    }

    fn from_tuple(tuple: &Bound<'py, PyTuple>) -> Self {
        let mut seq_reversed = Vec::new();
        for item in tuple.iter().rev() {
            seq_reversed.push(item);
        }
        Self { seq_reversed }
    }
}

impl<'de> SeqAccess<'de> for SeqDeserializer<'_> {
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
    keys: Vec<Bound<'py, PyAny>>,
    values: Vec<Bound<'py, PyAny>>,
}

impl<'py> MapDeserializer<'py> {
    fn new(dict: &Bound<'py, PyDict>) -> Self {
        let mut keys = Vec::new();
        let mut values = Vec::new();
        for (key, value) in dict.iter() {
            keys.push(key);
            values.push(value);
        }
        Self { keys, values }
    }
}

impl<'de> MapAccess<'de> for MapDeserializer<'_> {
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

// this lifetime is technically no longer 'py
struct EnumDeserializer<'py> {
    variant: &'py str,
    inner: Bound<'py, PyAny>,
}

impl<'de> de::EnumAccess<'de> for EnumDeserializer<'_> {
    type Error = Error;
    type Variant = Self;

    fn variant_seed<V>(self, seed: V) -> Result<(V::Value, Self::Variant)>
    where
        V: de::DeserializeSeed<'de>,
    {
        Ok((
            seed.deserialize(StrDeserializer::<Error>::new(self.variant))?,
            self,
        ))
    }
}

impl<'de> de::VariantAccess<'de> for EnumDeserializer<'_> {
    type Error = Error;

    fn unit_variant(self) -> Result<()> {
        Ok(())
    }

    fn newtype_variant_seed<T>(self, seed: T) -> Result<T::Value>
    where
        T: de::DeserializeSeed<'de>,
    {
        seed.deserialize(PyAnyDeserializer(self.inner))
    }

    fn tuple_variant<V>(self, _len: usize, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        PyAnyDeserializer(self.inner).deserialize_seq(visitor)
    }

    fn struct_variant<V>(self, _fields: &'static [&'static str], visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        PyAnyDeserializer(self.inner).deserialize_map(visitor)
    }
}

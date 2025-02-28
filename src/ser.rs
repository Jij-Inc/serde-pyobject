use crate::error::{Error, Result};
use pyo3::{prelude::*, types::*, IntoPyObjectExt};
use serde::{ser, Serialize};

/// Serialize `T: Serialize` into a [`pyo3::PyAny`] value.
///
/// # Examples
///
/// ## string
///
/// ```
/// use pyo3::{Python, types::{PyString, PyAnyMethods}};
/// use serde_pyobject::to_pyobject;
///
/// Python::with_gil(|py| {
///     // char
///     let obj = to_pyobject(py, &'a').unwrap();
///     assert!(obj.is_exact_instance_of::<PyString>());
///     // &str
///     let obj = to_pyobject(py, "test").unwrap();
///     assert!(obj.is_exact_instance_of::<PyString>());
/// });
/// ```
///
/// ## integer
///
/// ```
/// use pyo3::{Python, types::{PyLong, PyAnyMethods}};
/// use serde_pyobject::to_pyobject;
///
/// Python::with_gil(|py| {
///     let obj = to_pyobject(py, &1_u16).unwrap();
///     assert!(obj.is_exact_instance_of::<PyLong>());
///
///     let obj = to_pyobject(py, &1_i64).unwrap();
///     assert!(obj.is_exact_instance_of::<PyLong>());
///
///     let obj = to_pyobject(py, &1_i64).unwrap();
///     assert!(obj.is_exact_instance_of::<PyLong>());
/// });
/// ```
///
/// ## float
///
/// ```
/// use pyo3::{Python, types::{PyFloat, PyAnyMethods}};
/// use serde_pyobject::to_pyobject;
///
/// Python::with_gil(|py| {
///     let obj = to_pyobject(py, &3.1_f32).unwrap();
///     assert!(obj.is_exact_instance_of::<PyFloat>());
///
///     let obj = to_pyobject(py, &-3.1_f64).unwrap();
///     assert!(obj.is_exact_instance_of::<PyFloat>());
/// });
/// ```
///
/// ## option
///
/// Rust `None` is serialized to Python `None`, and `Some(value)` is serialized as `value` is serialized
///
/// ```
/// use pyo3::{Python, types::{PyLong, PyAnyMethods}};
/// use serde_pyobject::to_pyobject;
///
/// Python::with_gil(|py| {
///     let obj = to_pyobject(py, &Option::<i32>::None).unwrap();
///     assert!(obj.is(&py.None()));
///
///     let obj = to_pyobject(py, &Some(1_i64)).unwrap();
///     assert!(obj.is_exact_instance_of::<PyLong>());
/// });
/// ```
///
/// ## unit
///
/// Rust's `()` is serialized to Python's `()`
///
/// ```
/// use pyo3::{Python, types::{PyTuple, PyAnyMethods}};
/// use serde_pyobject::to_pyobject;
///
/// Python::with_gil(|py| {
///     let obj = to_pyobject(py, &()).unwrap();
///     assert!(obj.is(&PyTuple::empty_bound(py)));
/// });
/// ```
///
/// ## unit_struct
///
/// `Unit` is serialized as an empty tuple `()`
///
/// ```
/// use serde::Serialize;
/// use pyo3::{Python, types::{PyTuple, PyAnyMethods}};
/// use serde_pyobject::{to_pyobject, pydict};
///
/// #[derive(Serialize)]
/// struct UnitStruct;
///
/// Python::with_gil(|py| {
///     let obj = to_pyobject(py, &UnitStruct {}).unwrap();
///     assert!(obj.eq(PyTuple::empty_bound(py)).unwrap());
/// });
/// ```
///
/// ## unit_variant
///
/// ```
/// use serde::Serialize;
/// use pyo3::{Python, types::{PyTuple, PyAnyMethods}};
/// use serde_pyobject::{to_pyobject, pydict};
///
/// #[derive(Serialize)]
/// enum UnitVariant {
///     A,
///     B,
/// }
///
/// Python::with_gil(|py| {
///     let obj = to_pyobject(py, &UnitVariant::A).unwrap();
///     assert!(obj.eq("A").unwrap());
///     let obj = to_pyobject(py, &UnitVariant::B).unwrap();
///     assert!(obj.eq("B").unwrap());
/// });
/// ```
///
/// ## newtype_struct
///
/// ```
/// use serde::Serialize;
/// use pyo3::{Python, types::{PyLong, PyAnyMethods}};
/// use serde_pyobject::to_pyobject;
///
/// #[derive(Serialize)]
/// struct NewtypeStruct(u8);
///
/// Python::with_gil(|py| {
///     let obj = to_pyobject(py, &NewtypeStruct(10)).unwrap();
///     assert!(obj.is_exact_instance_of::<PyLong>());
///     assert!(obj.eq(10).unwrap());
/// });
/// ```
///
/// ## newtype_variant
///
/// ```
/// use serde::Serialize;
/// use pyo3::{Python, types::PyAnyMethods};
/// use serde_pyobject::{to_pyobject, pydict};
///
/// #[derive(Serialize)]
/// enum NewtypeVariant {
///     N(u8),
/// }
///
/// Python::with_gil(|py| {
///     let obj = to_pyobject(py, &NewtypeVariant::N(3)).unwrap();
///     assert!(obj.eq(pydict! { "N" => 3 }.unwrap()).unwrap());
/// });
/// ```
///
/// ## seq
///
/// ```
/// use pyo3::{Python, types::PyAnyMethods};
/// use serde_pyobject::{to_pyobject, pylist};
///
/// Python::with_gil(|py| {
///     let obj = to_pyobject(py, &vec![1, 2, 3]).unwrap();
///     assert!(obj.eq(pylist![py; 1, 2, 3].unwrap()).unwrap());
/// });
/// ```
///
/// ## tuple
///
/// ```
/// use pyo3::{IntoPy, Python, types::{PyTuple, PyAnyMethods}};
/// use serde_pyobject::to_pyobject;
///
/// Python::with_gil(|py| {
///     let obj = to_pyobject(py, &(3, "test")).unwrap();
///     assert!(obj.eq(PyTuple::new_bound(py, [3.into_py(py), "test".into_py(py)])).unwrap());
/// });
/// ```
///
/// ## tuple struct
///
/// ```
/// use pyo3::{Python, types::{PyTuple, PyAnyMethods}};
/// use serde::Serialize;
/// use serde_pyobject::to_pyobject;
///
/// #[derive(Serialize)]
/// struct TupleStruct(u8, u8, u8);
///
/// Python::with_gil(|py| {
///     let obj = to_pyobject(py, &TupleStruct(1, 2, 3)).unwrap();
///     assert!(obj.eq(PyTuple::new_bound(py, [1, 2, 3])).unwrap());
/// });
/// ```
///
/// ## tuple variant
///
/// ```
/// use pyo3::{Python, types::PyAnyMethods};
/// use serde::Serialize;
/// use serde_pyobject::{to_pyobject, pydict};
///
/// #[derive(Serialize)]
/// enum TupleVariant {
///     T(u8, u8),
/// }
///
/// Python::with_gil(|py| {
///     let obj = to_pyobject(py, &TupleVariant::T(1, 2)).unwrap();
///     assert!(obj.eq(pydict!{ "T" => (1, 2) }.unwrap()).unwrap());
/// });
/// ```
///
/// ## map
///
/// ```
/// use pyo3::{Python, types::PyAnyMethods};
/// use serde_pyobject::{to_pyobject, pydict};
/// use maplit::hashmap;
///
/// Python::with_gil(|py| {
///     let obj = to_pyobject(py, &hashmap! {
///         "a".to_owned() => 1_u8,
///         "b".to_owned() => 2,
///         "c".to_owned() => 3
///     }).unwrap();
///     assert!(obj.eq(pydict! {
///         "a" => 1,
///         "b" => 2,
///         "c" => 3
///     }.unwrap()).unwrap());
/// });
/// ```
///
/// ## struct
///
/// ```
/// use serde::Serialize;
/// use pyo3::{IntoPy, Python, types::{PyTuple, PyAnyMethods}};
/// use serde_pyobject::{to_pyobject, pydict};
///
/// #[derive(Serialize)]
/// struct Struct {
///     a: i32,
///     b: String,
/// }
///
/// Python::with_gil(|py| {
///     let obj = to_pyobject(py, &Struct { a: 32, b: "test".to_string() }).unwrap();
///     assert!(obj.eq(pydict!{ "a" => 32, "b" => "test" }.unwrap()).unwrap());
/// });
/// ```
///
/// ## struct variant
///
/// ```
/// use serde::Serialize;
/// use pyo3::{Python, types::PyAnyMethods};
/// use serde_pyobject::{to_pyobject, pydict};
///
/// #[derive(Serialize)]
/// enum StructVariant {
///     S { r: u8, g: u8, b: u8 },
/// }
///
/// Python::with_gil(|py| {
///     let obj = to_pyobject(py, &StructVariant::S { r: 1, g: 2, b: 3 }).unwrap();
///     assert!(
///         obj.eq(
///             pydict! {
///                 "S" => pydict! {
///                     "r" => 1,
///                     "g" => 2,
///                     "b" => 3
///                 }.unwrap()
///             }.unwrap()
///         ).unwrap()
///     );
/// });
/// ```
pub fn to_pyobject<'py, T>(py: Python<'py>, value: &T) -> Result<Bound<'py, PyAny>>
where
    T: Serialize + ?Sized,
{
    let serializer = PyAnySerializer { py };
    value.serialize(serializer)
}

pub struct PyAnySerializer<'py> {
    py: Python<'py>,
}

macro_rules! serialize_impl {
    ($f:ident, $t:ty) => {
        fn $f(self, v: $t) -> Result<Self::Ok> {
            Ok(v.into_bound_py_any(self.py)?)
        }
    };
}

impl<'py> ser::Serializer for PyAnySerializer<'py> {
    type Ok = Bound<'py, PyAny>;

    type Error = Error;

    type SerializeSeq = Seq<'py>;
    type SerializeTuple = Seq<'py>;
    type SerializeTupleStruct = TupleStruct<'py>;
    type SerializeTupleVariant = TupleVariant<'py>;
    type SerializeMap = Map<'py>;
    type SerializeStruct = Struct<'py>;
    type SerializeStructVariant = StructVariant<'py>;

    serialize_impl!(serialize_bool, bool);
    serialize_impl!(serialize_i8, i8);
    serialize_impl!(serialize_i16, i16);
    serialize_impl!(serialize_i32, i32);
    serialize_impl!(serialize_i64, i64);
    serialize_impl!(serialize_u8, u8);
    serialize_impl!(serialize_u16, u16);
    serialize_impl!(serialize_u32, u32);
    serialize_impl!(serialize_u64, u64);
    serialize_impl!(serialize_f32, f32);
    serialize_impl!(serialize_f64, f64);
    serialize_impl!(serialize_char, char);
    serialize_impl!(serialize_str, &str);
    serialize_impl!(serialize_bytes, &[u8]);

    fn serialize_none(self) -> Result<Self::Ok> {
        Ok(self.py.None().into_bound(self.py))
    }

    fn serialize_some<T>(self, value: &T) -> Result<Self::Ok>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<Self::Ok> {
        Ok(PyTuple::empty(self.py).into_any())
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok> {
        Ok(PyTuple::empty(self.py).into_any())
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok> {
        Ok(PyString::new(self.py, variant).into_any())
    }

    fn serialize_newtype_struct<T>(self, _name: &'static str, value: &T) -> Result<Self::Ok>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
    }

    fn serialize_newtype_variant<T>(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok>
    where
        T: ?Sized + Serialize,
    {
        let dict = PyDict::new(self.py).into_any();
        dict.set_item(variant, value.serialize(self)?)?;
        Ok(dict)
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq> {
        Ok(Seq {
            py: self.py,
            seq: Vec::new(),
        })
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple> {
        Ok(Seq {
            py: self.py,
            seq: Vec::new(),
        })
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct> {
        Ok(TupleStruct {
            py: self.py,
            fields: Vec::new(),
        })
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        Ok(TupleVariant {
            py: self.py,
            variant,
            fields: Vec::new(),
        })
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap> {
        Ok(Map {
            py: self.py,
            map: PyDict::new(self.py),
            key: None,
        })
    }

    fn serialize_struct(self, _name: &'static str, _len: usize) -> Result<Self::SerializeStruct> {
        Ok(Struct {
            py: self.py,
            fields: PyDict::new(self.py),
        })
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant> {
        Ok(StructVariant {
            py: self.py,
            variant,
            fields: PyDict::new(self.py),
        })
    }
}

pub struct Seq<'py> {
    py: Python<'py>,
    seq: Vec<Bound<'py, PyAny>>,
}

impl<'py> ser::SerializeSeq for Seq<'py> {
    type Ok = Bound<'py, PyAny>;
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        self.seq
            .push(value.serialize(PyAnySerializer { py: self.py })?);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok> {
        Ok(PyList::new(self.py, self.seq)?.into_any())
    }
}

impl<'py> ser::SerializeTuple for Seq<'py> {
    type Ok = Bound<'py, PyAny>;
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        self.seq
            .push(value.serialize(PyAnySerializer { py: self.py })?);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok> {
        Ok(PyTuple::new(self.py, self.seq)?.into_any())
    }
}

pub struct TupleStruct<'py> {
    py: Python<'py>,
    fields: Vec<Bound<'py, PyAny>>,
}

impl<'py> ser::SerializeTupleStruct for TupleStruct<'py> {
    type Ok = Bound<'py, PyAny>;
    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        self.fields
            .push(value.serialize(PyAnySerializer { py: self.py })?);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok> {
        Ok(PyTuple::new(self.py, self.fields)?.into_any())
    }
}

pub struct TupleVariant<'py> {
    py: Python<'py>,
    variant: &'static str,
    fields: Vec<Bound<'py, PyAny>>,
}

impl<'py> ser::SerializeTupleVariant for TupleVariant<'py> {
    type Ok = Bound<'py, PyAny>;
    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        self.fields
            .push(value.serialize(PyAnySerializer { py: self.py })?);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok> {
        let dict = PyDict::new(self.py);
        dict.set_item(self.variant, PyTuple::new(self.py, self.fields)?)?;
        Ok(dict.into_any())
    }
}

pub struct Map<'py> {
    py: Python<'py>,
    map: Bound<'py, PyDict>,
    key: Option<Bound<'py, PyAny>>,
}

impl<'py> ser::SerializeMap for Map<'py> {
    type Ok = Bound<'py, PyAny>;
    type Error = Error;

    fn serialize_key<T>(&mut self, key: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        self.key = Some(key.serialize(PyAnySerializer { py: self.py })?);
        Ok(())
    }

    fn serialize_value<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        let key = self
            .key
            .take()
            .expect("Invalid Serialize implementation. Key is missing.");
        self.map
            .set_item(key, value.serialize(PyAnySerializer { py: self.py })?)?;
        Ok(())
    }

    fn end(self) -> Result<Self::Ok> {
        Ok(self.map.into_any())
    }
}

pub struct Struct<'py> {
    py: Python<'py>,
    fields: Bound<'py, PyDict>,
}

impl<'py> ser::SerializeStruct for Struct<'py> {
    type Ok = Bound<'py, PyAny>;
    type Error = Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        self.fields
            .set_item(key, value.serialize(PyAnySerializer { py: self.py })?)?;
        Ok(())
    }

    fn end(self) -> Result<Self::Ok> {
        Ok(self.fields.into_any())
    }
}

pub struct StructVariant<'py> {
    py: Python<'py>,
    variant: &'static str,
    fields: Bound<'py, PyDict>,
}

impl<'py> ser::SerializeStructVariant for StructVariant<'py> {
    type Ok = Bound<'py, PyAny>;
    type Error = Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        self.fields
            .set_item(key, value.serialize(PyAnySerializer { py: self.py })?)?;
        Ok(())
    }

    fn end(self) -> Result<Self::Ok> {
        let dict = PyDict::new(self.py);
        dict.set_item(self.variant, self.fields)?;
        Ok(dict.into_any())
    }
}

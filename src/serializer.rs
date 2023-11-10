use crate::error::{Error, Result};
use pyo3::{
    prelude::*,
    types::{PyBool, PyByteArray, PyDict, PyFloat, PyList, PyString, PyTuple},
};
use serde::{ser, Serialize};

pub fn as_pyobject<'py, T>(py: Python<'py>, value: &T) -> Result<&'py PyAny>
where
    T: Serialize + ?Sized,
{
    let serializer = PyAnySerializer { py };
    value.serialize(serializer)
}

pub struct PyAnySerializer<'py> {
    py: Python<'py>,
}

macro_rules! serialize_integer {
    ($f:ident, $t:ty) => {
        fn $f(self, v: $t) -> Result<Self::Ok> {
            Ok(v.into_py(self.py).into_ref(self.py))
        }
    };
}

impl<'py> ser::Serializer for PyAnySerializer<'py> {
    type Ok = &'py PyAny;

    type Error = Error;

    type SerializeSeq = Seq<'py>;
    type SerializeTuple = Seq<'py>;
    type SerializeTupleStruct = TupleStruct<'py>;
    type SerializeTupleVariant = TupleVariant<'py>;
    type SerializeMap = Map<'py>;
    type SerializeStruct = Struct<'py>;
    type SerializeStructVariant = StructVariant<'py>;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok> {
        Ok(PyBool::new(self.py, v))
    }

    serialize_integer!(serialize_i8, i8);
    serialize_integer!(serialize_i16, i16);
    serialize_integer!(serialize_i32, i32);
    serialize_integer!(serialize_i64, i64);
    serialize_integer!(serialize_u8, u8);
    serialize_integer!(serialize_u16, u16);
    serialize_integer!(serialize_u32, u32);
    serialize_integer!(serialize_u64, u64);

    fn serialize_f32(self, v: f32) -> Result<Self::Ok> {
        Ok(PyFloat::new(self.py, v as f64))
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok> {
        Ok(PyFloat::new(self.py, v))
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok> {
        let s = v.to_string();
        Ok(PyString::new(self.py, &s))
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok> {
        Ok(PyString::new(self.py, v))
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok> {
        Ok(PyByteArray::new(self.py, v))
    }

    fn serialize_none(self) -> Result<Self::Ok> {
        Ok(self.py.None().into_ref(self.py))
    }

    fn serialize_some<T>(self, value: &T) -> Result<Self::Ok>
    where
        T: ?Sized + Serialize,
    {
        let inner = value.serialize(self)?;
        Ok(inner)
    }

    fn serialize_unit(self) -> Result<Self::Ok> {
        Ok(PyTuple::empty(self.py))
    }

    fn serialize_unit_struct(self, name: &'static str) -> Result<Self::Ok> {
        let dict = PyDict::new(self.py);
        dict.set_item(name, PyTuple::empty(self.py))?;
        Ok(dict)
    }

    fn serialize_unit_variant(
        self,
        name: &'static str,
        _index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok> {
        let dict = PyDict::new(self.py);
        dict.set_item(name, variant)?;
        Ok(dict)
    }

    fn serialize_newtype_struct<T>(self, name: &'static str, value: &T) -> Result<Self::Ok>
    where
        T: ?Sized + Serialize,
    {
        let dict = PyDict::new(self.py);
        dict.set_item(name, value.serialize(self)?)?;
        Ok(dict)
    }

    fn serialize_newtype_variant<T>(
        self,
        name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok>
    where
        T: ?Sized + Serialize,
    {
        let dict = PyDict::new(self.py);
        dict.set_item(name, (variant, value.serialize(self)?))?;
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
        name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct> {
        Ok(TupleStruct {
            py: self.py,
            name,
            fields: Vec::new(),
        })
    }

    fn serialize_tuple_variant(
        self,
        name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        Ok(TupleVariant {
            py: self.py,
            name,
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

    fn serialize_struct(self, name: &'static str, _len: usize) -> Result<Self::SerializeStruct> {
        Ok(Struct {
            py: self.py,
            name,
            fields: PyDict::new(self.py),
        })
    }

    fn serialize_struct_variant(
        self,
        name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant> {
        Ok(StructVariant {
            py: self.py,
            name,
            variant,
            fields: PyDict::new(self.py),
        })
    }
}

pub struct Seq<'py> {
    py: Python<'py>,
    seq: Vec<&'py PyAny>,
}

impl<'py> ser::SerializeSeq for Seq<'py> {
    type Ok = &'py PyAny;
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
        Ok(PyList::new(self.py, self.seq))
    }
}

impl<'py> ser::SerializeTuple for Seq<'py> {
    type Ok = &'py PyAny;
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
        Ok(PyTuple::new(self.py, self.seq))
    }
}

pub struct TupleStruct<'py> {
    py: Python<'py>,
    name: &'static str,
    fields: Vec<&'py PyAny>,
}

impl<'py> ser::SerializeTupleStruct for TupleStruct<'py> {
    type Ok = &'py PyAny;
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
        let inner = PyList::new(self.py, self.fields);
        let dict = PyDict::new(self.py);
        dict.set_item(self.name, inner)?;
        Ok(dict)
    }
}

pub struct TupleVariant<'py> {
    py: Python<'py>,
    name: &'static str,
    variant: &'static str,
    fields: Vec<&'py PyAny>,
}

impl<'py> ser::SerializeTupleVariant for TupleVariant<'py> {
    type Ok = &'py PyAny;
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
        let inner = PyTuple::new(self.py, self.fields);
        let dict = PyDict::new(self.py);
        dict.set_item(self.name, (self.variant, inner))?;
        Ok(dict)
    }
}

pub struct Map<'py> {
    py: Python<'py>,
    map: &'py PyDict,
    key: Option<&'py PyAny>,
}

impl<'py> ser::SerializeMap for Map<'py> {
    type Ok = &'py PyAny;
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
        Ok(self.map)
    }
}

pub struct Struct<'py> {
    py: Python<'py>,
    name: &'static str,
    fields: &'py PyDict,
}

impl<'py> ser::SerializeStruct for Struct<'py> {
    type Ok = &'py PyAny;
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
        dict.set_item(self.name, self.fields)?;
        Ok(dict)
    }
}

pub struct StructVariant<'py> {
    py: Python<'py>,
    name: &'static str,
    variant: &'static str,
    fields: &'py PyDict,
}

impl<'py> ser::SerializeStructVariant for StructVariant<'py> {
    type Ok = &'py PyAny;
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
        dict.set_item(self.name, (self.variant, self.fields))?;
        Ok(dict)
    }
}

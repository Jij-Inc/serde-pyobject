use crate::error::{Error, Result};
use pyo3::{
    prelude::*,
    types::{PyBool, PyDict, PyFloat, PyString, PyTuple},
};
use serde::{ser, Serialize};

pub fn as_py_object<'py, T>(py: Python<'py>, value: &T) -> Result<&'py PyAny>
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

    fn serialize_bytes(self, _v: &[u8]) -> Result<Self::Ok> {
        todo!()
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
        todo!()
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq> {
        todo!()
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple> {
        todo!()
    }

    fn serialize_tuple_struct(
        self,
        name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct> {
        Ok(TupleStruct {
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
            name,
            variant,
            fields: Vec::new(),
        })
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap> {
        todo!()
    }

    fn serialize_struct(self, name: &'static str, _len: usize) -> Result<Self::SerializeStruct> {
        Ok(Struct {
            name,
            fields: Vec::new(),
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
            name,
            variant,
            fields: Vec::new(),
        })
    }
}

#[derive(Debug)]
pub struct Seq<'py> {
    seq: Vec<&'py PyAny>,
}

impl<'py> ser::SerializeSeq for Seq<'py> {
    type Ok = &'py PyAny;
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        todo!()
    }

    fn end(self) -> Result<Self::Ok> {
        todo!()
    }
}

impl<'py> ser::SerializeTuple for Seq<'py> {
    type Ok = &'py PyAny;
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        todo!()
    }

    fn end(self) -> Result<Self::Ok> {
        todo!()
    }
}

#[derive(Debug)]
pub struct TupleStruct<'py> {
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
        todo!()
    }

    fn end(self) -> Result<Self::Ok> {
        todo!()
    }
}

#[derive(Debug)]
pub struct TupleVariant<'py> {
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
        todo!()
    }

    fn end(self) -> Result<Self::Ok> {
        todo!()
    }
}

#[derive(Debug)]
pub struct Map<'py> {
    map: Vec<(&'py PyAny, &'py PyAny)>,
    key: Option<&'py PyAny>,
}

impl<'py> ser::SerializeMap for Map<'py> {
    type Ok = &'py PyAny;
    type Error = Error;

    fn serialize_key<T>(&mut self, key: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        todo!()
    }

    fn serialize_value<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        todo!()
    }

    fn end(self) -> Result<Self::Ok> {
        todo!()
    }
}

#[derive(Debug)]
pub struct Struct<'py> {
    name: &'static str,
    fields: Vec<(&'static str, &'py PyAny)>,
}

impl<'py> ser::SerializeStruct for Struct<'py> {
    type Ok = &'py PyAny;
    type Error = Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        todo!()
    }

    fn end(self) -> Result<Self::Ok> {
        todo!()
    }
}

#[derive(Debug)]
pub struct StructVariant<'py> {
    name: &'static str,
    variant: &'static str,
    fields: Vec<(&'static str, &'py PyAny)>,
}

impl<'py> ser::SerializeStructVariant for StructVariant<'py> {
    type Ok = &'py PyAny;
    type Error = Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        todo!()
    }

    fn end(self) -> Result<Self::Ok> {
        todo!()
    }
}

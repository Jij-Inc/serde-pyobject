use crate::error::{Error, Result};
use pyo3::types::PyDict;
use serde::{ser, Serialize};

pub struct PyDictSerializer<'py> {
    phantom: std::marker::PhantomData<&'py ()>,
}

macro_rules! serialize_primitive {
    ($f:ident, $t:ty, $p:expr) => {
        fn $f(self, _v: $t) -> Result<Self::Ok> {
            todo!()
        }
    };
}

impl<'py> ser::Serializer for PyDictSerializer<'py> {
    type Ok = &'py PyDict;

    type Error = Error;

    type SerializeSeq = TypeTagSeq;
    type SerializeTuple = TypeTagSeq;
    type SerializeTupleStruct = TypeTagTupleStruct;
    type SerializeTupleVariant = TypeTagTupleVariant;
    type SerializeMap = TypeTagMap;
    type SerializeStruct = TypeTagStruct;
    type SerializeStructVariant = TypeTagStructVariant;

    serialize_primitive!(serialize_bool, bool, Primitive::Bool);
    serialize_primitive!(serialize_i8, i8, Primitive::I8);
    serialize_primitive!(serialize_i16, i16, Primitive::I16);
    serialize_primitive!(serialize_i32, i32, Primitive::I32);
    serialize_primitive!(serialize_i64, i64, Primitive::I64);
    serialize_primitive!(serialize_u8, u8, Primitive::U8);
    serialize_primitive!(serialize_u16, u16, Primitive::U16);
    serialize_primitive!(serialize_u32, u32, Primitive::U32);
    serialize_primitive!(serialize_u64, u64, Primitive::U64);
    serialize_primitive!(serialize_f32, f32, Primitive::F32);
    serialize_primitive!(serialize_f64, f64, Primitive::F64);
    serialize_primitive!(serialize_char, char, Primitive::Char);

    fn serialize_str(self, _v: &str) -> Result<Self::Ok> {
        Ok(TypeTag::String)
    }

    fn serialize_bytes(self, _v: &[u8]) -> Result<Self::Ok> {
        Ok(TypeTag::ByteArray)
    }

    fn serialize_none(self) -> Result<Self::Ok> {
        Ok(TypeTag::None)
    }

    fn serialize_some<T>(self, value: &T) -> Result<Self::Ok>
    where
        T: ?Sized + Serialize,
    {
        let tag = TypeTag::from_value(value);
        Ok(TypeTag::Some(Box::new(tag)))
    }

    fn serialize_unit(self) -> Result<Self::Ok> {
        Ok(TypeTag::Unit)
    }

    fn serialize_unit_struct(self, name: &'static str) -> Result<Self::Ok> {
        Ok(TypeTag::UnitStruct { name })
    }

    fn serialize_unit_variant(
        self,
        name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok> {
        Ok(TypeTag::UnitVariant { name, variant })
    }

    fn serialize_newtype_struct<T>(self, name: &'static str, value: &T) -> Result<Self::Ok>
    where
        T: ?Sized + Serialize,
    {
        let tag = TypeTag::from_value(value);
        Ok(TypeTag::NewTypeStruct {
            name,
            inner: Box::new(tag),
        })
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
        let tag = TypeTag::from_value(value);
        Ok(TypeTag::NewTypeVariant {
            name,
            variant,
            inner: Box::new(tag),
        })
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq> {
        Ok(TypeTagSeq {
            seq: Seq::default(),
        })
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple> {
        Ok(TypeTagSeq {
            seq: Seq::default(),
        })
    }

    fn serialize_tuple_struct(
        self,
        name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct> {
        Ok(TypeTagTupleStruct {
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
        Ok(TypeTagTupleVariant {
            name,
            variant,
            fields: Vec::new(),
        })
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap> {
        Ok(TypeTagMap {
            map: Map::default(),
            key: None,
        })
    }

    fn serialize_struct(self, name: &'static str, _len: usize) -> Result<Self::SerializeStruct> {
        Ok(TypeTagStruct {
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
        Ok(TypeTagStructVariant {
            name,
            variant,
            fields: Vec::new(),
        })
    }
}

#[derive(Debug)]
pub struct TypeTagSeq {
    seq: Seq,
}

impl ser::SerializeSeq for TypeTagSeq {
    type Ok = TypeTag;
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        let tag = TypeTag::from_value(value);
        self.seq.push(tag);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok> {
        Ok(TypeTag::Seq(self.seq))
    }
}

impl ser::SerializeTuple for TypeTagSeq {
    type Ok = TypeTag;
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        let tag = TypeTag::from_value(value);
        self.seq.push(tag);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok> {
        Ok(TypeTag::Tuple(self.seq))
    }
}

#[derive(Debug)]
pub struct TypeTagTupleStruct {
    name: &'static str,
    fields: Vec<TypeTag>,
}

impl ser::SerializeTupleStruct for TypeTagTupleStruct {
    type Ok = TypeTag;
    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        let tag = TypeTag::from_value(value);
        self.fields.push(tag);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok> {
        Ok(TypeTag::TupleStruct {
            name: self.name,
            fields: self.fields,
        })
    }
}

#[derive(Debug)]
pub struct TypeTagTupleVariant {
    name: &'static str,
    variant: &'static str,
    fields: Vec<TypeTag>,
}

impl ser::SerializeTupleVariant for TypeTagTupleVariant {
    type Ok = TypeTag;
    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        let tag = TypeTag::from_value(value);
        self.fields.push(tag);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok> {
        Ok(TypeTag::TupleVariant {
            name: self.name,
            variant: self.variant,
            fields: self.fields,
        })
    }
}

#[derive(Debug)]
pub struct TypeTagMap {
    map: Map,
    key: Option<TypeTag>,
}

impl ser::SerializeMap for TypeTagMap {
    type Ok = TypeTag;
    type Error = Error;

    fn serialize_key<T>(&mut self, key: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        let tag = TypeTag::from_value(key);
        self.key = Some(tag);
        Ok(())
    }

    fn serialize_value<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        let tag = TypeTag::from_value(value);
        let key = self.key.take().unwrap();
        self.map.push(key, tag);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok> {
        assert!(self.key.is_none());
        Ok(TypeTag::Map(self.map))
    }
}

#[derive(Debug)]
pub struct TypeTagStruct {
    name: &'static str,
    fields: Vec<(&'static str, TypeTag)>,
}

impl ser::SerializeStruct for TypeTagStruct {
    type Ok = TypeTag;
    type Error = Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        let tag = TypeTag::from_value(value);
        self.fields.push((key, tag));
        Ok(())
    }

    fn end(self) -> Result<Self::Ok> {
        Ok(TypeTag::Struct {
            name: self.name,
            fields: self.fields,
        })
    }
}

#[derive(Debug)]
pub struct TypeTagStructVariant {
    name: &'static str,
    variant: &'static str,
    fields: Vec<(&'static str, TypeTag)>,
}

impl ser::SerializeStructVariant for TypeTagStructVariant {
    type Ok = TypeTag;
    type Error = Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        let tag = TypeTag::from_value(value);
        self.fields.push((key, tag));
        Ok(())
    }

    fn end(self) -> Result<Self::Ok> {
        Ok(TypeTag::StructVariant {
            name: self.name,
            variant: self.variant,
            fields: self.fields,
        })
    }
}

use crate::{Error, Result};
use serde::ser::{self, Serialize};
use std::cell::RefCell;

pub struct BytesSerializer {
    buffer: RefCell<Vec<u8>>,
}

impl BytesSerializer {
    pub fn new() -> Self {
        BytesSerializer {
            buffer: RefCell::new(Vec::new()),
        }
    }

    pub fn to_bytes<T: Serialize>(&self, value: &T) -> Result<Vec<u8>> {
        self.buffer.borrow_mut().clear();
        value.serialize(self)?;
        Ok(self.buffer.borrow().clone())
    }
}

impl ser::Serializer for &BytesSerializer {
    type Ok = ();
    type Error = Error;

    type SerializeSeq = Self;
    type SerializeTuple = Self;
    type SerializeTupleStruct = Self;
    type SerializeTupleVariant = Self;
    type SerializeMap = Self;
    type SerializeStruct = Self;
    type SerializeStructVariant = Self;

    // We will only implement serialization logic for u8, u32 and structs containing those
    fn serialize_bool(self, v: bool) -> Result<()> {
        self.buffer.borrow_mut().push(if v { 1 } else { 0 });
        Ok(())
    }

    fn serialize_i8(self, _v: i8) -> Result<()> {
        Err(Error::UnsupportedType)
    }

    fn serialize_i16(self, _v: i16) -> Result<()> {
        Err(Error::UnsupportedType)
    }

    fn serialize_i32(self, _v: i32) -> Result<()> {
        Err(Error::UnsupportedType)
    }

    fn serialize_i64(self, _v: i64) -> Result<()> {
        Err(Error::UnsupportedType)
    }

    fn serialize_u8(self, v: u8) -> Result<()> {
        self.buffer.borrow_mut().push(v);
        Ok(())
    }

    fn serialize_u16(self, _v: u16) -> Result<()> {
        Err(Error::UnsupportedType)
    }

    fn serialize_u32(self, v: u32) -> Result<()> {
        self.buffer.borrow_mut().extend(&v.to_le_bytes());
        Ok(())
    }

    fn serialize_u64(self, _v: u64) -> Result<()> {
        Err(Error::UnsupportedType)
    }

    fn serialize_f32(self, _v: f32) -> Result<()> {
        Err(Error::UnsupportedType)
    }

    fn serialize_f64(self, _v: f64) -> Result<()> {
        Err(Error::UnsupportedType)
    }

    fn serialize_char(self, _v: char) -> Result<()> {
        Err(Error::UnsupportedType)
    }

    fn serialize_str(self, _v: &str) -> Result<()> {
        Err(Error::UnsupportedType)
    }

    fn serialize_bytes(self, _v: &[u8]) -> Result<()> {
        Err(Error::UnsupportedType)
    }

    fn serialize_none(self) -> Result<()> {
        self.buffer.borrow_mut().push(0);
        Ok(())
    }

    fn serialize_some<T>(self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        self.buffer.borrow_mut().push(1);
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<()> {
        self.buffer.borrow_mut().push(0);
        Ok(())
    }

    // Unit Structs are structs without any fields
    // They are created by `struct Unit;`
    fn serialize_unit_struct(self, _name: &'static str) -> Result<()> {
        self.buffer.borrow_mut().push(0);
        Ok(())
    }

    // Unit Variants are enum variants without any fields
    // They are created by `enum Enum { Variant, }`
    fn serialize_unit_variant(
        self,
        _name: &'static str,
        variant_index: u32,
        _variant: &'static str,
    ) -> Result<()> {
        self.serialize_u32(variant_index)
    }

    // Newtype Structs are structs with a single unnamed field
    // They are created by `struct NewType(u32);`
    fn serialize_newtype_struct<T>(self, _name: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
    }

    // Newtype Variants are enum variants with a single unnamed field
    // They are created by `enum Enum { Variant(u32), }`
    fn serialize_newtype_variant<T>(
        self,
        _name: &'static str,
        variant_index: u32,
        _variant: &'static str,
        value: &T,
    ) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        self.serialize_u32(variant_index)?;
        value.serialize(self)
    }

    // Seqs are used for serializing sequences of values
    // They are created by `vec![1, 2, 3]`
    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq> {
        let lenu32 = match len {
            Some(len) => len as u32,
            None => 0 as u32,
        };
        self.serialize_u32(lenu32)?;
        Ok(self)
    }

    // Tuples are used for serializing fixed size sequences of values
    // They are created by `(1, 2, 3)`
    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple> {
        Ok(self)
    }

    // Tuple Structs are used for serializing structs with unnamed fields
    // They are created by `struct Tuple(u32, u32);`
    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct> {
        Ok(self)
    }

    // Tuple Variants are used for serializing enum variants with unnamed fields
    // They are created by `enum Enum { Variant(u32, u32), }`
    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        self.serialize_u32(variant_index)?;
        Ok(self)
    }

    // Maps are used for serializing maps
    // They are created by `HashMap::new()`
    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap> {
        Ok(self)
    }

    // Structs are used for serializing structs
    // They are created by `struct Struct { a: u32, b: u32 }`
    fn serialize_struct(self, _name: &'static str, _len: usize) -> Result<Self::SerializeStruct> {
        Ok(self)
    }

    // Struct Variants are used for serializing enum variants with named fields
    // They are created by `enum Enum { Variant { a: u32, b: u32 }, }`
    fn serialize_struct_variant(
        self,
        _name: &'static str,
        variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant> {
        self.serialize_u32(variant_index)?;
        Ok(self)
    }
}

impl ser::SerializeSeq for &BytesSerializer {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(*self)
    }

    fn end(self) -> Result<()> {
        Ok(())
    }
}

impl ser::SerializeTuple for &BytesSerializer {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(*self)
    }

    fn end(self) -> Result<()> {
        Ok(())
    }
}

impl ser::SerializeTupleStruct for &BytesSerializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(*self)
    }

    fn end(self) -> Result<()> {
        Ok(())
    }
}

impl ser::SerializeTupleVariant for &BytesSerializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(*self)
    }

    fn end(self) -> Result<()> {
        Ok(())
    }
}

impl ser::SerializeMap for &BytesSerializer {
    type Ok = ();
    type Error = Error;

    fn serialize_key<T>(&mut self, _key: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        Err(Error::UnsupportedType)
    }

    fn serialize_value<T>(&mut self, _value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        Err(Error::UnsupportedType)
    }

    fn end(self) -> Result<()> {
        Ok(())
    }
}

impl ser::SerializeStruct for &BytesSerializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, _key: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(*self)
    }

    fn end(self) -> Result<()> {
        Ok(())
    }
}

impl ser::SerializeStructVariant for &BytesSerializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, _key: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(*self)
    }

    fn end(self) -> Result<()> {
        Ok(())
    }
}

use crate::{Error, Result};
use serde::ser::{self, Serialize};
use std::cell::RefCell;

pub struct BytesSerializer {
    buffer: RefCell<Vec<u8>>,
    offsets: RefCell<Vec<usize>>,
}

impl BytesSerializer {
    pub fn new() -> Self {
        BytesSerializer {
            buffer: RefCell::new(Vec::new()),
            offsets: RefCell::new(Vec::new()),
        }
    }

    pub fn to_bytes<T: Serialize>(&self, value: &T) -> Result<Vec<u8>> {
        value.serialize(self)?;
        Ok(self.buffer.take())
    }

    fn start_bytelen_encoding(&self) -> Result<&Self> {
        // Push the current buffer length to the offsets stack
        self.offsets.borrow_mut().push(self.buffer.borrow().len());
        // Extend the buffer with 4 bytes for the length of the sequence
        self.buffer.borrow_mut().extend(&0u32.to_le_bytes());
        Ok(self)
    }

    fn end_bytelen_encoding(&self) -> Result<()> {
        // Get the current buffer length
        let buffer_len = self.buffer.borrow().len();
        // Get the last offset
        let offset = self.offsets.borrow_mut().pop().unwrap_or_default();
        // Calculate the length of the sequence
        let len = (buffer_len - offset - 4) as u32;
        // Write the length to the buffer
        self.buffer.borrow_mut()[offset..offset + 4].copy_from_slice(&len.to_le_bytes());
        Ok(())
    }
}

pub fn to_bytes<T: Serialize>(value: &T) -> Result<Vec<u8>> {
    let ser = BytesSerializer::new();
    ser.to_bytes(value)
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
        Err(Error::Unimplemented)
    }

    fn serialize_i16(self, _v: i16) -> Result<()> {
        Err(Error::Unimplemented)
    }

    fn serialize_i32(self, _v: i32) -> Result<()> {
        Err(Error::Unimplemented)
    }

    fn serialize_i64(self, _v: i64) -> Result<()> {
        Err(Error::Unimplemented)
    }

    fn serialize_u8(self, v: u8) -> Result<()> {
        self.buffer.borrow_mut().push(v);
        Ok(())
    }

    fn serialize_u16(self, _v: u16) -> Result<()> {
        Err(Error::Unimplemented)
    }

    fn serialize_u32(self, v: u32) -> Result<()> {
        self.buffer.borrow_mut().extend(&v.to_le_bytes());
        Ok(())
    }

    fn serialize_u64(self, _v: u64) -> Result<()> {
        Err(Error::Unimplemented)
    }

    fn serialize_f32(self, _v: f32) -> Result<()> {
        Err(Error::Unimplemented)
    }

    fn serialize_f64(self, _v: f64) -> Result<()> {
        Err(Error::Unimplemented)
    }

    fn serialize_char(self, _v: char) -> Result<()> {
        Err(Error::Unimplemented)
    }

    fn serialize_str(self, _v: &str) -> Result<()> {
        Err(Error::Unimplemented)
    }

    fn serialize_bytes(self, _v: &[u8]) -> Result<()> {
        Err(Error::Unimplemented)
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
        self.serialize_unit()
    }

    // Unit Variants are enum variants without any fields
    // They are created by `enum Enum { Variant, }`
    fn serialize_unit_variant(
        self,
        _name: &'static str,
        variant_index: u32,
        _variant: &'static str,
    ) -> Result<()> {
        let _ = self.start_bytelen_encoding();
        // If variant_index < u8::MAX, we can serialize it as a single byte
        // Otherwise we return an error
        if variant_index <= u8::MAX as u32 {
            self.buffer.borrow_mut().push(variant_index as u8);
            self.end_bytelen_encoding()?;
            Ok(())
        } else {
            Err(Error::InvalidData)
        }
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
        let _ = self.start_bytelen_encoding();
        // If variant_index < u8::MAX, we can serialize it as a single byte
        // Otherwise we return an error
        if variant_index <= u8::MAX as u32 {
            self.buffer.borrow_mut().push(variant_index as u8);
        } else {
            return Err(Error::InvalidData);
        }
        value.serialize(self)
    }

    // Seqs are used for serializing sequences of values
    // They are created by `vec![1, 2, 3]`
    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq> {
        self.start_bytelen_encoding()
    }

    // Tuples are used for serializing fixed size sequences of values
    // They are created by `(1, 2, 3)`
    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple> {
        self.start_bytelen_encoding()
    }

    // Tuple Structs are used for serializing structs with unnamed fields
    // They are created by `struct Tuple(u32, u32);`
    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct> {
        self.start_bytelen_encoding()
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
        let _ = self.start_bytelen_encoding();
        if variant_index <= u8::MAX as u32 {
            self.buffer.borrow_mut().push(variant_index as u8);
        } else {
            return Err(Error::InvalidData);
        }
        Ok(self)
    }

    // Maps are used for serializing maps
    // They are created by `HashMap::new()`
    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap> {
        let _ = self.start_bytelen_encoding();
        Ok(self)
    }

    // Structs are used for serializing structs
    // They are created by `struct Struct { a: u32, b: u32 }`
    fn serialize_struct(self, _name: &'static str, _len: usize) -> Result<Self::SerializeStruct> {
        let _ = self.start_bytelen_encoding();
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
        let _ = self.start_bytelen_encoding();
        if variant_index <= u8::MAX as u32 {
            self.buffer.borrow_mut().push(variant_index as u8);
        } else {
            return Err(Error::InvalidData);
        }
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
        self.end_bytelen_encoding()
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
        self.end_bytelen_encoding()
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
        self.end_bytelen_encoding()
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
        self.end_bytelen_encoding()
    }
}

impl ser::SerializeMap for &BytesSerializer {
    type Ok = ();
    type Error = Error;

    fn serialize_key<T>(&mut self, key: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        key.serialize(*self)
    }

    fn serialize_value<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(*self)
    }

    fn end(self) -> Result<()> {
        self.end_bytelen_encoding()
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
        self.end_bytelen_encoding()
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
        self.end_bytelen_encoding()
    }
}

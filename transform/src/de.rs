use crate::{Error, Result};
use serde::de::{self, Deserialize};
use std::cell::RefCell;

pub struct BytesDeserializer {
    buffer: RefCell<Vec<u8>>,
    position: RefCell<usize>,
    offsets: RefCell<Vec<usize>>,
}

impl BytesDeserializer {
    pub fn new() -> Self {
        BytesDeserializer {
            buffer: RefCell::new(Vec::new()),
            position: RefCell::new(0),
            offsets: RefCell::new(Vec::new()),
        }
    }

    pub fn from_bytes<'a, T>(&self, bytes: &[u8]) -> Result<T>
    where
        T: Deserialize<'a>,
    {
        self.buffer.borrow_mut().clear();
        self.buffer.borrow_mut().extend(bytes);
        T::deserialize(self)
    }

    fn read_bytes(&self, len: usize) -> Result<Vec<u8>> {
        let mut pos = self.position.borrow_mut();
        let buffer = self.buffer.borrow();
        let end = *pos + len;
        if end > buffer.len() {
            return Err(Error::Custom("Unexpected end of input".to_string()));
        }
        let result = buffer[*pos..end].to_vec();
        *pos = end;
        Ok(result)
    }

    fn read_byte(&self) -> Result<u8> {
        let bytes = self.read_bytes(1)?;
        Ok(bytes[0])
    }

    fn read_u32(&self) -> Result<u32> {
        let bytes = self.read_bytes(4)?;
        Ok(u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]))
    }

    fn peek_position(&self) -> usize {
        *self.position.borrow()
    }
}

pub fn from_bytes<'a, T>(bytes: &[u8]) -> Result<T>
where
    T: de::Deserialize<'a>,
{
    let de = BytesDeserializer::new();
    de.from_bytes(bytes)
}

impl<'de> de::Deserializer<'de> for &BytesDeserializer {
    type Error = Error;

    fn deserialize_any<V: de::Visitor<'de>>(self, _visitor: V) -> Result<V::Value> {
        Err(Error::Unimplemented)
    }

    fn deserialize_bool<V: de::Visitor<'de>>(self, _visitor: V) -> Result<V::Value> {
        Err(Error::Unimplemented)
    }

    fn deserialize_i8<V: de::Visitor<'de>>(self, _visitor: V) -> Result<V::Value> {
        Err(Error::Unimplemented)
    }

    fn deserialize_i16<V: de::Visitor<'de>>(self, _visitor: V) -> Result<V::Value> {
        Err(Error::Unimplemented)
    }

    fn deserialize_i32<V: de::Visitor<'de>>(self, _visitor: V) -> Result<V::Value> {
        Err(Error::Unimplemented)
    }

    fn deserialize_i64<V: de::Visitor<'de>>(self, _visitor: V) -> Result<V::Value> {
        Err(Error::Unimplemented)
    }

    fn deserialize_u8<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        visitor.visit_u8(self.read_byte()?)
    }

    fn deserialize_u16<V: de::Visitor<'de>>(self, _visitor: V) -> Result<V::Value> {
        Err(Error::Unimplemented)
    }

    fn deserialize_u32<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        visitor.visit_u32(self.read_u32()?)
    }

    fn deserialize_u64<V: de::Visitor<'de>>(self, _visitor: V) -> Result<V::Value> {
        Err(Error::Unimplemented)
    }

    fn deserialize_f32<V: de::Visitor<'de>>(self, _visitor: V) -> Result<V::Value> {
        Err(Error::Unimplemented)
    }

    fn deserialize_f64<V: de::Visitor<'de>>(self, _visitor: V) -> Result<V::Value> {
        Err(Error::Unimplemented)
    }

    fn deserialize_char<V: de::Visitor<'de>>(self, _visitor: V) -> Result<V::Value> {
        Err(Error::Unimplemented)
    }

    /// Hint that the `Deserialize` type is expecting a string value and does
    /// not benefit from taking ownership of buffered data owned by the
    /// `Deserializer`.
    ///
    /// If the `Visitor` would benefit from taking ownership of `String` data,
    /// indicate this to the `Deserializer` by using `deserialize_string`
    /// instead.
    fn deserialize_str<V: de::Visitor<'de>>(self, _visitor: V) -> Result<V::Value> {
        Err(Error::Unimplemented)
    }

    /// Hint that the `Deserialize` type is expecting a string value and would
    /// benefit from taking ownership of buffered data owned by the
    /// `Deserializer`.
    ///
    /// If the `Visitor` would not benefit from taking ownership of `String`
    /// data, indicate that to the `Deserializer` by using `deserialize_str`
    /// instead.
    fn deserialize_string<V: de::Visitor<'de>>(self, _visitor: V) -> Result<V::Value> {
        Err(Error::Unimplemented)
    }

    /// Hint that the `Deserialize` type is expecting a byte array and does not
    /// benefit from taking ownership of buffered data owned by the
    /// `Deserializer`.
    ///
    /// If the `Visitor` would benefit from taking ownership of `Vec<u8>` data,
    /// indicate this to the `Deserializer` by using `deserialize_byte_buf`
    /// instead.
    fn deserialize_bytes<V: de::Visitor<'de>>(self, _visitor: V) -> Result<V::Value> {
        // Unimplmented for now as serialization is without length prefix
        Err(Error::Unimplemented)
    }

    /// Hint that the `Deserialize` type is expecting a byte array and would
    /// benefit from taking ownership of buffered data owned by the
    /// `Deserializer`.
    ///
    /// If the `Visitor` would not benefit from taking ownership of `Vec<u8>`
    /// data, indicate that to the `Deserializer` by using `deserialize_bytes`
    /// instead.
    fn deserialize_byte_buf<V: de::Visitor<'de>>(self, _visitor: V) -> Result<V::Value> {
        Err(Error::Unimplemented)
    }

    /// Hint that the `Deserialize` type is expecting an optional value.
    ///
    /// This allows deserializers that encode an optional value as a nullable
    /// value to convert the null value into `None` and a regular value into
    /// `Some(value)`.
    fn deserialize_option<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        let byte = self.read_byte()?;
        match byte {
            0 => de::Visitor::visit_none(visitor),
            1 => de::Visitor::visit_some(visitor, self),
            _ => Err(Error::Custom("Invalid Option value".to_string())),
        }
    }

    /// Hint that the `Deserialize` type is expecting a unit value.
    fn deserialize_unit<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        let byte = self.read_byte()?;
        if byte == 0 {
            visitor.visit_unit()
        } else {
            Err(Error::Custom("Invalid Unit value".to_string()))
        }
    }

    /// Hint that the `Deserialize` type is expecting a unit struct with a
    /// particular name.
    fn deserialize_unit_struct<V: de::Visitor<'de>>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value> {
        let byte = self.read_byte()?;
        if byte == 0 {
            visitor.visit_unit()
        } else {
            Err(Error::Custom("Invalid Unit Struct value".to_string()))
        }
    }

    /// Hint that the `Deserialize` type is expecting a newtype struct with a
    /// particular name.
    fn deserialize_newtype_struct<V: de::Visitor<'de>>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value> {
        visitor.visit_newtype_struct(self)
    }

    /// Hint that the `Deserialize` type is expecting a sequence of values.
    /// We need to implement this
    fn deserialize_seq<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        // read u32 for number of bytes
        let len = self.read_u32()? as usize;
        // Push the current buffer length to the offsets
        self.offsets.borrow_mut().push(self.buffer.borrow().len());
        match visitor.visit_seq(SeqAccess::new(self, len)) {
            Ok(value) => {
                self.offsets.borrow_mut().pop();
                Ok(value)
            }
            Err(e) => Err(e),
        }
    }

    /// Hint that the `Deserialize` type is expecting a sequence of values and
    /// knows how many values there are without looking at the serialized data.
    /// We need to implement this
    fn deserialize_tuple<V: de::Visitor<'de>>(self, _len: usize, visitor: V) -> Result<V::Value> {
        // read u32 for number of bytes
        let len = self.read_u32()? as usize;
        // Push the current buffer length to the offsets
        self.offsets.borrow_mut().push(self.buffer.borrow().len());
        match visitor.visit_seq(SeqAccess::new(self, len)) {
            Ok(value) => {
                self.offsets.borrow_mut().pop();
                Ok(value)
            }
            Err(e) => Err(e),
        }
    }

    /// Hint that the `Deserialize` type is expecting a tuple struct with a
    /// particular name and number of fields.
    fn deserialize_tuple_struct<V: de::Visitor<'de>>(
        self,
        _name: &'static str,
        _len: usize,
        visitor: V,
    ) -> Result<V::Value> {
        // read u32 for number of bytes
        let len = self.read_u32()? as usize;
        // Push the current buffer length to the offsets
        self.offsets.borrow_mut().push(self.buffer.borrow().len());
        match visitor.visit_seq(SeqAccess::new(self, len)) {
            Ok(value) => {
                self.offsets.borrow_mut().pop();
                Ok(value)
            }
            Err(e) => Err(e),
        }
    }

    /// Hint that the `Deserialize` type is expecting a map of key-value pairs.
    fn deserialize_map<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        // read u32 for number of bytes
        let len = self.read_u32()? as usize;
        // Push the current buffer length to the offsets
        self.offsets.borrow_mut().push(self.buffer.borrow().len());
        match visitor.visit_map(MapAccess::new(self, len)) {
            Ok(value) => {
                self.offsets.borrow_mut().pop();
                Ok(value)
            }
            Err(e) => Err(e),
        }
    }

    /// Hint that the `Deserialize` type is expecting a struct with a particular
    /// name and fields.
    /// We need to implement this
    fn deserialize_struct<V: de::Visitor<'de>>(
        self,
        _name: &'static str,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value> {
        // read u32 for number of bytes
        let len = self.read_u32()? as usize;
        // Push the current buffer length to the offsets
        self.offsets.borrow_mut().push(self.buffer.borrow().len());
        match visitor.visit_seq(SeqAccess::new(self, len)) {
            Ok(value) => {
                self.offsets.borrow_mut().pop();
                Ok(value)
            }
            Err(e) => Err(e),
        }
    }

    /// Hint that the `Deserialize` type is expecting an enum value with a
    /// particular name and possible variants.
    fn deserialize_enum<V: de::Visitor<'de>>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value> {
        let remaining = self.read_u32()? as usize;
        let variant_index = self.read_byte()?;

        visitor.visit_enum(EnumAccess::new(self, variant_index, remaining))
    }

    /// Hint that the `Deserialize` type is expecting the name of a struct
    /// field or the discriminant of an enum variant.
    fn deserialize_identifier<V: de::Visitor<'de>>(self, _visitor: V) -> Result<V::Value> {
        Err(Error::Unimplemented)
    }

    /// Hint that the `Deserialize` type needs to deserialize a value whose type
    /// doesn't matter because it is ignored.
    ///
    /// Deserializers for non-self-describing formats may not support this mode.
    fn deserialize_ignored_any<V: de::Visitor<'de>>(self, _visitor: V) -> Result<V::Value> {
        Err(Error::Unimplemented)
    }
}

struct SeqAccess<'a> {
    de: &'a BytesDeserializer,
    remaining: usize,
}

impl<'a> SeqAccess<'a> {
    fn new(de: &'a BytesDeserializer, remaining: usize) -> Self {
        SeqAccess { de, remaining }
    }
}

impl<'de> de::SeqAccess<'de> for SeqAccess<'_> {
    type Error = Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>>
    where
        T: de::DeserializeSeed<'de>,
    {
        if self.remaining == 0 {
            return Ok(None);
        }

        let before = self.de.peek_position();
        let val = seed.deserialize(&*self.de)?;
        let consumed = self.de.peek_position() - before;

        self.remaining -= consumed;
        Ok(Some(val))
    }
}

struct EnumAccess<'a> {
    de: &'a BytesDeserializer,
    variant_index: u8,
    remaining: RefCell<usize>,
}

impl<'a> EnumAccess<'a> {
    fn new(de: &'a BytesDeserializer, variant_index: u8, remaining: usize) -> Self {
        EnumAccess {
            de,
            variant_index,
            remaining: RefCell::new(remaining),
        }
    }
}

impl<'de> de::EnumAccess<'de> for EnumAccess<'_> {
    type Error = Error;
    type Variant = Self;

    fn variant_seed<V>(self, seed: V) -> Result<(V::Value, Self::Variant)>
    where
        V: de::DeserializeSeed<'de>,
    {
        let val = seed.deserialize(de::value::U8Deserializer::<Error>::new(self.variant_index))?;
        Ok((val, self))
    }
}

impl<'de> de::VariantAccess<'de> for EnumAccess<'_> {
    type Error = Error;

    fn unit_variant(self) -> Result<()> {
        Ok(())
    }

    fn newtype_variant_seed<T>(self, seed: T) -> Result<T::Value>
    where
        T: de::DeserializeSeed<'de>,
    {
        seed.deserialize(self.de)
    }

    fn tuple_variant<V>(self, _len: usize, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let len = *self.remaining.borrow();
        // Push the current buffer length to the offsets
        self.de
            .offsets
            .borrow_mut()
            .push(self.de.buffer.borrow().len());
        match visitor.visit_seq(SeqAccess::new(self.de, len)) {
            Ok(value) => {
                self.de.offsets.borrow_mut().pop();
                Ok(value)
            }
            Err(e) => Err(e),
        }
    }

    fn struct_variant<V>(self, _fields: &'static [&'static str], visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let len = *self.remaining.borrow();
        // Push the current buffer length to the offsets
        self.de
            .offsets
            .borrow_mut()
            .push(self.de.buffer.borrow().len());
        match visitor.visit_seq(SeqAccess::new(self.de, len)) {
            Ok(value) => {
                self.de.offsets.borrow_mut().pop();
                Ok(value)
            }
            Err(e) => Err(e),
        }
    }
}

struct MapAccess<'a> {
    de: &'a BytesDeserializer,
    remaining: usize,
}

impl<'a> MapAccess<'a> {
    fn new(de: &'a BytesDeserializer, len: usize) -> Self {
        MapAccess { de, remaining: len }
    }
}

impl<'de> de::MapAccess<'de> for MapAccess<'_> {
    type Error = Error;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>>
    where
        K: de::DeserializeSeed<'de>,
    {
        if self.remaining == 0 {
            return Ok(None);
        }

        let before = self.de.peek_position();
        let val = seed.deserialize(&*self.de)?;
        let consumed = self.de.peek_position() - before;

        self.remaining -= consumed;
        Ok(Some(val))
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value>
    where
        V: de::DeserializeSeed<'de>,
    {
        let before = self.de.peek_position();
        let val = seed.deserialize(self.de)?;
        let consumed = self.de.peek_position() - before;

        self.remaining -= consumed;
        Ok(val)
    }
}

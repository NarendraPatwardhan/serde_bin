mod de;
mod error;
mod ser;

pub use de::from_bytes;
pub use error::{Error, Result};
pub use ser::to_bytes;

pub fn load<'a, T>(data: Vec<u8>) -> Result<T>
where
    T: serde::de::Deserialize<'a> + serde::ser::Serialize + Default,
{
    let default = T::default();
    let serialized = to_bytes(&default)?;
    let data = &data[..serialized.len()];
    from_bytes(data)
}

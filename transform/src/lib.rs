mod de;
mod error;
mod ser;

pub use de::from_bytes;
pub use error::{Error, Result};
pub use ser::BytesSerializer;
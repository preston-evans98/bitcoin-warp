use byteorder::{LittleEndian, ReadBytesExt};
use std::error::Error;
use std::{fmt, io};

#[derive(Debug)]
pub enum DeserializationError {
    Io(io::Error),
    Parse(String),
}
impl DeserializationError {
    pub fn parse(source: &[u8], into: &str) -> DeserializationError {
        DeserializationError::Parse(format!("Could not construct {} from {:?}", into, source))
    }
}
impl Error for DeserializationError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match *self {
            DeserializationError::Io(ref err) => Some(err),
            DeserializationError::Parse(_) => None,
        }
    }
}
impl fmt::Display for DeserializationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeserializationError::Io(ref err) => err.fmt(f),
            DeserializationError::Parse(ref err) => err.fmt(f),
        }
    }
}
impl From<io::Error> for DeserializationError {
    fn from(err: io::Error) -> DeserializationError {
        DeserializationError::Io(err)
    }
}
type Result<T> = std::result::Result<T, DeserializationError>;

pub trait Deserializable {
    fn deserialize<T>(reader: &mut T) -> Result<Self>
    where
        Self: Sized,
        T: std::io::Read;
}

impl Deserializable for u32 {
    fn deserialize<T>(target: &mut T) -> Result<u32>
    where
        T: std::io::Read,
    {
        Ok(target.read_u32::<LittleEndian>()?)
    }
}

// TODO: Replace when const generics stabilize
macro_rules! impl_deserializable_byte_array {
    ($size:expr) => {
        impl Deserializable for [u8; $size] {
            fn deserialize<T>(target: &mut T) -> Result<[u8; $size]>
            where
                T: std::io::Read,
            {
                let mut result = [0u8; $size];
                target.read_exact(&mut result)?;
                Ok(result)
            }
        }
    };
}

impl_deserializable_byte_array!(4);

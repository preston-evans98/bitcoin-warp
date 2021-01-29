use crate::CompactInt;
use bytes::{Buf, BytesMut};
use std::error::Error;
use std::net::SocketAddr;
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
type Result<R> = std::result::Result<R, DeserializationError>;

pub trait Deserializable {
    fn deserialize(reader: &mut BytesMut) -> Result<Self>
    where
        Self: Sized;
}

impl Deserializable for bool {
    fn deserialize(target: &mut BytesMut) -> Result<bool> {
        let value = target.get_u8();
        match value {
            0 => Ok(false),
            1 => Ok(true),
            _ => Err(DeserializationError::Parse(format!(
                "Could not parse {:?} as bool",
                value
            ))),
        }
    }
}
impl Deserializable for u8 {
    fn deserialize(target: &mut BytesMut) -> Result<u8> {
        Ok(target.get_u8())
    }
}

impl Deserializable for u16 {
    fn deserialize(target: &mut BytesMut) -> Result<u16> {
        Ok(target.get_u16_le())
    }
}

impl Deserializable for u32 {
    fn deserialize(target: &mut BytesMut) -> Result<u32> {
        Ok(target.get_u32_le())
    }
}

impl Deserializable for u64 {
    fn deserialize(target: &mut BytesMut) -> Result<u64> {
        Ok(target.get_u64_le())
    }
}

impl Deserializable for i32 {
    fn deserialize(target: &mut BytesMut) -> Result<i32> {
        Ok(target.get_i32_le())
    }
}

impl Deserializable for i64 {
    fn deserialize(target: &mut BytesMut) -> Result<i64> {
        Ok(target.get_i64_le())
    }
}

impl<T> Deserializable for Vec<T>
where
    T: Deserializable,
{
    fn deserialize(target: &mut BytesMut) -> Result<Vec<T>> {
        let len = CompactInt::deserialize(target)?.value() as usize;
        let mut result: Vec<T> = Vec::with_capacity(len);
        for _ in 0..len {
            result.push(T::deserialize(target)?);
        }
        Ok(result)
    }
}

impl Deserializable for String {
    fn deserialize(target: &mut BytesMut) -> Result<String> {
        let len = CompactInt::deserialize(target)?.value() as usize;
        let mut vec = Vec::with_capacity(len);
        vec.resize(len, 0);
        target.copy_to_slice(&mut vec);
        if let Ok(string) = String::from_utf8(vec) {
            return Ok(string);
        }
        return Ok(String::from("?"));
        // let mut result = String::with_capacity(len);
        // for _ in 0..len {
        //     result.push(target.read_u8()? as char);
        // }
        // Ok(result)
    }
}

// TODO: test
impl Deserializable for SocketAddr {
    fn deserialize(target: &mut BytesMut) -> Result<SocketAddr> {
        Ok(SocketAddr::from((
            <[u8; 16]>::deserialize(target)?,
            target.get_u16(),
        )))
    }
}

// TODO: Replace when const generics stabilize
macro_rules! impl_deserializable_byte_array {
    ($size:expr) => {
        impl Deserializable for [u8; $size] {
            fn deserialize(target: &mut BytesMut) -> Result<[u8; $size]> {
                let mut result = [0u8; $size];
                target.copy_to_slice(&mut result);
                Ok(result)
            }
        }
    };
}

impl_deserializable_byte_array!(4);
impl_deserializable_byte_array!(16);
impl_deserializable_byte_array!(32);

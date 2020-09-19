use crate::CompactInt;
use byteorder::{LittleEndian, ReadBytesExt};
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
    fn deserialize<R>(reader: &mut R) -> Result<Self>
    where
        Self: Sized,
        R: std::io::Read;
}

impl Deserializable for bool {
    fn deserialize<R>(target: &mut R) -> Result<bool>
    where
        R: std::io::Read,
    {
        let value = target.read_u8()?;
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
    fn deserialize<R>(target: &mut R) -> Result<u8>
    where
        R: std::io::Read,
    {
        Ok(target.read_u8()?)
    }
}

impl Deserializable for u16 {
    fn deserialize<R>(target: &mut R) -> Result<u16>
    where
        R: std::io::Read,
    {
        Ok(target.read_u16::<LittleEndian>()?)
    }
}

impl Deserializable for u32 {
    fn deserialize<R>(target: &mut R) -> Result<u32>
    where
        R: std::io::Read,
    {
        Ok(target.read_u32::<LittleEndian>()?)
    }
}

impl Deserializable for u64 {
    fn deserialize<R>(target: &mut R) -> Result<u64>
    where
        R: std::io::Read,
    {
        Ok(target.read_u64::<LittleEndian>()?)
    }
}

impl<T> Deserializable for Vec<T>
where
    T: Deserializable,
{
    fn deserialize<R>(target: &mut R) -> Result<Vec<T>>
    where
        R: std::io::Read,
        T: Deserializable,
    {
        let len = CompactInt::deserialize(target)?.value() as usize;
        let mut result: Vec<T> = Vec::with_capacity(len);
        for _ in 0..len {
            result.push(T::deserialize(target)?);
        }
        Ok(result)
    }
}

impl Deserializable for SocketAddr {
    fn deserialize<R>(target: &mut R) -> Result<SocketAddr>
    where
        R: std::io::Read,
    {
        Ok(SocketAddr::from((
            <[u8; 4]>::deserialize(target)?,
            <u16>::deserialize(target)?,
        )))
    }
}

// TODO: Replace when const generics stabilize
macro_rules! impl_deserializable_byte_array {
    ($size:expr) => {
        impl Deserializable for [u8; $size] {
            fn deserialize<R>(target: &mut R) -> Result<[u8; $size]>
            where
                R: std::io::Read,
            {
                let mut result = [0u8; $size];
                target.read_exact(&mut result)?;
                Ok(result)
            }
        }
    };
}

impl_deserializable_byte_array!(4);

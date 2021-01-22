use crate::{self as shared, Serializable};

/// A Cached type is an option that is never serialized.
///
/// It can be added to any struct without risking a consensus break.
pub struct Cached<T>(Option<T>);

impl<T> Cached<T> {
    pub fn new() -> Cached<T> {
        Cached(None)
    }
}

impl<T> std::fmt::Debug for Cached<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Ok(())
    }
}
impl<T> Serializable for Cached<T> {
    fn serialize<W>(&self, target: &mut W) -> Result<(), std::io::Error>
    where
        W: std::io::Write,
    {
        Ok(())
    }
}
impl<T> shared::Deserializable for Cached<T> {
    fn deserialize<R>(_: &mut R) -> std::result::Result<Self, shared::DeserializationError>
    where
        R: std::io::Read,
    {
        Ok(Cached(None))
    }
}

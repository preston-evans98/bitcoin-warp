use crate::{self as shared, Serializable};
use bytes::Buf;

/// A Cached type is an option that is never serialized.
///
/// It can be added to any struct without risking a consensus break.
pub struct Cached<T>(Option<T>);

impl<T> Cached<T> {
    pub fn new() -> Cached<T> {
        Cached(None)
    }
    pub fn from(val: T) -> Cached<T> {
        Cached(Some(val))
    }
    pub fn value(&self) -> &Option<T> {
        &self.0
    }
    pub fn has_value(&self) -> bool {
        self.0.is_some()
    }
    pub fn ref_value(&self) -> Option<&T> {
        match self.0 {
            Some(ref v) => Some(v),
            None => None,
        }
    }
}

impl<T> std::fmt::Debug for Cached<T>
where
    T: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Ok(())
        self.0.fmt(f);
        Ok(())
    }
}
impl<T> Serializable for Cached<T> {
    fn serialize<W>(&self, _: &mut W) -> Result<(), std::io::Error>
    where
        W: std::io::Write,
    {
        Ok(())
    }
}
impl<T> shared::Deserializable for Cached<T> {
    fn deserialize<B: Buf>(_: B) -> std::result::Result<Self, shared::DeserializationError> {
        Ok(Cached(None))
    }
}

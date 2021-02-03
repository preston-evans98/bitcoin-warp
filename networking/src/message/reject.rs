use bytes::Buf;
use serde_derive::Deserializable;
use shared::Serializable;

#[allow(unused)]
#[derive(Deserializable, Debug)]
pub struct Reject {
    message: String,
    code: u8,
    reason: String,
    extra_data: Option<[u8; 32]>,
}

impl Reject {
    pub fn reason(&self) -> &str {
        &self.reason
    }
}

// TODO: maybe implement?
impl Serializable for Reject {
    fn serialize<W>(&self, target: &mut W) -> Result<(), std::io::Error>
    where
        W: std::io::Write,
    {
        self.message.serialize(target)?;
        target.write_all(&[self.code])?;
        self.reason.serialize(target)?;
        self.extra_data.serialize(target)
    }
}

// impl Deserializable for Reject {
//     fn deserialize<B: Buf>(target: B) -> Result<Self, shared::DeserializationError>
//     where
//         Self: Sized,
//     {
//         Ok(Reject {
//             message: String::deserialize(target)?,
//             code: u8::deserialize(target)?,
//             reason: String::deserialize(target)?,
//             extra_data: <Option<[u8; 32]>>::deserialize(target)?,
//         })
//     }
// }

impl super::Payload for Reject {
    fn serialized_size(&self) -> usize {
        let size = 1 + self.message.len() + self.reason.len();
        if let Some(_) = self.extra_data {
            return size + 32;
        }
        size
    }

    fn to_bytes(&self) -> Result<Vec<u8>, std::io::Error> {
        let mut out = Vec::with_capacity(self.serialized_size());
        self.serialize(&mut out)?;
        Ok(out)
    }
}

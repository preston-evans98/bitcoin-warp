use crate::block_header::BlockHeader;
use serde_derive::{Deserializable, Serializable};
#[derive(Deserializable, Serializable)]
pub struct Headers {
    headers: Vec<BlockHeader>,
}

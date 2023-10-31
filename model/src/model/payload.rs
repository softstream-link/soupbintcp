use crate::prelude::SoupBinTcpPayload;
use byteserde_derive::{ByteDeserializeSlice, ByteSerializeStack, ByteSerializedLenOf};
use serde::{Deserialize, Serialize};

#[rustfmt::skip]
#[derive(ByteSerializeStack, ByteDeserializeSlice, ByteSerializedLenOf, Serialize, Deserialize, PartialEq, Clone, Debug, Default)]
pub struct Nil;
impl SoupBinTcpPayload<Nil> for Nil {}

#[rustfmt::skip]
#[derive(ByteSerializeStack, ByteDeserializeSlice, ByteSerializedLenOf, Serialize, Deserialize, PartialEq, Clone, Debug, Default)]
pub struct VecPayload {
    pub payload: Vec<u8>,
}
impl VecPayload {
    pub fn new(payload: Vec<u8>) -> Self {
        Self { payload }
    }
}
impl SoupBinTcpPayload<VecPayload> for VecPayload {}

use crate::prelude::{SoupBinTcpPayload, UPayload};
use byteserde_derive::{ByteDeserializeSlice, ByteSerializeStack, ByteSerializedLenOf};
use serde::{Deserialize, Serialize};

#[derive(ByteSerializeStack, ByteDeserializeSlice, ByteSerializedLenOf, Serialize, Deserialize, PartialEq, Debug, Clone, Copy, Default)]
pub struct Nil;
impl Default for UPayload<Nil> {
    fn default() -> Self {
        UPayload::new(Nil)
    }
}
impl SoupBinTcpPayload<Nil> for Nil {}

#[derive(ByteSerializeStack, ByteDeserializeSlice, ByteSerializedLenOf, Serialize, Deserialize, PartialEq, Debug, Clone, Default)]
pub struct VecPayload {
    pub payload: Vec<u8>,
}
impl VecPayload {
    pub fn new(payload: Vec<u8>) -> Self {
        Self { payload }
    }
}
impl SoupBinTcpPayload<VecPayload> for VecPayload {}

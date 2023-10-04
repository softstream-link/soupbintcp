use byteserde_derive::{ByteDeserializeSlice, ByteSerializeStack, ByteSerializedLenOf};

use crate::prelude::SoupBinTcpPayload;

#[rustfmt::skip]
#[derive(ByteSerializeStack, ByteDeserializeSlice, ByteSerializedLenOf, PartialEq, Clone, Debug, Default)]
pub struct Nil;
impl SoupBinTcpPayload<Nil> for Nil {}

#[rustfmt::skip]
#[derive(ByteSerializeStack, ByteDeserializeSlice, ByteSerializedLenOf, PartialEq, Clone, Debug, Default)]
pub struct VecPayload {
    pub payload: Vec<u8>,
}
impl SoupBinTcpPayload<VecPayload> for VecPayload {}

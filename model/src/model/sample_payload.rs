use byteserde_derive::{ByteDeserializeSlice, ByteSerializeStack, ByteSerializedLenOf, ByteSerializedSizeOf};
use byteserde_types::string_ascii_fixed;
use serde::{Deserialize, Serialize};

use crate::prelude::{SoupBinTcpPayload, UPayload};

string_ascii_fixed!(Context1, 10, b' ', true, derive(ByteSerializeStack, ByteDeserializeSlice, ByteSerializedSizeOf, ByteSerializedLenOf, PartialEq, Clone, Copy));
string_ascii_fixed!(Context2, 10, b' ', true, derive(ByteSerializeStack, ByteDeserializeSlice, ByteSerializedSizeOf, ByteSerializedLenOf, PartialEq, Clone, Copy));

#[rustfmt::skip]
#[derive(ByteSerializeStack, ByteDeserializeSlice, ByteSerializedLenOf, Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct SamplePayload {
    pub context1: Context1,
    pub context2: Context2,
}
impl SoupBinTcpPayload<SamplePayload> for SamplePayload {}

impl Default for SamplePayload {
    fn default() -> Self {
        Self {
            context1: b"10 char load".as_slice().into(),
            context2: b"hello world".as_slice().into(),
        }
    }
}
impl Default for UPayload<SamplePayload> {
    fn default() -> Self {
        UPayload::new(SamplePayload::default())
    }
}

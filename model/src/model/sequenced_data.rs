use byteserde_derive::{ByteDeserializeSlice, ByteSerializeStack, ByteSerializedLenOf};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

use crate::prelude::{PacketTypeSequencedData, SoupBinTcpPayload};

pub const SEQUENCED_DATA_HEADER_BYTE_LEN: usize = 3;

#[derive(ByteSerializeStack, ByteDeserializeSlice, ByteSerializedLenOf, Serialize, Deserialize, PartialEq, Clone, Debug)]
#[byteserde(endian = "be")]
pub struct SPayloadHeader {
    #[serde(skip)]
    pub packet_length: u16,
    #[serde(default, skip_serializing)]
    pub packet_type: PacketTypeSequencedData,
}

impl SPayloadHeader {
    #[inline(always)]
    pub fn new(packet_length: u16) -> Self {
        SPayloadHeader {
            packet_length,
            packet_type: PacketTypeSequencedData::default(),
        }
    }
}
/// Sequenced Data Packet
/// [SOUP TCP/IP Specification](./model/docs/soupbintcp_spec_4.0.pdf)  // TODO - update link to correct path
#[derive(ByteSerializeStack, ByteDeserializeSlice, ByteSerializedLenOf, Serialize, Deserialize, PartialEq, Clone, Debug)]
#[byteserde(endian = "be")]
#[serde(from = "SPayloadJsonDesShadow<Payload>")]
pub struct SPayload<Payload: SoupBinTcpPayload<Payload>> {
    #[serde(skip)]
    header: SPayloadHeader,
    #[byteserde(deplete ( header.packet_length as usize - 1 ))]
    #[serde(flatten)]
    body: Payload,
}
impl<Payload: SoupBinTcpPayload<Payload>> SPayload<Payload> {
    pub fn new(body: Payload) -> SPayload<Payload> {
        let header = SPayloadHeader::new((body.byte_len() + 1) as u16);
        SPayload { header, body }
    }
}

// shadow struct for serde deserialization of [SPayload<Payload>], used to setup packet_length field
#[derive(Deserialize)]
struct SPayloadJsonDesShadow<Payload: SoupBinTcpPayload<Payload>>(Payload);
impl<Payload: SoupBinTcpPayload<Payload>> From<SPayloadJsonDesShadow<Payload>> for SPayload<Payload> {
    fn from(shadow: SPayloadJsonDesShadow<Payload>) -> Self {
        SPayload {
            header: SPayloadHeader::new((shadow.0.byte_len() + 1) as u16),
            body: shadow.0,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{model::sequenced_data::SEQUENCED_DATA_HEADER_BYTE_LEN, prelude::*};
    use byteserde::prelude::*;
    use links_core::unittest::setup;
    use log::info;
    use serde_json::{from_str, to_string};

    #[test]
    fn test_sequenced_data_header() {
        setup::log::configure_compact();
        let msg_inp = SPayloadHeader::new(10);
        info!("msg_inp:? {:?}", msg_inp);

        let ser: ByteSerializerStack<128> = to_serializer_stack(&msg_inp).unwrap();
        info!("ser: {:#x}", ser);
        assert_eq!(SEQUENCED_DATA_HEADER_BYTE_LEN, ser.len());
        assert_eq!(SEQUENCED_DATA_HEADER_BYTE_LEN, msg_inp.byte_len());

        let msg_out: SPayloadHeader = from_slice(ser.as_slice()).unwrap();
        info!("msg_out:? {:?}", msg_out);
        assert_eq!(msg_out, msg_inp);
    }

    #[test]
    fn test_sequenced_data_byteserde() {
        setup::log::configure_compact();
        let expected_len = SEQUENCED_DATA_HEADER_BYTE_LEN + SamplePayload::default().byte_len();
        let msg_inp = SPayload::new(SamplePayload::default());
        info!("msg_inp:? {:?}", msg_inp);

        let ser: ByteSerializerStack<128> = to_serializer_stack(&msg_inp).unwrap();
        info!("ser: {:#x}", ser);
        assert_eq!(expected_len, ser.len());
        assert_eq!(expected_len, msg_inp.byte_len());

        let msg_out: SPayload<SamplePayload> = from_slice(ser.as_slice()).unwrap();
        info!("msg_out:? {:?}", msg_out);
        assert_eq!(msg_out, msg_inp);
    }

    #[test]
    fn test_sequenced_data_serde() {
        setup::log::configure_compact();

        let msg_inp = SPayload::new(SamplePayload::default());
        info!("msg_inp:? {:?}", msg_inp);

        let json_out = to_string(&msg_inp).unwrap();
        info!("json_out: {}", json_out);
        assert_eq!(r#"{"context1":"10 char lo","context2":"hello worl"}"#, json_out);

        let msg_out: SPayload<SamplePayload> = from_str(&json_out).unwrap();
        info!("msg_out:? {:?}", msg_out);
        assert_eq!(msg_out, msg_inp);

        // acceptable alternatives
        for (i, pass_json) in vec![r#" { "context1": "10 char lo", "context2": "hello worl" } "#].iter().enumerate() {
            info!("=========== {} ===========", i + 1);
            info!("pass_json: {}", pass_json);
            let msg_out: SPayload<SamplePayload> = from_str(pass_json).unwrap();
            info!("msg_out:? {:?}", msg_out);
            assert_eq!(msg_out, msg_inp);
        }
    }
}

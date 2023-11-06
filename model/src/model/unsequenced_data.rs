use byteserde_derive::{ByteDeserializeSlice, ByteSerializeStack, ByteSerializedLenOf};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

use super::{types::PacketTypeUnsequencedData, SoupBinTcpPayload};

pub const UNSEQUENCED_DATA_BYTE_LEN: usize = 3;

#[derive(ByteSerializeStack, ByteDeserializeSlice, ByteSerializedLenOf, Serialize, Deserialize, PartialEq, Clone, Debug)]
#[byteserde(endian = "be")]
pub struct UPayloadHeader {
    #[serde(skip)]
    packet_length: u16,
    #[serde(default, skip_serializing)]
    packet_type: PacketTypeUnsequencedData,
}
impl UPayloadHeader {
    #[inline]
    pub fn new(packet_length: u16) -> Self {
        UPayloadHeader {
            packet_length,
            packet_type: PacketTypeUnsequencedData::default(),
        }
    }
}

#[derive(ByteSerializeStack, ByteDeserializeSlice, ByteSerializedLenOf, Serialize, Deserialize, PartialEq, Clone, Debug)]
#[byteserde(endian = "be")]
#[serde(from = "UPayloadJsonDesShadow<Payload>")]
pub struct UPayload<Payload: SoupBinTcpPayload<Payload>> {
    #[serde(skip)]
    header: UPayloadHeader,
    #[byteserde(deplete ( header.packet_length as usize - 1 ))]
    #[serde(flatten)]
    pub body: Payload,
}
impl<Payload: SoupBinTcpPayload<Payload>> UPayload<Payload> {
    #[inline]
    pub fn new(body: Payload) -> UPayload<Payload> {
        let header = UPayloadHeader::new((body.byte_len() + 1) as u16);
        UPayload { header, body }
    }
}

// shadow struct for serde deserialization of [UPayload<Payload>], used to setup packet_length field
#[derive(Deserialize, Debug)]
struct UPayloadJsonDesShadow<Payload: SoupBinTcpPayload<Payload>>(Payload);
impl<Payload: SoupBinTcpPayload<Payload>> From<UPayloadJsonDesShadow<Payload>> for UPayload<Payload> {
    fn from(shadow: UPayloadJsonDesShadow<Payload>) -> Self {
        UPayload {
            header: UPayloadHeader::new((shadow.0.byte_len() + 1) as u16),
            body: shadow.0,
        }
    }
}
#[cfg(test)]
mod test {
    use crate::{model::unsequenced_data::UNSEQUENCED_DATA_BYTE_LEN, prelude::*};
    use byteserde::prelude::*;
    use links_core::unittest::setup;
    use log::info;
    use serde_json::{from_str, to_string};

    #[test]
    fn test_unsequenced_data_header() {
        setup::log::configure_compact();

        let msg_inp = UPayloadHeader::new(10);
        info!("msg_inp:? {:?}", msg_inp);

        let ser: ByteSerializerStack<128> = to_serializer_stack(&msg_inp).unwrap();
        info!("ser: {:x}", ser);
        assert_eq!(UNSEQUENCED_DATA_BYTE_LEN, ser.len());
        assert_eq!(UNSEQUENCED_DATA_BYTE_LEN, msg_inp.byte_len());

        let msg_out: UPayloadHeader = from_serializer_stack(&ser).unwrap();
        info!("msg_out:? {:?}", msg_out);
        assert_eq!(msg_out, msg_inp);
    }

    #[test]
    fn test_unsequenced_data_byteserde() {
        setup::log::configure_compact();
        let expected_len = UNSEQUENCED_DATA_BYTE_LEN + SamplePayload::default().byte_len();
        let msg_inp = UPayload::default();
        info!("msg_inp:? {:?}", msg_inp);

        let ser: ByteSerializerStack<128> = to_serializer_stack(&msg_inp).unwrap();
        info!("ser: {:x}", ser);
        assert_eq!(expected_len, ser.len());
        assert_eq!(expected_len, msg_inp.byte_len());

        let msg_out: UPayload<SamplePayload> = from_slice(ser.as_slice()).unwrap();
        info!("msg_out:? {:?}", msg_out);
        assert_eq!(msg_out, msg_inp);
    }

    #[test]
    fn test_unsequenced_data_serde() {
        setup::log::configure_compact();
        let msg_inp = UPayload::new(SamplePayload::default());
        info!("msg_inp:? {:?}", msg_inp);

        let json_out = to_string(&msg_inp).unwrap();
        info!("json_out: {}", json_out);
        assert_eq!(r#"{"context1":"10 char lo","context2":"hello worl"}"#, json_out);

        // acceptable alternatives
        for (i, pass_json) in vec![r#" { "context1": "10 char lo", "context2": "hello worl" } "#].iter().enumerate() {
            info!("=========== {} ===========", i + 1);
            info!("pass_json: {}", pass_json);
            let msg_out: UPayload<SamplePayload> = from_str(pass_json).unwrap();
            info!("msg_out:? {:?}", msg_out);
            assert_eq!(msg_out, msg_inp);
        }
    }
}

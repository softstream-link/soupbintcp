use byteserde::prelude::*;
use byteserde_derive::{ByteDeserializeSlice, ByteSerializeStack, ByteSerializedLenOf};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

use super::types::PacketTypeSequencedData;

pub const SEQUENCED_DATA_HEADER_BYTE_LEN: usize = 3;

#[derive(ByteSerializeStack, ByteDeserializeSlice, ByteSerializedLenOf, Serialize, Deserialize, PartialEq, Clone, Debug)]
#[byteserde(endian = "be")]
pub struct SPayloadHeader {
    pub packet_length: u16,
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

#[derive(ByteSerializeStack, ByteDeserializeSlice, ByteSerializedLenOf, Serialize, Deserialize, PartialEq, Clone, Debug)]
#[byteserde(endian = "be")]
pub struct SPayload<Payload>
where Payload: ByteSerializeStack+ByteDeserializeSlice<Payload>+ByteSerializedLenOf+PartialEq+Clone+Debug
{
    // header: SPayloadHeader, // TODO benchmark, measure without header on ouch
    #[serde(skip)]
    pub packet_length: u16,
    pub packet_type: PacketTypeSequencedData,
    // #[byteserde(deplete ( header.packet_length as usize - 1 ))]
    #[byteserde(deplete ( packet_length as usize - 1 ))]
    body: Payload,
}

impl<Payload: ByteSerializeStack+ByteDeserializeSlice<Payload>+ByteSerializedLenOf+PartialEq+Clone+Debug> SPayload<Payload> {
    pub fn new(body: Payload) -> SPayload<Payload> {
        // let header = SPayloadHeader::new((body.byte_len() + 1) as u16);
        // SPayload { header, body }
        SPayload {
            packet_length: (body.byte_len() + 1) as u16,
            packet_type: PacketTypeSequencedData::default(),
            body,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::model::sample_payload::SamplePayload;
    use links_core::unittest::setup;
    use log::info;
    use serde_json::to_string;

    #[test]
    fn test_sequenced_data_header() {
        setup::log::configure();
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
        assert_eq!(r#"{"packet_type":"S","body":{"context1":"10 char lo","context2":"hello worl"}}"#, json_out);
    }
}

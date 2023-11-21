use byteserde::prelude::*;
use byteserde_derive::{ByteDeserializeSlice, ByteSerializeStack, ByteSerializedLenOf};
use byteserde_types::prelude::*;
use serde::{Deserialize, Serialize};
use std::fmt;

use super::types::PacketTypeDebug;

#[derive(ByteSerializeStack, ByteDeserializeSlice, ByteSerializedLenOf, Serialize, Deserialize, PartialEq, Clone, fmt::Debug)]
#[byteserde(endian = "be")]
#[serde(from = "DebugJsonDesShadow")]
pub struct Debug {
    #[serde(skip)]
    #[byteserde(replace( packet_type.byte_len() + text.byte_len() ))]
    packet_length: u16,
    #[serde(default, skip_serializing)]
    packet_type: PacketTypeDebug,
    #[byteserde(deplete ( packet_length as usize - packet_type.byte_len() ))]
    text: StringAscii,
}
impl Debug {
    pub fn new(text: &[u8]) -> Self {
        Debug {
            packet_length: (text.len() + PacketTypeDebug::byte_size()) as u16,
            text: text.into(),
            ..Default::default()
        }
    }
}
impl Default for Debug {
    fn default() -> Self {
        let text = b"This is a default debug message text";
        Debug {
            packet_length: (text.len() + PacketTypeDebug::byte_size()) as u16,
            text: text.into(),
            packet_type: Default::default(),
        }
    }
}
impl fmt::Display for Debug {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.text)
    }
}

// shadow struct for serde deserialization of Debug, used to setup packet_length field
#[derive(Deserialize, Debug)]
struct DebugJsonDesShadow {
    #[serde(default)]
    packet_type: PacketTypeDebug,
    text: StringAscii,
}
impl From<DebugJsonDesShadow> for Debug {
    fn from(shadow: DebugJsonDesShadow) -> Self {
        Debug {
            packet_length: (shadow.text.byte_len() + shadow.packet_type.byte_len()) as u16,
            text: shadow.text,
            packet_type: shadow.packet_type,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::prelude::*;
    use byteserde::prelude::*;
    use links_core::unittest::setup;
    use log::{info, LevelFilter};
    use serde_json::{from_str, to_string};

    #[test]
    fn test_debug_byteserde() {
        setup::log::configure_compact(LevelFilter::Info);

        let msg_inp = Debug::default();
        let expected_packet_len: u16 = (msg_inp.text.len() + msg_inp.packet_type.byte_len()) as u16;
        let expected_byte_len: usize = expected_packet_len as usize + 2;

        info!("msg_inp: {}", msg_inp);
        info!("msg_inp:? {:?}", msg_inp);
        let ser: ByteSerializerStack<128> = to_serializer_stack(&msg_inp).unwrap();
        info!("ser: {:x}", ser);
        assert_eq!(expected_byte_len, ser.len());
        assert_eq!(expected_byte_len, msg_inp.byte_len());

        let msg_out: Debug = from_serializer_stack(&ser).unwrap();
        info!("msg_out:? {:?}", msg_out);
        assert_eq!(msg_out, Debug { packet_length: expected_packet_len, ..msg_inp });
    }

    #[test]
    fn test_debug_serde() {
        setup::log::configure_compact(LevelFilter::Info);

        let msg_inp = Debug::default();
        info!("msg_inp:? {:?}", msg_inp);

        let json_out = to_string(&msg_inp).unwrap();
        info!("json_out: {}", json_out);
        assert_eq!(r#"{"text":"This is a default debug message text"}"#, json_out);

        // acceptable alternatives
        for (i, pass_json) in vec![r#" { "text": "This is a default debug message text" } "#].iter().enumerate() {
            info!("=========== {} ===========", i + 1);
            info!("pass_json: {}", pass_json);
            let msg_out: Debug = from_str(pass_json).unwrap();
            info!("msg_out:? {:?}", msg_out);
            assert_eq!(msg_out, msg_inp);
        }
    }
}

use byteserde_derive::{ByteDeserializeSlice, ByteSerializeStack, ByteSerializedLenOf};
use serde::{Deserialize, Serialize};
use std::fmt::Display;

use crate::model::types::PacketTypeEndOfSession;

pub const END_OF_SESSION_PACKET_LENGTH: u16 = 1;
pub const END_OF_SESSION_BYTE_LEN: usize = END_OF_SESSION_PACKET_LENGTH as usize + 2;
#[derive(ByteSerializeStack, ByteDeserializeSlice, ByteSerializedLenOf, Serialize, Deserialize, PartialEq, Clone, Debug)]
#[byteserde(endian = "be")]
pub struct EndOfSession {
    #[serde(default = "default_packet_length", skip_serializing)]
    packet_length: u16,
    #[serde(default, skip_serializing)]
    packet_type: PacketTypeEndOfSession,
}
impl Default for EndOfSession {
    fn default() -> Self {
        EndOfSession {
            packet_length: END_OF_SESSION_PACKET_LENGTH,
            packet_type: Default::default(),
        }
    }
}
impl Display for EndOfSession {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "End of Session")
    }
}
fn default_packet_length() -> u16 {
    END_OF_SESSION_PACKET_LENGTH
}

#[cfg(test)]
mod test {
    use crate::{model::svc::end_of_session::END_OF_SESSION_BYTE_LEN, prelude::*};
    use byteserde::prelude::*;
    use links_core::unittest::setup;
    use log::{info, LevelFilter};
    use serde_json::{from_str, to_string};

    #[test]
    fn test_end_of_session_byteserde() {
        setup::log::configure_compact(LevelFilter::Info);

        let msg_inp = EndOfSession::default();
        info!("msg_inp: {}", msg_inp);
        info!("msg_inp:? {:?}", msg_inp);
        let ser: ByteSerializerStack<128> = to_serializer_stack(&msg_inp).unwrap();
        info!("ser: {:x}", ser);
        assert_eq!(END_OF_SESSION_BYTE_LEN, ser.len());
        assert_eq!(END_OF_SESSION_BYTE_LEN, msg_inp.byte_len());

        let msg_out: EndOfSession = from_serializer_stack(&ser).unwrap();
        info!("msg_out:? {:?}", msg_out);
        assert_eq!(msg_out, msg_inp);
    }

    #[test]
    fn test_end_of_session_serde() {
        setup::log::configure_compact(LevelFilter::Info);

        let msg_inp = EndOfSession::default();
        info!("msg_inp:? {:?}", msg_inp);

        let json_out = to_string(&msg_inp).unwrap();
        info!("json_out: {}", json_out);
        assert_eq!(r#"{}"#, json_out);

        // acceptable alternatives
        for (i, pass_json) in vec![r#" { } "#].iter().enumerate() {
            info!("=========== {} ===========", i + 1);
            info!("pass_json: {}", pass_json);
            let msg_out: EndOfSession = from_str(pass_json).unwrap();
            info!("msg_out:? {:?}", msg_out);
            assert_eq!(msg_inp, msg_out);
        }
    }
}

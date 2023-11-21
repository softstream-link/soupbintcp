use byteserde_derive::{ByteDeserializeSlice, ByteSerializeStack, ByteSerializedLenOf};
use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Display};

use crate::model::types::{PacketTypeLoginAccepted, SequenceNumber, SessionId};

pub const LOGIN_ACCEPTED_PACKET_LENGTH: u16 = 31; // packet_type/1 + session/10 + sequence_number/20
pub const LOGIN_ACCEPTED_BYTE_LEN: usize = LOGIN_ACCEPTED_PACKET_LENGTH as usize + 2;
#[derive(ByteSerializeStack, ByteDeserializeSlice, ByteSerializedLenOf, Serialize, Deserialize, PartialEq, Clone, Debug)]
#[byteserde(endian = "be")]
pub struct LoginAccepted {
    #[serde(default = "default_packet_length", skip_serializing)]
    packet_length: u16,
    #[serde(default, skip_serializing)]
    packet_type: PacketTypeLoginAccepted,
    session_id: SessionId,
    sequence_number: SequenceNumber,
}
impl LoginAccepted {
    pub fn new(session_id: SessionId, sequence_number: SequenceNumber) -> LoginAccepted {
        LoginAccepted {
            packet_length: LOGIN_ACCEPTED_PACKET_LENGTH,
            packet_type: Default::default(),
            session_id,
            sequence_number,
        }
    }
}
impl Default for LoginAccepted {
    fn default() -> Self {
        LoginAccepted {
            packet_length: LOGIN_ACCEPTED_PACKET_LENGTH,
            packet_type: Default::default(),
            session_id: b"session #1".into(),
            sequence_number: 1_u64.into(),
        }
    }
}
impl Display for LoginAccepted {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Login Accepted, your session \"{}\", next sequence number \"{}\"", self.session_id, self.sequence_number)
    }
}
fn default_packet_length() -> u16 {
    LOGIN_ACCEPTED_PACKET_LENGTH
}

#[cfg(test)]
mod test {
    use crate::{model::svc::login_accepted::LOGIN_ACCEPTED_BYTE_LEN, prelude::*};
    use byteserde::prelude::*;
    use links_core::unittest::setup;
    use log::{info, LevelFilter};
    use serde_json::to_string;

    #[test]
    fn test_login_accepted_byteserde() {
        setup::log::configure_compact(LevelFilter::Info);
        let msg_inp = LoginAccepted::default();
        info!("msg_inp: {}", msg_inp);
        info!("msg_inp:? {:?}", msg_inp);
        let ser: ByteSerializerStack<128> = to_serializer_stack(&msg_inp).unwrap();
        info!("ser: {:#x}", ser);
        assert_eq!(LOGIN_ACCEPTED_BYTE_LEN, ser.len());
        assert_eq!(LOGIN_ACCEPTED_BYTE_LEN, msg_inp.byte_len());

        let msg_out: LoginAccepted = from_serializer_stack(&ser).unwrap();
        info!("msg_out:? {:?}", msg_out);
        assert_eq!(msg_out, msg_inp);
    }

    #[test]
    fn test_login_accepted_serde() {
        setup::log::configure_compact(LevelFilter::Info);
        let msg_inp = LoginAccepted::default();
        info!("msg_inp:? {:?}", msg_inp);

        let json_out = to_string(&msg_inp).unwrap();
        info!("json_out: {}", json_out);
        assert_eq!(r#"{"session_id":"session #1","sequence_number":"1"}"#, json_out);

        // acceptable alternatives
        for (i, pass_json) in vec![r#" {  "session_id": "session #1", "sequence_number": "1" } "#].iter().enumerate() {
            info!("=========== {} ===========", i + 1);
            info!("pass_json: {}", pass_json);
            let msg_out: LoginAccepted = serde_json::from_str(pass_json).unwrap();
            info!("msg_out:? {:?}", msg_out);
            assert_eq!(msg_out, msg_inp);
        }
    }
}

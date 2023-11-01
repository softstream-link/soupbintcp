use byteserde_derive::{ByteDeserializeSlice, ByteSerializeStack, ByteSerializedLenOf};
use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Display};

use crate::model::types::{PacketTypeLoginRequest, Password, SequenceNumber, SessionId, TimeoutMs, UserName};

// packet_type/1 + usr/6 + pwd/10 + requested_session/10 + requested_sequence_number/20 + heartbeat_timeout_ms/5
pub const LOGIN_REQUEST_PACKET_LENGTH: u16 = 52;
pub const LOGIN_REQUEST_BYTE_LEN: usize = LOGIN_REQUEST_PACKET_LENGTH as usize + 2;

#[derive(ByteSerializeStack, ByteDeserializeSlice, ByteSerializedLenOf, Serialize, Deserialize, PartialEq, Clone)]
#[byteserde(endian = "be")]
pub struct LoginRequest {
    #[serde(default = "default_packet_length", skip_serializing)]
    packet_length: u16,
    #[serde(default, skip_serializing)]
    packet_type: PacketTypeLoginRequest,

    pub username: UserName,
    pub password: Password,
    pub session_id: SessionId,
    pub sequence_number: SequenceNumber,
    pub hbeat_timeout_ms: TimeoutMs,
}
impl LoginRequest {
    pub fn new(username: UserName, password: Password, session_id: SessionId, sequence_number: SequenceNumber, hbeat_timeout_ms: TimeoutMs) -> LoginRequest {
        LoginRequest {
            packet_length: LOGIN_REQUEST_PACKET_LENGTH,
            packet_type: Default::default(),
            username,
            password,
            session_id,
            sequence_number,
            hbeat_timeout_ms,
        }
    }
}
impl Debug for LoginRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut obfuscated = self.clone();
        obfuscated.password = b"********".as_slice().into(); // obfuscate password
        f.debug_struct("LoginRequest")
            .field("packet_length", &obfuscated.packet_length)
            .field("packet_type", &obfuscated.packet_type)
            .field("username", &obfuscated.username)
            .field("password", &obfuscated.password)
            .field("session_id", &obfuscated.session_id)
            .field("sequence_number", &obfuscated.sequence_number)
            .field("hbeat_timeout", &obfuscated.hbeat_timeout_ms)
            .finish()
    }
}
impl Default for LoginRequest {
    fn default() -> Self {
        LoginRequest::new(b"dummy".as_slice().into(), b"dummy".as_slice().into(), b"session #1".into(), 1_u64.into(), 5000_u16.into())
    }
}
impl Display for LoginRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Login Request, as username \"{}\" requested for session \"{}\", sequence \"{}\", heartbeat timeout {}ms",
            self.username, self.session_id, self.sequence_number, self.hbeat_timeout_ms,
        )
    }
}
fn default_packet_length() -> u16 {
    LOGIN_REQUEST_PACKET_LENGTH
}
#[cfg(test)]
mod test {
    use crate::{model::clt::login_request::LOGIN_REQUEST_BYTE_LEN, prelude::*};
    use byteserde::prelude::*;
    use links_core::unittest::setup;
    use log::info;
    use serde_json::{from_str, to_string};

    #[test]
    fn test_login_request_byteserde() {
        setup::log::configure_compact();
        let msg_inp = LoginRequest::default();
        info!("msg_inp: {}", msg_inp);
        info!("msg_inp:? {:?}", msg_inp);

        let msg_inp = LoginRequest::new(b"abcdef".into(), b"1234567890".into(), b"session #1".into(), 1_u64.into(), 5000_u16.into());
        info!("msg_inp: {}", msg_inp);
        info!("msg_inp:? {:?}", msg_inp);

        let ser: ByteSerializerStack<128> = to_serializer_stack(&msg_inp).unwrap();
        info!("ser: {:#x}", ser);
        assert_eq!(LOGIN_REQUEST_BYTE_LEN, ser.len());
        assert_eq!(LOGIN_REQUEST_BYTE_LEN, msg_inp.byte_len());

        let msg_out: LoginRequest = from_serializer_stack(&ser).unwrap();
        info!("msg_out:? {:?}", msg_out);
        assert_eq!(msg_out, msg_inp);
    }

    #[test]
    fn test_login_request_serde() {
        setup::log::configure_compact();
        let msg_inp = LoginRequest::default();
        info!("msg_inp:? {:?}", msg_inp);

        let json_out = to_string(&msg_inp).unwrap();
        info!("json_out: {}", json_out);
        assert_eq!(r#"{"username":"dummy","password":"dummy","session_id":"session #1","sequence_number":"1","hbeat_timeout_ms":"5000"}"#, json_out);

        // acceptable alternatives
        for (i, pass_json) in vec![r#" {"username":"dummy","password":"dummy","session_id":"session #1","sequence_number":"1","hbeat_timeout_ms":"5000"} "#].iter().enumerate() {
            info!("=========== {} ===========", i + 1);
            info!("pass_json: {}", pass_json);
            let msg_out: LoginRequest = from_str(pass_json).unwrap();
            info!("msg_out:? {:?}", msg_out);
            assert_eq!(msg_inp, msg_out);
        }
    }
}

use byteserde_derive::{ByteDeserializeSlice, ByteSerializeStack, ByteSerializedLenOf};
use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Display};

use crate::model::types::{LoginRejectReason, PacketTypeLoginRejected};

pub const LOGIN_REJECTED_PACKET_LENGTH: u16 = 2;
pub const LOGIN_REJECTED_BYTE_LEN: usize = LOGIN_REJECTED_PACKET_LENGTH as usize + 2;
#[derive(ByteSerializeStack, ByteDeserializeSlice, ByteSerializedLenOf, Serialize, Deserialize, PartialEq, Clone, Debug)]
#[byteserde(endian = "be")]
pub struct LoginRejected {
    #[serde(default = "default_packet_length", skip_serializing)]
    packet_length: u16,
    #[serde(default, skip_serializing)]
    packet_type: PacketTypeLoginRejected,
    reason: LoginRejectReason,
}
impl LoginRejected {
    pub fn not_authorized() -> Self {
        LoginRejected {
            packet_length: LOGIN_REJECTED_PACKET_LENGTH,
            packet_type: Default::default(),
            reason: LoginRejectReason::not_authorized(),
        }
    }
    pub fn session_not_available() -> Self {
        LoginRejected {
            packet_length: LOGIN_REJECTED_PACKET_LENGTH,
            packet_type: Default::default(),
            reason: LoginRejectReason::session_not_available(),
        }
    }
    #[inline(always)]
    pub fn is_not_authorized(&self) -> bool {
        self.reason.is_not_authorized()
    }
    #[inline(always)]
    pub fn is_session_not_available(&self) -> bool {
        self.reason.is_session_not_available()
    }
}
impl Display for LoginRejected {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = if self.reason == LoginRejectReason::new(b'A') {
            "Not Authorized. Invalid username or password in the LoginRequest"
        } else if self.reason == LoginRejectReason::new(b'S') {
            "Session Not Available. Te requested session in the LoginRequest was not valid or not available"
        } else {
            "Unknown"
        };
        write!(f, "Login Rejected reason \"{}\"", msg)
    }
}
fn default_packet_length() -> u16 {
    LOGIN_REJECTED_PACKET_LENGTH
}

#[cfg(test)]
mod test {
    use crate::{
        model::svc::login_rejected::{LOGIN_REJECTED_BYTE_LEN, LOGIN_REJECTED_PACKET_LENGTH},
        prelude::*,
    };
    use byteserde::prelude::*;
    use links_core::unittest::setup;
    use log::{info, LevelFilter};
    use serde_json::{from_str, to_string};

    #[test]
    fn test_login_rejected_byteserde() {
        setup::log::configure_compact(LevelFilter::Info);

        let msg_inp = LoginRejected::not_authorized();
        info!("msg_inp: {}", msg_inp);
        info!("msg_inp:? {:?}", msg_inp);
        let ser: ByteSerializerStack<128> = to_serializer_stack(&msg_inp).unwrap();
        info!("ser: {:x}", ser);
        assert_eq!(LOGIN_REJECTED_BYTE_LEN, ser.len());
        assert_eq!(LOGIN_REJECTED_BYTE_LEN, msg_inp.byte_len());

        let msg_inp = LoginRejected::session_not_available();
        info!("msg_inp: {}", msg_inp);
        let ser: ByteSerializerStack<128> = to_serializer_stack(&msg_inp).unwrap();
        info!("ser: {:x}", ser);
        assert_eq!(ser.len() - 2, LOGIN_REJECTED_PACKET_LENGTH as usize);

        let msg_out: LoginRejected = from_serializer_stack(&ser).unwrap();
        info!("msg_out:? {:?}", msg_out);
        assert_eq!(msg_out, msg_inp);
    }

    #[test]
    fn test_login_rejected_serde() {
        setup::log::configure_compact(LevelFilter::Info);
        let msg_inp = LoginRejected::not_authorized();
        info!("msg_inp:? {:?}", msg_inp);

        let json_out = to_string(&msg_inp).unwrap();
        info!("json_out: {}", json_out);
        assert_eq!(r#"{"reason":"NOT_AUTHORIZED"}"#, json_out);

        let msg_inp = LoginRejected::session_not_available();
        info!("msg_inp:? {:?}", msg_inp);

        let json_out = to_string(&msg_inp).unwrap();
        info!("json_out: {}", json_out);
        assert_eq!(r#"{"reason":"SESSION_NOT_AVAILABLE"}"#, json_out);

        // acceptable alternatives
        for (i, pass_json) in vec![r#" { "reason":"SESSION_NOT_AVAILABLE" } "#, r#" { "reason":"S" } "#].iter().enumerate() {
            info!("=========== {} ===========", i + 1);
            info!("pass_json: {}", pass_json);
            let msg_out: LoginRejected = from_str(pass_json).unwrap();
            info!("msg_out:? {:?}", msg_out);
            assert_eq!(msg_inp, msg_out);
        }
    }
}

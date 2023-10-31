use byteserde_derive::{ByteDeserializeSlice, ByteSerializeStack, ByteSerializedLenOf};
use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Display};

use crate::model::types::PacketTypeLogoutRequest;

pub const LOGOUT_REQUEST_PACKET_LENGTH: u16 = 1;
pub const LOGOUT_REQUEST_BYTE_LEN: usize = LOGOUT_REQUEST_PACKET_LENGTH as usize + 2;

#[derive(ByteSerializeStack, ByteDeserializeSlice, ByteSerializedLenOf, Serialize, Deserialize, PartialEq, Clone, Debug)]
#[byteserde(endian = "be")]
pub struct LogoutRequest {
    #[serde(default = "default_packet_length", skip_serializing)]
    packet_length: u16,
    #[serde(default)]
    packet_type: PacketTypeLogoutRequest,
}
impl Default for LogoutRequest {
    fn default() -> Self {
        LogoutRequest {
            packet_length: LOGOUT_REQUEST_PACKET_LENGTH,
            packet_type: Default::default(),
        }
    }
}
impl Display for LogoutRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Logout Request")
    }
}
fn default_packet_length() -> u16 {
    LOGOUT_REQUEST_PACKET_LENGTH
}

#[cfg(test)]
#[cfg(feature = "unittest")]
mod test {
    use super::*;
    use byteserde::prelude::*;
    use links_core::unittest::setup;
    use log::info;
    use serde_json::{to_string, from_str};

    #[test]
    fn test_logout_request_byteserde() {
        setup::log::configure();
        let msg_inp = LogoutRequest::default();
        info!("msg_inp: {}", msg_inp);
        info!("msg_inp:? {:?}", msg_inp);
        let ser: ByteSerializerStack<128> = to_serializer_stack(&msg_inp).unwrap();
        info!("ser: {:#x}", ser);
        assert_eq!(LOGOUT_REQUEST_BYTE_LEN, ser.len());
        assert_eq!(LOGOUT_REQUEST_BYTE_LEN, msg_inp.byte_len());

        let msg_out: LogoutRequest = from_serializer_stack(&ser).unwrap();
        info!("msg_out:? {:?}", msg_out);
        assert_eq!(msg_out, msg_inp);
    }

    #[test]
    fn test_logout_request_serde() {
        setup::log::configure();
        let msg_inp = LogoutRequest::default();
        info!("msg_inp:? {:?}", msg_inp);

        let json_out = to_string(&msg_inp).unwrap();
        info!("json_out: {}", json_out);
        assert_eq!(r#"{"packet_type":"O"}"#, json_out);

        // acceptable alternatives
        for (i, pass_json) in vec![
            r#" {"packet_type":"O"} "#,
            r#" {} "#,
        ]
        .iter()
        .enumerate()
        {
            info!("=========== {} ===========", i + 1);
            info!("pass_json: {}", pass_json);
            let msg_out: LogoutRequest = from_str(&pass_json).unwrap();
            info!("msg_out:? {:?}", msg_out);
            assert_eq!(msg_out, msg_inp);
        }
    }
}

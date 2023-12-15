pub use soupbintcp_field_types::*;
pub use soupbintcp_packet_types::*;

use byteserde_derive::{ByteDeserializeSlice, ByteSerializeStack, ByteSerializedLenOf, ByteSerializedSizeOf};

pub mod soupbintcp_packet_types {
    use super::*;
    use byteserde_types::const_char_ascii;
    const_char_ascii!(PacketTypeCltHeartbeat, b'R', true, #[derive(ByteSerializeStack, ByteSerializedSizeOf, ByteSerializedLenOf, PartialEq, Clone, Copy)]);
    const_char_ascii!(PacketTypeSvcHeartbeat, b'H', true, #[derive(ByteSerializeStack, ByteSerializedSizeOf, ByteSerializedLenOf, PartialEq, Clone, Copy)]);
    const_char_ascii!(PacketTypeDebug, b'+', true, #[derive(ByteSerializeStack, ByteSerializedSizeOf, ByteSerializedLenOf, PartialEq, Clone, Copy)]);
    const_char_ascii!(PacketTypeEndOfSession, b'Z', true, #[derive(ByteSerializeStack, ByteSerializedSizeOf, ByteSerializedLenOf, PartialEq, Clone, Copy)]);
    const_char_ascii!(PacketTypeLoginAccepted, b'A', true, #[derive(ByteSerializeStack, ByteSerializedSizeOf, ByteSerializedLenOf, PartialEq, Clone, Copy)]);
    const_char_ascii!(PacketTypeLoginRejected, b'J', true, #[derive(ByteSerializeStack, ByteSerializedSizeOf, ByteSerializedLenOf, PartialEq, Clone, Copy)]);
    const_char_ascii!(PacketTypeLoginRequest, b'L', true, #[derive(ByteSerializeStack, ByteSerializedSizeOf, ByteSerializedLenOf, PartialEq, Clone, Copy)]);
    const_char_ascii!(PacketTypeLogoutRequest, b'O', true, #[derive(ByteSerializeStack, ByteSerializedSizeOf, ByteSerializedLenOf, PartialEq, Clone, Copy)]);
    const_char_ascii!(PacketTypeSequencedData, b'S', true, #[derive(ByteSerializeStack, ByteSerializedSizeOf, ByteSerializedLenOf, PartialEq, Clone, Copy)]);
    const_char_ascii!(PacketTypeUnsequencedData, b'U', true, #[derive(ByteSerializeStack, ByteSerializedSizeOf, ByteSerializedLenOf, PartialEq, Clone, Copy)]);
}

pub mod soupbintcp_field_types {
    use std::time::Duration;

    use super::*;
    use byteserde_types::{char_ascii, string_ascii_fixed};
    use links_core::core::macros::short_type_name;

    string_ascii_fixed!(SessionId, 10, b' ', true, true, #[derive(ByteSerializeStack, ByteDeserializeSlice, ByteSerializedSizeOf, ByteSerializedLenOf, PartialEq, Clone, Copy)]);
    impl Default for SessionId {
        fn default() -> Self {
            // all banks to log into the currently active session
            b"          ".into()
        }
    }

    // TODO add docs https://stackoverflow.com/questions/33999341/generating-documentation-in-macros
    string_ascii_fixed!(SequenceNumber, 20, b' ', true, true, #[derive(ByteSerializeStack, ByteDeserializeSlice, ByteSerializedSizeOf, ByteSerializedLenOf, PartialEq, Clone, Copy)]);
    impl From<u64> for SequenceNumber {
        fn from(v: u64) -> Self {
            v.to_string().as_bytes().into()
        }
    }
    impl Default for SequenceNumber {
        fn default() -> Self {
            // 0 to start receiving the most recently generated message
            b"0".as_slice().into()
        }
    }

    string_ascii_fixed!(TimeoutMs, 5, b' ', true, true, #[derive(ByteSerializeStack, ByteDeserializeSlice, ByteSerializedSizeOf, ByteSerializedLenOf, PartialEq, Clone, Copy)]);
    impl From<u16> for TimeoutMs {
        fn from(v: u16) -> Self {
            v.to_string().as_bytes().into()
        }
    }
    impl From<TimeoutMs> for u16 {
        fn from(v: TimeoutMs) -> Self {
            let s = std::str::from_utf8(v.as_slice()).unwrap_or_else(|_| panic!("Failed to convert {:?} to u16", v)).trim();
            s.parse::<u16>().unwrap_or_else(|_| panic!("Failed to convert {:?} to u16", v))
        }
    }
    impl From<TimeoutMs> for u64 {
        fn from(v: TimeoutMs) -> Self {
            let s = std::str::from_utf8(v.as_slice()).unwrap_or_else(|_| panic!("Failed to convert {:?} to u64", v)).trim();
            s.parse::<u64>().unwrap_or_else(|_| panic!("Failed to convert {:?} to u64", v))
        }
    }
    impl From<TimeoutMs> for Duration {
        fn from(v: TimeoutMs) -> Self {
            Duration::from_millis(u64::from(v))
        }
    }
    impl From<Duration> for TimeoutMs {
        fn from(v: Duration) -> Self {
            assert!(v.as_millis() <= u16::MAX as u128, "Duration {:?} exceeds max of {:?}", v, Duration::from_millis(u16::MAX as u64));
            (v.as_millis() as u16).into()
        }
    }

    #[cfg(test)]
    mod test_timeout_ms {
        use super::TimeoutMs;
        use links_core::unittest::setup;
        use log::info;
        use std::time::Duration;

        #[test]
        fn test_sequence_number() {
            setup::log::configure();
            let mut t: TimeoutMs = Duration::from_millis(1000).into();
            info!("Duration::from_millis: {}", t);
            let d : Duration = t.into();
            info!("Duration: {:?}", d);
            t = 1000_u16.into();

            let millis_u64: u64 = t.into();
            info!("millis_u64: {}", millis_u64);
            assert_eq!(millis_u64, 1000);
            let millis_u16: u16 = t.into();
            info!("millis_u16: {}", millis_u16);
            assert_eq!(millis_u16, 1000);
        }
    }

    string_ascii_fixed!(UserName, 6, b' ', true, true, #[derive(ByteSerializeStack, ByteDeserializeSlice, ByteSerializedSizeOf, ByteSerializedLenOf, PartialEq, Clone, Copy)]);
    string_ascii_fixed!(Password, 10, b' ', true, true, #[derive(ByteSerializeStack, ByteDeserializeSlice, ByteSerializedSizeOf, ByteSerializedLenOf, PartialEq, Clone, Copy)]);
    char_ascii!(LoginRejectReason, #[derive(ByteSerializeStack, ByteDeserializeSlice, ByteSerializedSizeOf, ByteSerializedLenOf, PartialEq, Clone, Copy)]);
    impl LoginRejectReason {
        #[inline(always)]
        pub fn is_not_authorized(&self) -> bool {
            self.0 == b'A'
        }
        #[inline(always)]
        pub fn is_session_not_available(&self) -> bool {
            self.0 == b'S'
        }
        #[inline(always)]
        pub fn not_authorized() -> Self {
            LoginRejectReason::new(b'A')
        }
        #[inline(always)]
        pub fn session_not_available() -> Self {
            LoginRejectReason::new(b'S')
        }
    }
    impl serde::Serialize for LoginRejectReason {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            if self.is_not_authorized() {
                serializer.serialize_str("NOT_AUTHORIZED")
            } else if self.is_session_not_available() {
                serializer.serialize_str("SESSION_NOT_AVAILABLE")
            } else {
                serializer.serialize_str("UNKNOWN")
            }
        }
    }
    impl<'de> serde::Deserialize<'de> for LoginRejectReason {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            let value = String::deserialize(deserializer)?.to_uppercase();
            match value.as_str() {
                "NOT_AUTHORIZED" | "A" => Ok(Self::not_authorized()),
                "SESSION_NOT_AVAILABLE" | "S" => Ok(Self::session_not_available()),
                _ => panic!("Unknown value for {}: {}", short_type_name::<Self>(), value),
            }
        }
    }

    #[cfg(test)]
    mod test_login_reject_reason {
        use super::LoginRejectReason;
        use links_core::unittest::setup;
        use log::{info, LevelFilter};
        use serde_json::{from_str, to_string};

        #[test]
        fn test_login_reject_reason() {
            setup::log::configure_compact(LevelFilter::Info);

            let msg_inp = LoginRejectReason::not_authorized();
            log::info!("msg_inp:? {:?}", msg_inp);
            let json_out = to_string(&msg_inp).unwrap();
            info!("json_out: {}", json_out);
            assert_eq!(json_out, r#""NOT_AUTHORIZED""#);

            // acceptable alternatives
            for (i, pass_json) in vec![r#""NOT_AUTHORIZED""#, r#""A""#].iter().enumerate() {
                info!("=========== {} ===========", i + 1);
                info!("pass_json: {}", pass_json);
                let msg_out: LoginRejectReason = from_str(pass_json).unwrap();
                info!("msg_out:? {:?}", msg_out);
                assert_eq!(msg_inp, msg_out);
            }

            let msg_inp = LoginRejectReason::session_not_available();
            log::info!("msg_inp:? {:?}", msg_inp);
            let json_out = to_string(&msg_inp).unwrap();
            info!("json_out: {}", json_out);
            assert_eq!(json_out, r#""SESSION_NOT_AVAILABLE""#);

            // acceptable alternatives
            for (i, pass_json) in vec![r#""SESSION_NOT_AVAILABLE""#, r#""S""#].iter().enumerate() {
                info!("=========== {} ===========", i + 1);
                info!("pass_json: {}", pass_json);
                let msg_out: LoginRejectReason = from_str(pass_json).unwrap();
                info!("msg_out:? {:?}", msg_out);
                assert_eq!(msg_inp, msg_out);
            }
        }
    }
}

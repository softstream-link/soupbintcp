pub use soupbintcp_field_types::*;
pub use soupbintcp_packet_types::*;

use byteserde_derive::{ByteDeserializeSlice, ByteSerializeStack, ByteSerializedLenOf, ByteSerializedSizeOf};

pub mod soupbintcp_packet_types {
    use super::*;
    use byteserde_types::const_char_ascii;
    const_char_ascii!(PacketTypeCltHeartbeat, b'R', derive(ByteSerializeStack, ByteSerializedSizeOf, ByteSerializedLenOf, PartialEq, Clone, Copy));
    const_char_ascii!(PacketTypeSvcHeartbeat, b'H', derive(ByteSerializeStack, ByteSerializedSizeOf, ByteSerializedLenOf, PartialEq, Clone, Copy));
    const_char_ascii!(PacketTypeDebug, b'+', derive(ByteSerializeStack, ByteSerializedSizeOf, ByteSerializedLenOf, PartialEq, Clone, Copy));
    const_char_ascii!(PacketTypeEndOfSession, b'Z', derive(ByteSerializeStack, ByteSerializedSizeOf, ByteSerializedLenOf, PartialEq, Clone, Copy));
    const_char_ascii!(PacketTypeLoginAccepted, b'A', derive(ByteSerializeStack, ByteSerializedSizeOf, ByteSerializedLenOf, PartialEq, Clone, Copy));
    const_char_ascii!(PacketTypeLoginRejected, b'J', derive(ByteSerializeStack, ByteSerializedSizeOf, ByteSerializedLenOf, PartialEq, Clone, Copy));
    const_char_ascii!(PacketTypeLoginRequest, b'L', derive(ByteSerializeStack, ByteSerializedSizeOf, ByteSerializedLenOf, PartialEq, Clone, Copy));
    const_char_ascii!(PacketTypeLogoutRequest, b'O', derive(ByteSerializeStack, ByteSerializedSizeOf, ByteSerializedLenOf, PartialEq, Clone, Copy));
    const_char_ascii!(PacketTypeSequencedData, b'S', derive(ByteSerializeStack, ByteSerializedSizeOf, ByteSerializedLenOf, PartialEq, Clone, Copy));
    const_char_ascii!(PacketTypeUnsequencedData, b'U', derive(ByteSerializeStack, ByteSerializedSizeOf, ByteSerializedLenOf, PartialEq, Clone, Copy));
}

pub mod soupbintcp_field_types {
    use super::*;
    use byteserde_types::{char_ascii, string_ascii_fixed};

    string_ascii_fixed!(SessionId, 10, b' ', true, derive(ByteSerializeStack, ByteDeserializeSlice, ByteSerializedSizeOf, ByteSerializedLenOf, PartialEq, Clone, Copy));
    impl Default for SessionId {
        fn default() -> Self {
            // all banks to log into the currently active session
            b"          ".into()
        }
    }

    // TODO add docs https://stackoverflow.com/questions/33999341/generating-documentation-in-macros
    string_ascii_fixed!(SequenceNumber, 20, b' ', true, derive(ByteSerializeStack, ByteDeserializeSlice, ByteSerializedSizeOf, ByteSerializedLenOf, PartialEq, Clone, Copy));
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

    string_ascii_fixed!(TimeoutMs, 5, b' ', true, derive(ByteSerializeStack, ByteDeserializeSlice, ByteSerializedSizeOf, ByteSerializedLenOf, PartialEq, Clone, Copy));
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
    impl Default for TimeoutMs {
        fn default() -> Self {
            1000u16.into()
        }
    }
    #[cfg(test)]
    mod test {
        use super::TimeoutMs;
        use links_core::unittest::setup;
        use log::info;

        #[test]
        fn test_sequence_number() {
            setup::log::configure();
            let t = TimeoutMs::default();
            let millis_u64: u64 = t.into();
            info!("millis_u64: {}", millis_u64);
            assert_eq!(millis_u64, 1000);
            let millis_u16: u16 = t.into();
            info!("millis_u16: {}", millis_u16);
            assert_eq!(millis_u16, 1000);
        }
    }

    string_ascii_fixed!(UserName, 6, b' ', true, derive(ByteSerializeStack, ByteDeserializeSlice, ByteSerializedSizeOf, ByteSerializedLenOf, PartialEq, Clone, Copy));
    string_ascii_fixed!(Password, 10, b' ', true, derive(ByteSerializeStack, ByteDeserializeSlice, ByteSerializedSizeOf, ByteSerializedLenOf, PartialEq, Clone, Copy));
    char_ascii!(LoginRejectReason, derive(ByteSerializeStack, ByteDeserializeSlice, ByteSerializedSizeOf, ByteSerializedLenOf, PartialEq, Clone, Copy));
    impl LoginRejectReason {
        #[inline(always)]
        pub fn is_not_authorized(&self) -> bool {
            if self.0 == b'A' {
                true
            } else {
                false
            }
        }
        #[inline(always)]
        pub fn is_session_not_available(&self) -> bool {
            if self.0 == b'S' {
                true
            } else {
                false
            }
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
}

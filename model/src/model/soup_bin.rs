use crate::prelude::*;
use byteserde_derive::{ByteDeserializeSlice, ByteSerializeStack, ByteSerializedLenOf};
use derive_more::TryInto;
use serde::{Deserialize, Serialize};

use super::unsequenced_data::UPayload;

pub const SOUPBINTCP_MAX_FRAME_SIZE_EXCLUDING_PAYLOAD_DEBUG: usize = 54;

#[rustfmt::skip] // rustfmt bug: removes UPayload::<P> variant into UPayload<P> variant which is invalid syntax
#[derive(ByteSerializeStack, ByteDeserializeSlice, ByteSerializedLenOf, Serialize, Deserialize, PartialEq, Clone, Debug, TryInto)]
#[byteserde(peek(2, 1))]
pub enum CltSoupBinTcpMsg<Payload: SoupBinTcpPayload<Payload>> {
    #[byteserde(eq(PacketTypeUnsequencedData::as_slice()))]
    UPayload(UPayload::<Payload>),
    #[byteserde(eq(PacketTypeSequencedData::as_slice()))]
    SPayload(SPayload::<Payload>),
    #[byteserde(eq(PacketTypeCltHeartbeat::as_slice()))]
    HBeat(CltHeartbeat),
    #[byteserde(eq(PacketTypeDebug::as_slice()))]
    Dbg(crate::model::debug::Debug),
    #[byteserde(eq(PacketTypeLoginRequest::as_slice()))]
    LoginRequest(LoginRequest),
    #[byteserde(eq(PacketTypeLogoutRequest::as_slice()))]
    LogoutRequest(LogoutRequest),
}
#[rustfmt::skip]
impl<Payload: SoupBinTcpPayload<Payload>> CltSoupBinTcpMsg<Payload> {
    #[inline(always)]
    pub const fn hbeat() -> Self { Self::HBeat(CltHeartbeat::new()) }
    #[inline(always)]
    pub fn login(username: UserName, password: Password, session_id: SessionId, sequence_number: SequenceNumber, hbeat_timeout_ms: TimeoutMs) -> Self { LoginRequest::new(username, password, session_id, sequence_number, hbeat_timeout_ms).into() }
    #[inline(always)]
    pub const fn logout() -> Self { Self::LogoutRequest(LogoutRequest::new()) }
    #[inline(always)]
    pub fn dbg(text: &[u8]) -> Self { Debug::new(text).into() }
    #[inline(always)]
    pub fn sdata(payload: Payload) -> Self { CltSoupBinTcpMsg::SPayload(SPayload::new(payload)) }
    #[inline(always)]
    pub fn udata(payload: Payload) -> Self { CltSoupBinTcpMsg::UPayload(UPayload::new(payload)) }
}
mod from_clt_msgs {
    use super::*;
    impl<P: SoupBinTcpPayload<P>> From<CltHeartbeat> for CltSoupBinTcpMsg<P> {
        #[inline(always)]
        fn from(payload: CltHeartbeat) -> Self {
            CltSoupBinTcpMsg::HBeat(payload)
        }
    }
    impl<P: SoupBinTcpPayload<P>> From<Debug> for CltSoupBinTcpMsg<P> {
        #[inline(always)]
        fn from(payload: Debug) -> Self {
            CltSoupBinTcpMsg::Dbg(payload)
        }
    }
    impl<P: SoupBinTcpPayload<P>> From<LoginRequest> for CltSoupBinTcpMsg<P> {
        #[inline(always)]
        fn from(payload: LoginRequest) -> Self {
            CltSoupBinTcpMsg::LoginRequest(payload)
        }
    }
    impl<P: SoupBinTcpPayload<P>> From<LogoutRequest> for CltSoupBinTcpMsg<P> {
        #[inline(always)]
        fn from(payload: LogoutRequest) -> Self {
            CltSoupBinTcpMsg::LogoutRequest(payload)
        }
    }
}

#[rustfmt::skip] // rustfmt bug: removes UPayload::<P> variant into UPayload<P> variant which is invalid syntax
#[derive(ByteSerializeStack, ByteDeserializeSlice, ByteSerializedLenOf, Serialize, Deserialize, PartialEq, Clone, Debug, TryInto)]
#[byteserde(peek(2, 1))]
pub enum SvcSoupBinTcpMsg<Payload: SoupBinTcpPayload<Payload>> {
    #[byteserde(eq(PacketTypeSvcHeartbeat::as_slice()))]
    HBeat(SvcHeartbeat),
    #[byteserde(eq(PacketTypeDebug::as_slice()))]
    Dbg(crate::model::debug::Debug),
    #[byteserde(eq(PacketTypeLoginAccepted::as_slice()))]
    LoginAccepted(LoginAccepted),
    #[byteserde(eq(PacketTypeLoginRejected::as_slice()))]
    LoginRejected(LoginRejected),
    #[byteserde(eq(PacketTypeEndOfSession::as_slice()))]
    EndOfSession(EndOfSession),
    #[byteserde(eq(PacketTypeUnsequencedData::as_slice()))]
    UPayload(UPayload::<Payload>),
    #[byteserde(eq(PacketTypeSequencedData::as_slice()))]
    SPayload(SPayload::<Payload>),
}

impl<Payload: SoupBinTcpPayload<Payload>> SvcSoupBinTcpMsg<Payload> {
    #[inline(always)]
    pub const fn hbeat() -> Self {
        Self::HBeat(SvcHeartbeat::new())
    }
    #[inline(always)]
    pub fn dbg(text: &[u8]) -> Self {
        Debug::new(text).into()
    }
    #[inline(always)]
    pub fn login_acc(session_id: SessionId, sequence_number: SequenceNumber) -> Self {
        LoginAccepted::new(session_id, sequence_number).into()
    }
    #[inline(always)]
    pub fn login_rej_not_auth() -> Self {
        LoginRejected::not_authorized().into()
    }
    #[inline(always)]
    pub fn login_rej_ses_not_avail() -> Self {
        LoginRejected::session_not_available().into()
    }
    #[inline(always)]
    pub const fn end() -> Self {
        Self::EndOfSession(EndOfSession::new())
    }
    #[inline(always)]
    pub fn sdata(payload: Payload) -> Self {
        Self::SPayload(SPayload::new(payload))
    }
    #[inline(always)]
    pub fn udata(payload: Payload) -> Self {
        Self::UPayload(UPayload::new(payload))
    }
}
mod from_svc_msgs {
    use super::*;
    impl<P: SoupBinTcpPayload<P>> From<SvcHeartbeat> for SvcSoupBinTcpMsg<P> {
        #[inline(always)]
        fn from(payload: SvcHeartbeat) -> Self {
            SvcSoupBinTcpMsg::HBeat(payload)
        }
    }
    impl<P: SoupBinTcpPayload<P>> From<Debug> for SvcSoupBinTcpMsg<P> {
        #[inline(always)]
        fn from(payload: Debug) -> Self {
            SvcSoupBinTcpMsg::Dbg(payload)
        }
    }
    impl<P: SoupBinTcpPayload<P>> From<LoginAccepted> for SvcSoupBinTcpMsg<P> {
        #[inline(always)]
        fn from(payload: LoginAccepted) -> Self {
            SvcSoupBinTcpMsg::LoginAccepted(payload)
        }
    }
    impl<P: SoupBinTcpPayload<P>> From<LoginRejected> for SvcSoupBinTcpMsg<P> {
        #[inline(always)]
        fn from(payload: LoginRejected) -> Self {
            SvcSoupBinTcpMsg::LoginRejected(payload)
        }
    }
    impl<P: SoupBinTcpPayload<P>> From<EndOfSession> for SvcSoupBinTcpMsg<P> {
        #[inline(always)]
        fn from(payload: EndOfSession) -> Self {
            SvcSoupBinTcpMsg::EndOfSession(payload)
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, TryInto)]
pub enum UniSoupBinTcpMsg<CltP: SoupBinTcpPayload<CltP>, SvcP: SoupBinTcpPayload<SvcP>> {
    Clt(CltSoupBinTcpMsg<CltP>),
    Svc(SvcSoupBinTcpMsg<SvcP>),
}
impl<CltP: SoupBinTcpPayload<CltP>, SvcP: SoupBinTcpPayload<SvcP>> UniSoupBinTcpMsg<CltP, SvcP> {
    pub fn unwrap_clt(self) -> CltSoupBinTcpMsg<CltP> {
        match self {
            UniSoupBinTcpMsg::Clt(msg) => msg,
            _ => panic!("SoupBinTcp message is not Clt, instead it is: {:?}", self),
        }
    }
    pub fn unwrap_svc(self) -> SvcSoupBinTcpMsg<SvcP> {
        match self {
            UniSoupBinTcpMsg::Svc(msg) => msg,
            _ => panic!("SoupBinTcp message is not Svc, instead it is: {:?}", self),
        }
    }
    pub fn unwrap_clt_upayload(self) -> CltP {
        match self {
            UniSoupBinTcpMsg::Clt(CltSoupBinTcpMsg::UPayload(UPayload { payload, .. })) => payload,
            _ => panic!("SoupBinTcp message is not Clt with UPayload, instead it is: {:?}", self),
        }
    }
    pub fn unwrap_svc_spayload(&self) -> &SvcP {
        match self {
            UniSoupBinTcpMsg::Svc(SvcSoupBinTcpMsg::SPayload(SPayload { payload, .. })) => payload,
            _ => panic!("SoupBinTcp message is not Svc with SPayload, instead it is: {:?}", self),
        }
    }
}
impl<CltP: SoupBinTcpPayload<CltP>, SvcP: SoupBinTcpPayload<SvcP>> From<&CltSoupBinTcpMsg<CltP>> for UniSoupBinTcpMsg<CltP, SvcP> {
    fn from(value: &CltSoupBinTcpMsg<CltP>) -> Self {
        UniSoupBinTcpMsg::Clt(value.clone())
    }
}
impl<CltP: SoupBinTcpPayload<CltP>, SvcP: SoupBinTcpPayload<SvcP>> From<&SvcSoupBinTcpMsg<SvcP>> for UniSoupBinTcpMsg<CltP, SvcP> {
    fn from(value: &SvcSoupBinTcpMsg<SvcP>) -> Self {
        UniSoupBinTcpMsg::Svc(value.clone())
    }
}

#[cfg(test)]
#[cfg(feature = "unittest")]
mod test {

    use crate::{prelude::*, unittest::setup::model::*};
    use byteserde::prelude::*;
    use links_core::unittest::setup;
    use log::{info, LevelFilter};
    use serde_json::{from_str, to_string};

    #[test]
    fn test_soupbintcp_clt_byteserde() {
        setup::log::configure_compact(LevelFilter::Info);
        let mut ser = ByteSerializerStack::<1024>::default();
        let msg_inp = clt_msgs_default();

        for msg in msg_inp.iter() {
            info!("msg_inp: {:?}", msg);
            let _ = ser.serialize(msg).unwrap();
        }
        info!("ser: {:#x}", ser);

        let mut des = ByteDeserializerSlice::new(ser.as_slice());
        let mut msg_out = vec![];
        while !des.is_empty() {
            let msg = CltSoupBinTcpMsg::<SamplePayload>::byte_deserialize(&mut des).unwrap();
            info!("msg_out: {:?}", msg);
            msg_out.push(msg);
        }
        assert_eq!(msg_inp, msg_out);
    }

    #[test]
    fn test_soupbintcp_clt_serde() {
        setup::log::configure_compact(LevelFilter::Info);

        let msgs_inp = clt_msgs_default::<SamplePayload>();
        let mut msgs_out = vec![];
        for msg_inp in msgs_inp.iter() {
            // info!("msg_inp: {:?}", msg_inp);
            let json_out = to_string(msg_inp).unwrap();
            info!("json_out: {}", json_out);
            let msg_out = from_str(&json_out).unwrap();
            // info!("msg_out: {:?}", msg_out);
            msgs_out.push(msg_out);
        }
        assert_eq!(msgs_inp, msgs_out);
    }

    #[test]
    fn test_soupbintcp_svc_byteserde() {
        setup::log::configure_compact(LevelFilter::Info);
        let mut ser = ByteSerializerStack::<1024>::default();
        let msg_inp = svc_msgs_default();

        for msg in msg_inp.iter() {
            info!("msg_inp: {:?}", msg);
            let _ = ser.serialize(msg).unwrap();
        }
        info!("ser: {:#x}", ser);

        let mut des = ByteDeserializerSlice::new(ser.as_slice());
        let mut msg_out = vec![];
        while !des.is_empty() {
            let msg = SvcSoupBinTcpMsg::<SamplePayload>::byte_deserialize(&mut des).unwrap();
            info!("msg_out: {:?}", msg);
            msg_out.push(msg);
        }
        assert_eq!(msg_inp, msg_out);
    }

    #[test]
    fn test_soupbintcp_svc_serde() {
        setup::log::configure_compact(LevelFilter::Info);

        let msgs_inp = svc_msgs_default::<SamplePayload>();
        let mut msgs_out = vec![];
        for msg_inp in msgs_inp.iter() {
            // info!("msg_inp: {:?}", msg_inp);
            let json_out = to_string(msg_inp).unwrap();
            info!("json_out: {}", json_out);
            let msg_out = from_str(&json_out).unwrap();
            // info!("msg_out: {:?}", msg_out);
            msgs_out.push(msg_out);
        }
        assert_eq!(msgs_inp, msgs_out);
    }

    #[test]
    fn test_soupbintcp_msg_serde() {
        setup::log::configure_compact(LevelFilter::Info);
        let mut msgs_inp: Vec<UniSoupBinTcpMsg<SamplePayload, SamplePayload>> = vec![];
        let msgs_clt = clt_msgs_default();
        let msgs_svc = svc_msgs_default();
        for msg in msgs_clt {
            msgs_inp.push((&msg).into());
        }
        for msg in msgs_svc {
            msgs_inp.push((&msg).into());
        }
        let mut msgs_out = vec![];
        for msg in msgs_inp.iter() {
            // info!("msg_inp: {:?}", msg);
            let json_out = to_string(msg).unwrap();
            info!("json_out: {}", json_out);
            let msg_out: UniSoupBinTcpMsg<SamplePayload, SamplePayload> = from_str(&json_out).unwrap();
            // info!("msg_out: {:?}", msg_out);
            msgs_out.push(msg_out);
        }
        assert_eq!(msgs_inp, msgs_out);
    }

    #[test]
    fn test_soupbintcp_max_frame_size() {
        setup::log::configure_compact(LevelFilter::Info);
        let msg_inp_clt = clt_msgs_default::<Nil>().into_iter().map(|msg| (msg.byte_len(), msg)).collect::<Vec<_>>();
        let msg_inp_svc = svc_msgs_default::<Nil>().into_iter().map(|msg| (msg.byte_len(), msg)).collect::<Vec<_>>();
        for (byte_len, msg) in msg_inp_clt.iter() {
            info!("byte_len: {:>3}, msg:? {:?} ", byte_len, msg);
        }
        for (byte_len, msg) in msg_inp_svc.iter() {
            info!("byte_len: {:>3}, msg:? {:?} ", byte_len, msg);
        }
        let max_frame_size_no_payload = std::cmp::max(msg_inp_clt.iter().map(|(len, _)| *len).max().unwrap(), msg_inp_svc.iter().map(|(len, _)| *len).max().unwrap());
        info!("max_frame_size_no_payload: {}", max_frame_size_no_payload);
        assert_eq!(max_frame_size_no_payload, SOUPBINTCP_MAX_FRAME_SIZE_EXCLUDING_PAYLOAD_DEBUG)
    }
}

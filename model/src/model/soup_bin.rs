use crate::prelude::*;
use byteserde_derive::{ByteDeserializeSlice, ByteSerializeStack, ByteSerializedLenOf};
use derive_more::TryInto;
use serde::{Deserialize, Serialize};
// use std::fmt;

use super::unsequenced_data::UPayload;

pub const SOUPBINTCP_MAX_FRAME_SIZE_EXCLUDING_PAYLOAD_DEBUG: usize = 54;

#[rustfmt::skip]
#[derive(ByteSerializeStack, ByteDeserializeSlice, ByteSerializedLenOf, Serialize, Deserialize, PartialEq, Clone, Debug, TryInto)]
#[byteserde(peek(2, 1))]
pub enum CltSoupBinTcpMsg<P: SoupBinTcpPayload<P>> {
    #[byteserde(eq(PacketTypeUnsequencedData::as_slice()))]
    UPayload(UPayload::<P>),
    #[byteserde(eq(PacketTypeSequencedData::as_slice()))]
    SPayload(SPayload::<P>),
    #[byteserde(eq(PacketTypeCltHeartbeat::as_slice()))]
    Hbeat(CltHeartbeat),
    #[byteserde(eq(PacketTypeDebug::as_slice()))]
    Dbg(crate::model::debug::Debug),
    #[byteserde(eq(PacketTypeLoginRequest::as_slice()))]
    Login(LoginRequest),
    #[byteserde(eq(PacketTypeLogoutRequest::as_slice()))]
    Logout(LogoutRequest),
}
#[rustfmt::skip]
impl<P: SoupBinTcpPayload<P>> CltSoupBinTcpMsg<P> {
    #[inline(always)]
    pub fn login(username: UserName, password: Password, session_id: SessionId, sequence_number: SequenceNumber, hbeat_timeout_ms: TimeoutMs) -> Self { 
        Self::Login( LoginRequest::new(username, password, session_id, sequence_number, hbeat_timeout_ms)) 
    }
    #[inline(always)]
    pub fn logout() -> Self { CltSoupBinTcpMsg::Logout(LogoutRequest::default()) }
    #[inline(always)]
    pub fn hbeat() -> Self { CltSoupBinTcpMsg::Hbeat(CltHeartbeat::default()) }
    #[inline(always)]
    pub fn dbg(text: &[u8]) -> Self { CltSoupBinTcpMsg::Dbg(Debug::new(text)) }
    #[inline(always)]
    pub fn sdata(payload: P) -> Self { CltSoupBinTcpMsg::SPayload(SPayload::new(payload)) }
    #[inline(always)]
    pub fn udata(payload: P) -> Self { CltSoupBinTcpMsg::UPayload(UPayload::new(payload)) }
}

#[rustfmt::skip]
#[derive(ByteSerializeStack, ByteDeserializeSlice, ByteSerializedLenOf, Serialize, Deserialize, PartialEq, Clone, Debug, TryInto)]
#[byteserde(peek(2, 1))]
pub enum SvcSoupBinTcpMsg<P: SoupBinTcpPayload<P>>{
    #[byteserde(eq(PacketTypeUnsequencedData::as_slice()))]
    UPayload(UPayload::<P>),
    #[byteserde(eq(PacketTypeSequencedData::as_slice()))]
    SPayload(SPayload::<P>),
    #[byteserde(eq(PacketTypeSvcHeartbeat::as_slice()))]
    Hbeat(SvcHeartbeat),
    #[byteserde(eq(PacketTypeDebug::as_slice()))]
    Dbg(crate::model::debug::Debug),
    #[byteserde(eq(PacketTypeEndOfSession::as_slice()))]
    EndOfSession(EndOfSession),
    #[byteserde(eq(PacketTypeLoginAccepted::as_slice()))]
    LoginAccepted(LoginAccepted),
    #[byteserde(eq(PacketTypeLoginRejected::as_slice()))]
    LoginRejected(LoginRejected),
}
#[rustfmt::skip]
impl<P: SoupBinTcpPayload<P>> SvcSoupBinTcpMsg<P> {
    pub fn end() -> Self { Self::EndOfSession(EndOfSession::default()) }
    pub fn login_acc(session_id: SessionId, sequence_number: SequenceNumber) -> Self { Self::LoginAccepted(LoginAccepted::new(session_id, sequence_number)) }
    pub fn login_rej_not_auth() -> Self { Self::LoginRejected(LoginRejected::not_authorized()) }
    pub fn login_rej_ses_not_avail() -> Self { Self::LoginRejected(LoginRejected::session_not_available()) }
    pub fn hbeat() -> Self { Self::Hbeat(SvcHeartbeat::default()) }
    pub fn dbg(text: &[u8]) -> Self { Self::Dbg(Debug::new(text)) }
    pub fn sdata(payload: P) -> Self { Self::SPayload(SPayload::new(payload)) }
    pub fn udata(payload: P) -> Self { Self::UPayload(UPayload::new(payload)) }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, TryInto)]
pub enum SoupBinTcpMsg<CltP, SvcP>
where
    CltP: SoupBinTcpPayload<CltP>,
    SvcP: SoupBinTcpPayload<SvcP>,
{
    Clt(CltSoupBinTcpMsg<CltP>),
    Svc(SvcSoupBinTcpMsg<SvcP>),
}
impl<CltP, SvcP> SoupBinTcpMsg<CltP, SvcP>
where
    CltP: SoupBinTcpPayload<CltP>,
    SvcP: SoupBinTcpPayload<SvcP>,
{
    pub fn unwrap_clt_u(&self) -> &CltP {
        match self {
            SoupBinTcpMsg::Clt(CltSoupBinTcpMsg::UPayload(UPayload { body, .. })) => body,
            _ => panic!("SoupBinTcp message is not Clt and/or UPayload, instead it is: {:?}", self),
        }
    }
    pub fn unwrap_svc_u(&self) -> &SvcP {
        match self {
            SoupBinTcpMsg::Svc(SvcSoupBinTcpMsg::UPayload(UPayload { body, .. })) => body,
            _ => panic!("SoupBinTcp message is not Svc and/or UPayload, instead it is: {:?}", self),
        }
    }
}
impl<CltP, SvcP> From<CltSoupBinTcpMsg<CltP>> for SoupBinTcpMsg<CltP, SvcP>
where
    CltP: SoupBinTcpPayload<CltP>,
    SvcP: SoupBinTcpPayload<SvcP>,
{
    fn from(value: CltSoupBinTcpMsg<CltP>) -> Self {
        SoupBinTcpMsg::Clt(value)
    }
}
impl<CltP, SvcP> From<SvcSoupBinTcpMsg<SvcP>> for SoupBinTcpMsg<CltP, SvcP>
where
    CltP: SoupBinTcpPayload<CltP>,
    SvcP: SoupBinTcpPayload<SvcP>,
{
    fn from(value: SvcSoupBinTcpMsg<SvcP>) -> Self {
        SoupBinTcpMsg::Svc(value)
    }
}

#[cfg(test)]
#[cfg(feature = "unittest")]
mod test {

    use crate::{prelude::*, unittest::setup::model::*};
    use byteserde::prelude::*;
    use links_core::unittest::setup;
    use log::info;
    use serde_json::{from_str, to_string};

    #[test]
    fn test_soupbintcp_clt_byteserde() {
        setup::log::configure_compact();
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
        setup::log::configure_compact();

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
        setup::log::configure_compact();
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
        setup::log::configure_compact();

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
        setup::log::configure_compact();
        let mut msgs_inp: Vec<SoupBinTcpMsg<SamplePayload, SamplePayload>> = vec![];
        let msgs_clt = clt_msgs_default();
        let msgs_svc = svc_msgs_default();
        for msg in msgs_clt {
            msgs_inp.push(msg.into());
        }
        for msg in msgs_svc {
            msgs_inp.push(msg.into());
        }
        let mut msgs_out = vec![];
        for msg in msgs_inp.iter() {
            // info!("msg_inp: {:?}", msg);
            let json_out = to_string(msg).unwrap();
            info!("json_out: {}", json_out);
            let msg_out: SoupBinTcpMsg<SamplePayload, SamplePayload> = from_str(&json_out).unwrap();
            // info!("msg_out: {:?}", msg_out);
            msgs_out.push(msg_out);
        }
        assert_eq!(msgs_inp, msgs_out);
    }

    #[test]
    fn test_soupbintcp_max_frame_size() {
        setup::log::configure_compact();
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

use std::{fmt::Debug, io::Error, marker::PhantomData};

use crate::prelude::*;
use byteserde::prelude::{from_slice, to_bytes_stack};

#[rustfmt::skip]

/// Performs two tasks
///  * Divides [bytes::BytesMut] into frames and deserializes into a [SvcSoupBinTcpMsg] type
///  * Takes [CltSoupBinTcpMsg] type and serializes into byte array 
#[derive(Debug)]
pub struct CltSoupBinTcpMessenger<RecvP: SoupBinTcpPayload<RecvP>, SendP: SoupBinTcpPayload<SendP>> {
    phantom: PhantomData<(RecvP, SendP)>,
}
impl<RecvP: SoupBinTcpPayload<RecvP>, SendP: SoupBinTcpPayload<SendP>> Framer
    for CltSoupBinTcpMessenger<RecvP, SendP>
{
    #[inline(always)]
    fn get_frame_length(bytes: &mut bytes::BytesMut) -> Option<usize> {
        SoupBinTcpFramer::get_frame_length(bytes)
    }
}
impl<RecvP: SoupBinTcpPayload<RecvP>, SendP: SoupBinTcpPayload<SendP>> Messenger
    for CltSoupBinTcpMessenger<RecvP, SendP>
{
    type RecvT = SvcSoupBinTcpMsg<RecvP>;
    type SendT = CltSoupBinTcpMsg<SendP>;

    #[inline(always)]
    fn serialize<const MAX_MSG_SIZE: usize>(
        msg: &Self::SendT,
    ) -> Result<([u8; MAX_MSG_SIZE], usize), std::io::Error> {
        match to_bytes_stack::<MAX_MSG_SIZE, Self::SendT>(msg) {
            Ok(res) => Ok(res),
            Err(e) => Err(Error::new(std::io::ErrorKind::Other, e)),
        }
    }

    #[inline(always)]
    fn deserialize(frame: &[u8]) -> Result<Self::RecvT, Error> {
        match from_slice::<Self::RecvT>(frame) {
            Ok(res) => Ok(res),
            Err(e) => Err(Error::new(std::io::ErrorKind::Other, e)),
        }
    }
}

/// Performs two tasks
///  * Divides [bytes::BytesMut] into frames and deserializes into a [CltSoupBinTcpMsg] type
///  * Takes [SvcSoupBinTcpMsg] type and serializes into byte array
#[derive(Debug)]
pub struct SvcSoupBinTcpMessenger<RecvP: SoupBinTcpPayload<RecvP>, SendP: SoupBinTcpPayload<SendP>>
{
    phantom: PhantomData<(RecvP, SendP)>,
}
impl<RecvP: SoupBinTcpPayload<RecvP>, SendP: SoupBinTcpPayload<SendP>> Framer
    for SvcSoupBinTcpMessenger<RecvP, SendP>
{
    #[inline(always)]
    fn get_frame_length(bytes: &mut bytes::BytesMut) -> Option<usize> {
        SoupBinTcpFramer::get_frame_length(bytes)
    }
}
impl<RecvP: SoupBinTcpPayload<RecvP>, SendP: SoupBinTcpPayload<SendP>> Messenger
    for SvcSoupBinTcpMessenger<RecvP, SendP>
{
    type RecvT = CltSoupBinTcpMsg<RecvP>;
    type SendT = SvcSoupBinTcpMsg<RecvP>;

    #[inline(always)]
    fn serialize<const MAX_MSG_SIZE: usize>(
        msg: &Self::SendT,
    ) -> Result<([u8; MAX_MSG_SIZE], usize), std::io::Error> {
        match to_bytes_stack::<MAX_MSG_SIZE, Self::SendT>(msg) {
            Ok(res) => Ok(res),
            Err(e) => Err(Error::new(std::io::ErrorKind::Other, e)),
        }
    }

    #[inline(always)]
    fn deserialize(frame: &[u8]) -> Result<Self::RecvT, Error> {
        match from_slice::<Self::RecvT>(frame) {
            Ok(res) => Ok(res),
            Err(e) => Err(Error::new(std::io::ErrorKind::Other, e)),
        }
    }
}

#[cfg(test)]
#[cfg(feature = "unittest")]
mod test {

    use crate::prelude::*;
    use bytes::{BufMut, BytesMut};
    use byteserde::prelude::*;
    use log::info;

    use soupbintcp_model::unittest::setup::{
        self,
        model::{clt_msgs_default, svc_msgs_default},
    };
    

    type CltMessenger = CltSoupBinTcpMessenger<SamplePayload, SamplePayload>;
    type SvcMessenger = SvcSoupBinTcpMessenger<SamplePayload, SamplePayload>;
    #[test]
    fn test_soup_bin_clt_send_messenger() {
        setup::log::configure();

        const CAP: usize = 1024;
        let mut ser = ByteSerializerStack::<CAP>::default();
        let msg_inp = clt_msgs_default();
        for msg in msg_inp.iter() {
            info!("msg_inp {:?}", msg);
            let (buf, size) = CltMessenger::serialize::<CAP>(msg).unwrap();
            ser.serialize_bytes_slice(&buf[..size]).unwrap();
        }
        info!("ser: {:#x}", ser);

        let mut bytes = BytesMut::with_capacity(CAP);
        bytes.put_slice(ser.as_slice());

        let mut msg_out: Vec<CltSoupBinTcpMsg<_>> = vec![];
        loop {
            let frame = SvcMessenger::get_frame(&mut bytes);
            match frame {
                Some(frame) => {
                    let msg = SvcMessenger::deserialize(&frame[..]).unwrap();
                    info!("msg_out {:?}", msg);
                    msg_out.push(msg);
                }
                None => break,
            }
        }
        assert_eq!(msg_inp, msg_out);
    }

    #[test]
    fn test_soup_bin_svc_send_messenger() {
        setup::log::configure();
        const CAP: usize = 1024;
        let mut ser = ByteSerializerStack::<CAP>::default();
        let msg_inp = svc_msgs_default();
        for msg in msg_inp.iter() {
            info!("msg_inp {:?}", msg);
            let (buf, size) = SvcMessenger::serialize::<CAP>(msg).unwrap();
            ser.serialize_bytes_slice(&buf[..size]).unwrap();
        }
        info!("ser: {:#x}", ser);

        let mut bytes = BytesMut::with_capacity(CAP);
        bytes.put_slice(ser.as_slice());

        let mut msg_out: Vec<SvcSoupBinTcpMsg<_>> = vec![];
        loop {
            let frame = CltMessenger::get_frame(&mut bytes);
            match frame {
                Some(frame) => {
                    let msg = CltMessenger::deserialize(&frame[..]).unwrap();
                    info!("msg_out {:?}", msg);
                    msg_out.push(msg);
                }
                None => break,
            }
        }
        assert_eq!(msg_inp, msg_out);
    }
}

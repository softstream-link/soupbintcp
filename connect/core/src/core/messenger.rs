use std::{fmt::Debug, io::Error, marker::PhantomData};

use byteserde::prelude::{from_slice, to_bytes_stack};
use links_nonblocking::prelude::{Framer, Messenger};
use soupbintcp_model::prelude::{CltSoupBinTcpMsg, SoupBinTcpPayload, SvcSoupBinTcpMsg};

use super::framer::SoupBinTcpFramer;

#[rustfmt::skip]

/// Performs two tasks
///  * Divides [bytes::BytesMut] into frames and deserializes into a [SBSvcMsg] type
///  * Takes [SBCltMsg] type and serializes into byte array 
#[derive(Debug)]
pub struct CltSoupBinTcpMessenger<P: SoupBinTcpPayload<P>> {
    phantom: PhantomData<P>,
}
impl<P: SoupBinTcpPayload<P>> Framer for CltSoupBinTcpMessenger<P> {
    #[inline(always)]
    fn get_frame_length(bytes: &mut bytes::BytesMut) -> Option<usize> {
        SoupBinTcpFramer::get_frame_length(bytes)
    }
}
impl<P: SoupBinTcpPayload<P>> Messenger for CltSoupBinTcpMessenger<P> {
    type RecvT = SvcSoupBinTcpMsg<P>;
    type SendT = CltSoupBinTcpMsg<P>;

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
///  * Divides [bytes::BytesMut] into frames and deserializes into a [SBCltcMsg] type
///  * Takes [SBSvcMsg] type and serializes into byte array
#[derive(Debug)]
pub struct SvcSoupBinTcpMessenger<P: SoupBinTcpPayload<P>> {
    phantom: PhantomData<P>,
}
impl<P: SoupBinTcpPayload<P>> Framer for SvcSoupBinTcpMessenger<P> {
    #[inline(always)]
    fn get_frame_length(bytes: &mut bytes::BytesMut) -> Option<usize> {
        SoupBinTcpFramer::get_frame_length(bytes)
    }
}
impl<P: SoupBinTcpPayload<P>> Messenger for SvcSoupBinTcpMessenger<P> {
    type RecvT = CltSoupBinTcpMsg<P>;
    type SendT = SvcSoupBinTcpMsg<P>;

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

    use super::*;
    use bytes::{BufMut, BytesMut};
    use byteserde::prelude::*;
    use log::info;

    use soupbintcp_model::unittest::setup;
    use soupbintcp_model::{
        prelude::*,
        unittest::setup::model::{clt_msgs_default, svc_msgs_default},
    };

    #[test]
    fn test_soup_bin_clt_send_messenger() {
        setup::log::configure();
        const CAP: usize = 1024;
        let mut ser = ByteSerializerStack::<CAP>::default();
        let msg_inp = clt_msgs_default();
        for msg in msg_inp.iter() {
            info!("msg_inp {:?}", msg);
            let (buf, size) = CltSoupBinTcpMessenger::serialize::<CAP>(msg).unwrap();
            ser.serialize_bytes_slice(&buf[..size]).unwrap();
        }
        info!("ser: {:#x}", ser);

        let mut bytes = BytesMut::with_capacity(CAP);
        bytes.put_slice(ser.as_slice());

        let mut msg_out: Vec<CltSoupBinTcpMsg<SamplePayload>> = vec![];
        loop {
            let frame = SvcSoupBinTcpMessenger::<SamplePayload>::get_frame(&mut bytes);
            match frame {
                Some(frame) => {
                    let msg = SvcSoupBinTcpMessenger::deserialize(&frame[..]).unwrap();
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
            let (buf, size) = SvcSoupBinTcpMessenger::serialize::<CAP>(msg).unwrap();
            ser.serialize_bytes_slice(&buf[..size]).unwrap();
        }
        info!("ser: {:#x}", ser);

        let mut bytes = BytesMut::with_capacity(CAP);
        bytes.put_slice(ser.as_slice());

        let mut msg_out: Vec<SvcSoupBinTcpMsg<SamplePayload>> = vec![];
        loop {
            let frame = CltSoupBinTcpMessenger::<SamplePayload>::get_frame(&mut bytes);
            match frame {
                Some(frame) => {
                    let msg = CltSoupBinTcpMessenger::deserialize(&frame[..]).unwrap();
                    info!("msg_out {:?}", msg);
                    msg_out.push(msg);
                }
                None => break,
            }
        }
        assert_eq!(msg_inp, msg_out);
    }
}

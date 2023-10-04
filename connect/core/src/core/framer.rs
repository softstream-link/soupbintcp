use bytes::BytesMut;
use links_nonblocking::prelude::{Framer, PacketLengthU16Framer};

pub struct SoupBinTcpFramer;

impl Framer for SoupBinTcpFramer {
    #[inline(always)]
    fn get_frame_length(bytes: &mut BytesMut) -> Option<usize> {
        PacketLengthU16Framer::<0, true, true>::get_frame_length(bytes)
    }
    // fn get_frame(bytes: &mut BytesMut) -> Option<Bytes> {
    //     // ensures there is at least 2 bytes to represent packet_length
    //     if bytes.len() < 2 {
    //         return None;
    //     }

    //     // access packet length with out advancing the cursor, below is a take of the bytes::Buf::get_u16() method
    //     let packet_length = {
    //         const SIZE: usize = std::mem::size_of::<u16>();
    //         // try to convert directly from the bytes
    //         // this Option<ret> trick is to avoid keeping a borrow on self
    //         // when advance() is called (mut borrow) and to call bytes() only once
    //         let ret = bytes
    //             // .chunk()
    //             .get(..SIZE)
    //             .map(|src| unsafe { u16::from_be_bytes(*(src as *const _ as *const [_; SIZE])) });

    //         if let Some(ret) = ret {
    //             ret
    //         } else {
    //             // if not we copy the bytes in a temp buffer then convert
    //             let mut buf = [0_u8; SIZE];
    //             let packet_length = &bytes[..SIZE];
    //             buf[0] = packet_length[0];
    //             buf[1] = packet_length[1];
    //             u16::from_be_bytes(buf)
    //         }
    //     };

    //     // ensure that there is a full frame available in the buffer
    //     let frame_length = (packet_length + 2) as usize;
    //     if bytes.len() < frame_length {
    //         None
    //     } else {
    //         let frame = bytes.split_to(frame_length);
    //         Some(frame.freeze())
    //     }
    // }
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
    fn test_soup_bin_clt_framing() {
        setup::log::configure();
        const CAP: usize = 1024;
        let mut ser = ByteSerializerStack::<CAP>::default();
        let msg_inp = clt_msgs_default();
        for msg in msg_inp.iter() {
            info!("msg_inp {:?}", msg);
            let _ = ser.serialize(msg).unwrap();
        }
        info!("ser: {:#x}", ser);

        let mut bytes = BytesMut::with_capacity(CAP);
        bytes.put_slice(ser.as_slice());

        let mut msg_out: Vec<CltSoupBinTcpMsg<SamplePayload>> = vec![];
        loop {
            let frame = SoupBinTcpFramer::get_frame(&mut bytes);
            match frame {
                Some(frame) => {
                    let des = &mut ByteDeserializerSlice::new(&frame[..]);
                    let msg = CltSoupBinTcpMsg::byte_deserialize(des).unwrap();
                    info!("msg_out {:?}", msg);
                    msg_out.push(msg);
                }
                None => break,
            }
        }
        assert_eq!(msg_inp, msg_out);
    }
    #[test]
    fn test_soup_bin_svc_framing() {
        setup::log::configure();
        const CAP: usize = 1024;
        let mut ser = ByteSerializerStack::<CAP>::default();
        let msg_inp = svc_msgs_default();
        for msg in msg_inp.iter() {
            // info!("msg_inp {:?}", msg);
            let len = ser.len();
            let _ = ser.serialize(msg).unwrap();
            info!(
                "msg.len() {}, \tser.len(): {},\tmsg_inp {:?}",
                ser.len() - len,
                ser.len(),
                msg
            );
        }
        info!("ser: {:#x}", ser);

        let mut bytes = BytesMut::with_capacity(CAP);
        bytes.put_slice(ser.as_slice());

        let mut msg_out: Vec<SvcSoupBinTcpMsg<SamplePayload>> = vec![];
        loop {
            let len = bytes.len();
            // let des = &mut ByteDeserializerSlice::new(&bytes[..]);
            // info!("des: {des:#x}");
            let frame = SoupBinTcpFramer::get_frame(&mut bytes);

            match frame {
                Some(frame) => {
                    let des = &mut ByteDeserializerSlice::new(&frame[..]);
                    let msg = SvcSoupBinTcpMsg::byte_deserialize(des).unwrap();
                    info!(
                        "frame.len(): {}, \tbyte.len(): {}, msg_out {:?}",
                        frame.len(),
                        len,
                        msg
                    );
                    msg_out.push(msg);
                }
                None => {
                    info!("frame: None, \t byte.len(): {}", len);
                    break;
                }
            }
        }
        assert_eq!(msg_inp, msg_out);
    }
}

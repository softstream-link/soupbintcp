use crate::prelude::*;
use links_nonblocking::prelude::*;
use std::{fmt::Debug, io::Error, marker::PhantomData};

/// Implements SoupBinTcp protocol for client side.
///
/// # [ProtocolCore] Features
/// * [`Self::is_connected`] - always returns `true`
///
/// # [Protocol] Features
/// * Not implemented - falls back to defaults, which are optimized away by compiler.
#[derive(Debug, Clone)]
pub struct CltSoupBinTcpProtocolManual<RecvP: SoupBinTcpPayload<RecvP>, SendP: SoupBinTcpPayload<SendP>> {
    phantom: PhantomData<(RecvP, SendP)>,
}
impl<RecvP: SoupBinTcpPayload<RecvP>, SendP: SoupBinTcpPayload<SendP>> Default for CltSoupBinTcpProtocolManual<RecvP, SendP> {
    fn default() -> Self {
        Self { phantom: PhantomData }
    }
}
impl<RecvP: SoupBinTcpPayload<RecvP>, SendP: SoupBinTcpPayload<SendP>> Framer for CltSoupBinTcpProtocolManual<RecvP, SendP> {
    #[inline(always)]
    fn get_frame_length(bytes: &bytes::BytesMut) -> Option<usize> {
        SoupBinTcpFramer::get_frame_length(bytes)
    }
}
impl<RecvP: SoupBinTcpPayload<RecvP>, SendP: SoupBinTcpPayload<SendP>> Messenger for CltSoupBinTcpProtocolManual<RecvP, SendP> {
    type RecvT = <CltSoupBinTcpMessenger<RecvP, SendP> as Messenger>::RecvT;
    type SendT = <CltSoupBinTcpMessenger<RecvP, SendP> as Messenger>::SendT;
    #[inline(always)]
    fn serialize<const MAX_MSG_SIZE: usize>(msg: &Self::SendT) -> Result<([u8; MAX_MSG_SIZE], usize), std::io::Error> {
        CltSoupBinTcpMessenger::<RecvP, SendP>::serialize(msg)
    }
    #[inline(always)]
    fn deserialize(frame: &[u8]) -> Result<Self::RecvT, Error> {
        CltSoupBinTcpMessenger::<RecvP, SendP>::deserialize(frame)
    }
}
impl<RecvP: SoupBinTcpPayload<RecvP>, SendP: SoupBinTcpPayload<SendP>> ProtocolCore for CltSoupBinTcpProtocolManual<RecvP, SendP> {
    fn is_connected(&self) -> bool {
        true
    }
}
impl<RecvP: SoupBinTcpPayload<RecvP>, SendP: SoupBinTcpPayload<SendP>> Protocol for CltSoupBinTcpProtocolManual<RecvP, SendP> {}

/// Implements SoupBinTcp protocol for server side.
///
/// # [ProtocolCore] Features
/// * [`Self::is_connected`] - always returns `true`
///
/// # [Protocol] Features
/// * Not implemented - falls back to defaults, which are optimized away by compiler.
#[derive(Debug, Clone)]
pub struct SvcSoupBinTcpProtocolManual<RecvP: SoupBinTcpPayload<RecvP>, SendP: SoupBinTcpPayload<SendP>> {
    phantom: PhantomData<(RecvP, SendP)>,
}
impl<RecvP: SoupBinTcpPayload<RecvP>, SendP: SoupBinTcpPayload<SendP>> Default for SvcSoupBinTcpProtocolManual<RecvP, SendP> {
    fn default() -> Self {
        Self { phantom: PhantomData }
    }
}
impl<RecvP: SoupBinTcpPayload<RecvP>, SendP: SoupBinTcpPayload<SendP>> Framer for SvcSoupBinTcpProtocolManual<RecvP, SendP> {
    #[inline(always)]
    fn get_frame_length(bytes: &bytes::BytesMut) -> Option<usize> {
        SoupBinTcpFramer::get_frame_length(bytes)
    }
}
impl<RecvP: SoupBinTcpPayload<RecvP>, SendP: SoupBinTcpPayload<SendP>> Messenger for SvcSoupBinTcpProtocolManual<RecvP, SendP> {
    type RecvT = <SvcSoupBinTcpMessenger<RecvP, SendP> as Messenger>::RecvT;
    type SendT = <SvcSoupBinTcpMessenger<RecvP, SendP> as Messenger>::SendT;

    #[inline(always)]
    fn serialize<const MAX_MSG_SIZE: usize>(msg: &Self::SendT) -> Result<([u8; MAX_MSG_SIZE], usize), std::io::Error> {
        SvcSoupBinTcpMessenger::<RecvP, SendP>::serialize(msg)
    }

    #[inline(always)]
    fn deserialize(frame: &[u8]) -> Result<Self::RecvT, Error> {
        SvcSoupBinTcpMessenger::<RecvP, SendP>::deserialize(frame)
    }
}
impl<RecvP: SoupBinTcpPayload<RecvP>, SendP: SoupBinTcpPayload<SendP>> ProtocolCore for SvcSoupBinTcpProtocolManual<RecvP, SendP> {
    fn is_connected(&self) -> bool {
        true
    }
}
impl<RecvP: SoupBinTcpPayload<RecvP>, SendP: SoupBinTcpPayload<SendP>> Protocol for SvcSoupBinTcpProtocolManual<RecvP, SendP> {}

#[cfg(test)]
#[cfg(feature = "unittest")]
mod test {

    use crate::prelude::*;
    use links_nonblocking::prelude::{unittest::setup, *};
    use log::info;
    use std::num::NonZeroUsize;
    type CltProtocolManual = CltSoupBinTcpProtocolManual<SamplePayload, SamplePayload>;
    type SvcProtocolManual = SvcSoupBinTcpProtocolManual<SamplePayload, SamplePayload>;

    #[test]
    fn test_protocol() {
        setup::log::configure_compact(log::LevelFilter::Info);
        const SOUP_BIN_MAX_FRAME_SIZE: usize = SOUPBINTCP_MAX_FRAME_SIZE_EXCLUDING_PAYLOAD_DEBUG;
        let clt_count = CounterCallback::new_ref();
        let svc_count = CounterCallback::new_ref();
        let clt_clbk = ChainCallback::new_ref(vec![LoggerCallback::with_level_ref(log::Level::Info, log::Level::Debug), clt_count.clone()]);
        let svc_clbk = ChainCallback::new_ref(vec![LoggerCallback::with_level_ref(log::Level::Info, log::Level::Debug), svc_count.clone()]);
        let io_timeout = setup::net::default_io_timeout();
        let addr = setup::net::rand_avail_addr_port();

        let protocol = SvcProtocolManual::default();
        let mut svc_sender = Svc::<_, _, SOUP_BIN_MAX_FRAME_SIZE>::bind(addr, NonZeroUsize::new(1).unwrap(), svc_clbk.clone(), protocol, Some("svc/soupbintcp/supervised"))
            .unwrap()
            .into_sender_with_spawned_recver();

        let protocol = CltProtocolManual::default();
        let mut clt_sender = Clt::<_, _, SOUP_BIN_MAX_FRAME_SIZE>::connect(
            addr,
            setup::net::default_connect_timeout(),
            setup::net::default_connect_retry_after(),
            clt_clbk.clone(),
            protocol,
            Some("clt/soupbintcp/supervised"),
        )
        .unwrap()
        .into_sender_with_spawned_recver();

        // client should not send any messages to perform login
        assert_eq!(clt_count.sent_count(), 0);
        assert_eq!(svc_count.sent_count(), 0);

        info!("clt.is_connected(): {}", clt_sender.is_connected());
        assert!(clt_sender.is_connected());
        info!(
            "svc.all_connected_busywait_timeout(): {}",
            svc_sender.all_connected_busywait_timeout(setup::net::default_find_timeout())
        );
        assert!(svc_sender.all_connected());

        const N: usize = 10;
        for i in 1..=N {
            clt_sender
                .send_busywait_timeout(&mut Debug::new(format!("Msg  #{}", i).as_bytes()).into(), io_timeout)
                .unwrap()
                .unwrap_completed();
        }

        assert_eq!(svc_count.recv_count_busywait_timeout(N, setup::net::default_find_timeout()), N);
        assert_eq!(svc_count.sent_count(), 0);
        info!("svc_count: {}", svc_count);

        for i in 1..=N {
            svc_sender
                .send_busywait_timeout(&mut Debug::new(format!("Msg  #{}", i).as_bytes()).into(), io_timeout)
                .unwrap()
                .unwrap_completed();
        }
        assert_eq!(clt_count.recv_count_busywait_timeout(N, setup::net::default_find_timeout()), N);
        assert_eq!(clt_count.sent_count(), N);
        info!("clt_count: {}", clt_count);
    }
}

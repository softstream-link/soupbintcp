use crate::prelude::*;
use lazy_static::lazy_static;
use links_nonblocking::prelude::*;
use soupbintcp_model::prelude::*;
use std::{fmt::Debug, io::Error, marker::PhantomData, time::Duration};

lazy_static! {
    static ref DEFAULT_MAX_RECV_INTERVAL: Duration = Duration::from_secs_f64(2.5);
}

/// Implements SoupBinTcp protocol for client side.
///
/// # [ProtocolCore] Features
/// * [`Self::on_recv`]
/// * [`Self::is_connected`]
///
/// # [Protocol] Features
/// * Not implemented - falls back to defaults, which are optimized away by compiler.
#[derive(Debug, Clone)]
pub struct CltSoupBinTcpProtocolIsConnected<RecvP: SoupBinTcpPayload<RecvP>, SendP: SoupBinTcpPayload<SendP>> {
    recv_con_state: ProtocolConnectionState<CltSoupBinTcpRecvConnectionState>,
    phantom: PhantomData<(RecvP, SendP)>,
}
impl<RecvP: SoupBinTcpPayload<RecvP>, SendP: SoupBinTcpPayload<SendP>> Default for CltSoupBinTcpProtocolIsConnected<RecvP, SendP> {
    fn default() -> Self {
        Self {
            recv_con_state: CltSoupBinTcpRecvConnectionState::new(*DEFAULT_MAX_RECV_INTERVAL).into(),
            phantom: PhantomData,
        }
    }
}
impl<RecvP: SoupBinTcpPayload<RecvP>, SendP: SoupBinTcpPayload<SendP>> Framer for CltSoupBinTcpProtocolIsConnected<RecvP, SendP> {
    #[inline(always)]
    fn get_frame_length(bytes: &bytes::BytesMut) -> Option<usize> {
        SoupBinTcpFramer::get_frame_length(bytes)
    }
}
impl<RecvP: SoupBinTcpPayload<RecvP>, SendP: SoupBinTcpPayload<SendP>> Messenger for CltSoupBinTcpProtocolIsConnected<RecvP, SendP> {
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
impl<RecvP: SoupBinTcpPayload<RecvP>, SendP: SoupBinTcpPayload<SendP>> ProtocolCore for CltSoupBinTcpProtocolIsConnected<RecvP, SendP> {
    // Will delegate to [`CltSoupBinTcpRecvConnectionState::on_recv`]
    #[allow(unused_variables)] // when compiled in release mode `who` is not used
    #[inline(always)]
    fn on_recv<I: ConnectionId>(&self, who: &I, msg: &<Self as Messenger>::RecvT) {
        #[cfg(debug_assertions)]
        log::debug!("{}::on_recv: con_id: {}, msg: {:?}", asserted_short_name!("CltSoupBinTcpProtocolIsConnected", Self), who.con_id(), msg);

        (*self.recv_con_state.lock()).on_recv(msg);
    }

    // Will delegate to [`CltSoupBinTcpRecvConnectionState::is_connected`]
    #[inline(always)]
    fn is_connected(&self) -> bool {
        (*self.recv_con_state.lock()).is_connected()
    }
}
impl<RecvP: SoupBinTcpPayload<RecvP>, SendP: SoupBinTcpPayload<SendP>> Protocol for CltSoupBinTcpProtocolIsConnected<RecvP, SendP> {}

/// Implements SoupBinTcp protocol for server side.
///
/// # [ProtocolCore] Features
/// * [`Self::on_recv`]
/// * [`Self::on_sent`]
/// * [`Self::is_connected`]
///
/// # [Protocol] Features
/// * Not implemented - falls back to defaults, which are optimized away by compiler.
#[derive(Debug, Clone)]
pub struct SvcSoupBinTcpProtocolIsConnected<RecvP: SoupBinTcpPayload<RecvP>, SendP: SoupBinTcpPayload<SendP>> {
    recv_con_state: ProtocolConnectionState<SvcSoupBinTcpRecvConnectionState>,
    send_con_state: ProtocolConnectionState<SvcSoupBinTcpSendConnectionState>,
    phantom: PhantomData<(RecvP, SendP)>,
}
impl<RecvP: SoupBinTcpPayload<RecvP>, SendP: SoupBinTcpPayload<SendP>> Default for SvcSoupBinTcpProtocolIsConnected<RecvP, SendP> {
    fn default() -> Self {
        Self {
            recv_con_state: SvcSoupBinTcpRecvConnectionState::new(*DEFAULT_MAX_RECV_INTERVAL).into(),
            send_con_state: SvcSoupBinTcpSendConnectionState::default().into(),
            phantom: PhantomData,
        }
    }
}
impl<RecvP: SoupBinTcpPayload<RecvP>, SendP: SoupBinTcpPayload<SendP>> Framer for SvcSoupBinTcpProtocolIsConnected<RecvP, SendP> {
    #[inline(always)]
    fn get_frame_length(bytes: &bytes::BytesMut) -> Option<usize> {
        SoupBinTcpFramer::get_frame_length(bytes)
    }
}
impl<RecvP: SoupBinTcpPayload<RecvP>, SendP: SoupBinTcpPayload<SendP>> Messenger for SvcSoupBinTcpProtocolIsConnected<RecvP, SendP> {
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
impl<RecvP: SoupBinTcpPayload<RecvP>, SendP: SoupBinTcpPayload<SendP>> ProtocolCore for SvcSoupBinTcpProtocolIsConnected<RecvP, SendP> {
    /// Will delegate to [`SvcSoupBinTcpRecvConnectionState::on_recv`]
    #[allow(unused_variables)] // when compiled in release mode `who` is not used
    #[inline(always)]
    fn on_recv<I: ConnectionId>(&self, who: &I, msg: &<Self as Messenger>::RecvT) {
        #[cfg(debug_assertions)]
        log::debug!("{}::on_recv: con_id: {}, msg: {:?}", asserted_short_name!("SvcSoupBinTcpProtocolIsConnected", Self), who.con_id(), msg);

        (*self.recv_con_state.lock()).on_recv(msg);
    }

    /// Will delegate to [`SvcSoupBinTcpSendConnectionState::on_sent`]
    #[allow(unused_variables)] // when compiled in release mode `who` is not used
    #[inline(always)]
    fn on_sent<I: ConnectionId>(&self, who: &I, msg: &<Self as Messenger>::SendT) {
        #[cfg(debug_assertions)]
        log::debug!("{}::on_sent: con_id: {}, msg: {:?}", asserted_short_name!("SvcSoupBinTcpProtocolIsConnected", Self), who.con_id(), msg);

        (*self.send_con_state.lock()).on_sent(msg);
    }
    /// Will returns `true` if all of below are `true`
    /// * [`crate::prelude::SvcSoupBinTcpRecvConnectionState::is_connected`]
    /// * [`crate::prelude::SvcSoupBinTcpSendConnectionState::is_connected`]
    #[inline(always)]
    fn is_connected(&self) -> bool {
        (*self.recv_con_state.lock()).is_connected() && (*self.send_con_state.lock()).is_connected()
    }
}
impl<RecvP: SoupBinTcpPayload<RecvP>, SendP: SoupBinTcpPayload<SendP>> Protocol for SvcSoupBinTcpProtocolIsConnected<RecvP, SendP> {}

#[cfg(test)]
#[cfg(feature = "unittest")]
mod test {

    use crate::prelude::*;
    use links_nonblocking::prelude::{unittest::setup, *};
    use log::info;
    use soupbintcp_model::prelude::*;
    use std::num::NonZeroUsize;
    type CltProtocolIsConnected = CltSoupBinTcpProtocolIsConnected<SamplePayload, SamplePayload>;
    type SvcProtocolIsConnected = SvcSoupBinTcpProtocolIsConnected<SamplePayload, SamplePayload>;

    #[test]
    fn test_protocol() {
        setup::log::configure_compact(log::LevelFilter::Info);
        const SOUP_BIN_MAX_FRAME_SIZE: usize = SOUPBINTCP_MAX_FRAME_SIZE_EXCLUDING_PAYLOAD_DEBUG;
        let clt_count = CounterCallback::new_ref();
        let svc_count = CounterCallback::new_ref();
        let clt_clbk = ChainCallback::new_ref(vec![clt_count.clone(), LoggerCallback::with_level_ref(log::Level::Info, log::Level::Debug)]);
        let svc_clbk = ChainCallback::new_ref(vec![svc_count.clone(), LoggerCallback::with_level_ref(log::Level::Info, log::Level::Debug)]);
        let addr = setup::net::rand_avail_addr_port();

        let mut svc_sender = Svc::<_, _, SOUP_BIN_MAX_FRAME_SIZE>::bind(addr, NonZeroUsize::new(1).unwrap(), svc_clbk, SvcProtocolIsConnected::default(), Some("svc/soupbintcp/supervised"))
            .unwrap()
            .into_sender_with_spawned_recver();

        let mut clt_sender = Clt::<_, _, SOUP_BIN_MAX_FRAME_SIZE>::connect(
            addr,
            setup::net::default_connect_timeout(),
            setup::net::default_connect_retry_after(),
            clt_clbk,
            CltProtocolIsConnected::default(),
            Some("clt/soupbintcp/supervised"),
        )
        .unwrap()
        .into_sender_with_spawned_recver();

        // client should not send any messages to perform login
        assert_eq!(clt_count.sent_count(), 0);

        info!("svc.all_connected(): {}", svc_sender.all_connected());
        assert!(!svc_sender.all_connected());
        info!("clt.is_connected(): {}", clt_sender.is_connected());
        assert!(!clt_sender.is_connected());

        let timeout = setup::net::default_connect_timeout();

        clt_sender.send_busywait_timeout(&mut LoginRequest::default().into(), timeout).unwrap().unwrap_completed();
        svc_sender.send_busywait_timeout(&mut LoginAccepted::default().into(), timeout).unwrap().unwrap_completed();

        let clt_is_connected_busywait_timeout = clt_sender.is_connected_busywait_timeout(timeout);
        info!("clt.is_connected_busywait_timeout(): {}", clt_is_connected_busywait_timeout);
        assert!(clt_is_connected_busywait_timeout);
        let is_next_connected = svc_sender.is_next_connected();
        info!("svc.is_next_connected(): {}", is_next_connected);
        assert!(is_next_connected);

        assert_eq!(clt_count.sent_count(), 1);
        assert_eq!(svc_count.sent_count(), 1);
    }
}

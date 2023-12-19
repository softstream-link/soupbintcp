use crate::prelude::*;
use std::{
    fmt::Debug,
    io::{Error, ErrorKind},
    marker::PhantomData,
    time::Duration,
};

/// Implements SoupBinTcp protocol for client side.
///
/// # [ProtocolCore] Features
/// * [`Self::on_connect`]
/// * [`Self::on_recv`]
/// * [`Self::is_connected`]
///
/// # [Protocol] Features
/// * [`Self::conf_heart_beat_interval`]
/// * [`Self::send_heart_beat`]
#[derive(Debug, Clone)]
pub struct CltSoupBinTcpProtocolAuto<RecvP: SoupBinTcpPayload<RecvP>, SendP: SoupBinTcpPayload<SendP>> {
    username: UserName,
    password: Password,
    session_id: SessionId,
    sequence_number: SequenceNumber,
    io_timeout: Duration,
    max_hbeat_send_interval: Duration,
    recv_con_state: ProtocolState<CltSoupBinTcpRecvConnectionState>,
    phantom: PhantomData<(RecvP, SendP)>,
}
impl<RecvP: SoupBinTcpPayload<RecvP>, SendP: SoupBinTcpPayload<SendP>> CltSoupBinTcpProtocolAuto<RecvP, SendP> {
    /// Creates new instance
    ///
    /// # Arguments
    /// * `username` - username to be used during authentication
    /// * `password` - password to be used during authentication
    /// * `session_id` - session_id to be used during authentication
    /// * `sequence_num` - sequence_num that client wants to start receiving messages from
    /// * `io_timeout` - timeout for login sequence during [`Self::on_connect`] hook
    /// * `max_hbeat_interval_send` - maximum interval between sending heartbeats, will result in [Self::conf_heart_beat_interval] be 2.5 times faster, so if max is set to 250 seconds then heartbeats will be sent every 100 seconds
    /// * `max_hbeat_interval_recv` - maximum interval between receiving heartbeats, if exceeded [`Self::is_connected`] returns `false`
    pub fn new(username: UserName, password: Password, session_id: SessionId, sequence_number: SequenceNumber, io_timeout: Duration, max_hbeat_interval_send: Duration, max_hbeat_interval_recv: Duration) -> Self {
        Self {
            username,
            password,
            session_id,
            sequence_number,
            io_timeout,
            max_hbeat_send_interval: max_hbeat_interval_send,
            recv_con_state: CltSoupBinTcpRecvConnectionState::new(max_hbeat_interval_recv).into(),
            phantom: PhantomData,
        }
    }
}
impl<RecvP: SoupBinTcpPayload<RecvP>, SendP: SoupBinTcpPayload<SendP>> Framer for CltSoupBinTcpProtocolAuto<RecvP, SendP> {
    #[inline(always)]
    fn get_frame_length(bytes: &mut bytes::BytesMut) -> Option<usize> {
        SoupBinTcpFramer::get_frame_length(bytes)
    }
}
impl<RecvP: SoupBinTcpPayload<RecvP>, SendP: SoupBinTcpPayload<SendP>> Messenger for CltSoupBinTcpProtocolAuto<RecvP, SendP> {
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
impl<RecvP: SoupBinTcpPayload<RecvP>, SendP: SoupBinTcpPayload<SendP>> ProtocolCore for CltSoupBinTcpProtocolAuto<RecvP, SendP> {
    /// handles [LoginRequest]/[LoginAccepted][LoginRejected] authentication sequence
    #[inline(always)]
    fn on_connect<C: SendNonBlocking<Self::SendT> + RecvNonBlocking<Self::RecvT> + ConnectionId>(&self, con: &mut C) -> Result<(), Error> {
        let mut msg = LoginRequest::new(self.username, self.password, self.session_id, self.sequence_number, self.max_hbeat_send_interval.into()).into();
        match con.send_busywait_timeout(&mut msg, self.io_timeout)? {
            SendStatus::Completed => match con.recv_busywait_timeout(self.io_timeout)? {
                RecvStatus::Completed(Some(SvcSoupBinTcpMsg::LoginAccepted(msg))) => Ok(()), // TODO don't remove warning until dealt with LoginAccepted.SequenceNumber need to add store for sent messages to be able to recover
                RecvStatus::Completed(msg) => Err(Error::new(ErrorKind::Other, format!("Failed to login: {:?}", msg))),
                RecvStatus::WouldBlock => Err(Error::new(ErrorKind::TimedOut, format!("Failed to receive login: {:?}", msg))),
            },
            SendStatus::WouldBlock => Err(Error::new(ErrorKind::TimedOut, format!("Failed to send login: {:?}", msg))),
        }
    }

    /// Will delegate to [`CltSoupBinTcpRecvConnectionState::on_recv`]
    #[allow(unused_variables)] // when compiled in release mode `who` is not used
    #[inline(always)]
    fn on_recv<I: ConnectionId>(&self, who: &I, msg: &<Self as Messenger>::RecvT) {
        #[cfg(debug_assertions)]
        log::debug!("{}::on_recv: con_id: {}, msg: {:?}", asserted_short_name!("CltSoupBinTcpProtocolAuto", Self), who.con_id(), msg);

        (*self.recv_con_state.lock()).on_recv(msg);
    }

    /// Will delegate to [`CltSoupBinTcpRecvConnectionState::is_connected`]
    #[inline(always)]
    fn is_connected(&self) -> bool {
        (*self.recv_con_state.lock()).is_connected()
    }
}
impl<RecvP: SoupBinTcpPayload<RecvP>, SendP: SoupBinTcpPayload<SendP>> Protocol for CltSoupBinTcpProtocolAuto<RecvP, SendP> {
    /// Configures interval from arguments of [`Self::new`]
    #[inline(always)]
    fn conf_heart_beat_interval(&self) -> Option<Duration> {
        Some(self.max_hbeat_send_interval.div_f64(2.5))
    }

    /// sends [CltHeartbeat] instance to the server
    #[inline(always)]
    fn send_heart_beat<S: SendNonBlocking<Self::SendT> + ConnectionId>(&self, sender: &mut S) -> Result<SendStatus, Error> {
        sender.send(&mut CltSoupBinTcpMsg::hbeat())
    }
}

/// Implements SoupBinTcp protocol for server side.
///
/// # [ProtocolCore] Features
/// * [`Self::on_connect`]
/// * [`Self::on_recv`]
/// * [`Self::on_sent`]
/// * [`Self::is_connected`]
/// * [`Self::on_disconnect`]
///
/// # [Protocol] Features
/// * [`Self::conf_heart_beat_interval`]
/// * [`Self::send_heart_beat`]
#[derive(Debug, Clone)]
pub struct SvcSoupBinTcpProtocolAuto<RecvP: SoupBinTcpPayload<RecvP>, SendP: SoupBinTcpPayload<SendP>> {
    username: UserName,
    password: Password,
    session_id: SessionId,
    io_timeout: Duration,
    max_hbeat_interval_send: Duration,
    recv_con_state: ProtocolState<SvcSoupBinTcpRecvConnectionState>,
    send_con_state: ProtocolState<SvcSoupBinTcpSendConnectionState>,
    phantom: PhantomData<(RecvP, SendP)>,
}
impl<RecvP: SoupBinTcpPayload<RecvP>, SendP: SoupBinTcpPayload<SendP>> SvcSoupBinTcpProtocolAuto<RecvP, SendP> {
    /// Creates new instance
    ///
    /// # Arguments
    /// * `username` - username to be used during authentication
    /// * `password` - password to be used during authentication
    /// * `session_id` - session_id to be used during authentication
    /// * `io_timeout` - timeout for login sequence during [`Self::on_connect`] hook
    /// * `max_hbeat_interval_send` - maximum interval between sending heartbeats, will result in [Self::conf_heart_beat_interval] be 2.5 times faster, so if max is set to 250 seconds then heartbeats will be sent every 100 seconds
    pub fn new(username: UserName, password: Password, session_id: SessionId, io_timeout: Duration, max_hbeat_interval_send: Duration) -> Self {
        Self {
            username,
            password,
            session_id,
            io_timeout,
            max_hbeat_interval_send,
            recv_con_state: SvcSoupBinTcpRecvConnectionState::default().into(),
            send_con_state: SvcSoupBinTcpSendConnectionState::default().into(),
            phantom: PhantomData,
        }
    }
}
impl<RecvP: SoupBinTcpPayload<RecvP>, SendP: SoupBinTcpPayload<SendP>> Framer for SvcSoupBinTcpProtocolAuto<RecvP, SendP> {
    #[inline(always)]
    fn get_frame_length(bytes: &mut bytes::BytesMut) -> Option<usize> {
        SoupBinTcpFramer::get_frame_length(bytes)
    }
}
impl<RecvP: SoupBinTcpPayload<RecvP>, SendP: SoupBinTcpPayload<SendP>> Messenger for SvcSoupBinTcpProtocolAuto<RecvP, SendP> {
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
impl<RecvP: SoupBinTcpPayload<RecvP>, SendP: SoupBinTcpPayload<SendP>> ProtocolCore for SvcSoupBinTcpProtocolAuto<RecvP, SendP> {
    /// handles [LoginRequest]/[LoginAccepted][LoginRejected] authentication sequence
    #[inline(always)]
    fn on_connect<C: SendNonBlocking<Self::SendT> + RecvNonBlocking<Self::RecvT> + ConnectionId>(&self, con: &mut C) -> Result<(), Error> {
        match con.recv_busywait_timeout(self.io_timeout)? {
            RecvStatus::Completed(Some(CltSoupBinTcpMsg::Login(msg))) => {
                if msg.username == self.username && msg.password == self.password && (msg.session_id == self.session_id || msg.session_id == SessionId::default()) {
                    // self.login_request.set(msg);
                    let mut msg = LoginAccepted::default().into();
                    match con.send_busywait_timeout(&mut msg, self.io_timeout)? {
                        SendStatus::Completed => Ok(()),
                        SendStatus::WouldBlock => Err(Error::new(ErrorKind::TimedOut, format!("Failed to send login: {:?}", msg))),
                    }
                } else if msg.session_id != self.session_id {
                    con.send_busywait_timeout(&mut LoginRejected::session_not_available().into(), self.io_timeout)?;
                    Err(Error::new(ErrorKind::NotConnected, format!("Invalid session_id msg: {:?}", msg)))
                } else {
                    con.send_busywait_timeout(&mut LoginRejected::not_authorized().into(), self.io_timeout)?;
                    Err(Error::new(ErrorKind::NotConnected, format!("Not Authorized msg: {:?}", msg)))
                }
            }
            RecvStatus::Completed(msg) => Err(Error::new(ErrorKind::Other, format!("Expected LoginRequest instead got msg:{:?}", msg))),
            RecvStatus::WouldBlock => Err(Error::new(ErrorKind::TimedOut, format!("Did not get LoginRequest during timeout: {:?}", self.io_timeout))),
        }
    }
    /// Will delegate to [`SvcSoupBinTcpRecvConnectionState::on_recv`]
    #[allow(unused_variables)] // when compiled in release mode `who` is not used
    #[inline(always)]
    fn on_recv<I: ConnectionId>(&self, who: &I, msg: &<Self as Messenger>::RecvT) {
        #[cfg(debug_assertions)]
        log::debug!("{}::on_recv: con_id: {}, msg: {:?}", asserted_short_name!("SvcSoupBinTcpProtocolAuto", Self), who.con_id(), msg);

        (*self.recv_con_state.lock()).on_recv(msg);
    }
    /// updates internal timestamp of when [LoginAccepted] and [EndOfSession] where sent detect connection loss via [`Self::is_connected`]
    #[allow(unused_variables)] // when compiled in release mode `who` is not used
    #[inline(always)]
    fn on_sent<I: ConnectionId>(&self, who: &I, msg: &<Self as Messenger>::SendT) {
        #[cfg(debug_assertions)]
        log::debug!("{}::on_sent: con_id: {}, msg: {:?}", asserted_short_name!("SvcSoupBinTcpProtocolAuto", Self), who.con_id(), msg);

        (*self.send_con_state.lock()).on_sent(msg);
    }

    /// Will returns `true` if all of below are `true`
    /// * [`crate::prelude::SvcSoupBinTcpRecvConnectionState::is_connected`]
    /// * [`crate::prelude::SvcSoupBinTcpSendConnectionState::is_connected`]
    #[inline(always)]
    fn is_connected(&self) -> bool {
        (*self.recv_con_state.lock()).is_connected() && (*self.send_con_state.lock()).is_connected()
    }
    /// Returns a tuple of
    /// * [Duration] - the timeout during which the [CltSender] will wait while delivering final message before disconnecting
    /// * [EndOfSession] - the message to be sent to the client
    #[inline(always)]
    fn on_disconnect(&self) -> Option<(Duration, <Self as Messenger>::SendT)> {
        Some((self.io_timeout, EndOfSession::default().into()))
    }
}
impl<RecvP: SoupBinTcpPayload<RecvP>, SendP: SoupBinTcpPayload<SendP>> Protocol for SvcSoupBinTcpProtocolAuto<RecvP, SendP> {
    #[inline(always)]
    fn conf_heart_beat_interval(&self) -> Option<Duration> {
        Some(self.max_hbeat_interval_send.div_f64(2.5))
    }
    /// sends [SvcHeartbeat] instance to the client
    #[inline(always)]
    fn send_heart_beat<S: SendNonBlocking<Self::SendT> + ConnectionId>(&self, sender: &mut S) -> Result<SendStatus, Error> {
        sender.send(&mut SvcSoupBinTcpMsg::hbeat())
    }
}

#[cfg(test)]
#[cfg(feature = "unittest")]
mod test {

    use crate::prelude::*;
    use std::{num::NonZeroUsize, thread::sleep, time::Duration};

    use links_core::unittest::setup;
    use log::info;
    type CltProtocolAuto = CltSoupBinTcpProtocolAuto<SamplePayload, SamplePayload>;
    type SvcProtocolAuto = SvcSoupBinTcpProtocolAuto<SamplePayload, SamplePayload>;

    #[test]
    fn test_protocol() {
        // setup::log::configure_compact(log::LevelFilter::Info);
        setup::log::configure_level(log::LevelFilter::Info);

        const SOUP_BIN_MAX_FRAME_SIZE: usize = SOUPBINTCP_MAX_FRAME_SIZE_EXCLUDING_PAYLOAD_DEBUG;
        let addr = setup::net::rand_avail_addr_port();
        let max_hbeat_interval = Duration::from_secs_f64(2.5);
        let io_timeout = setup::net::default_connect_timeout();

        let clt_count = CounterCallback::new_ref();
        let svc_count = CounterCallback::new_ref();
        let clt_clbk = ChainCallback::new_ref(vec![clt_count.clone(), LoggerCallback::with_level_ref(log::Level::Info, log::Level::Debug)]);
        let svc_clbk = ChainCallback::new_ref(vec![svc_count.clone(), LoggerCallback::with_level_ref(log::Level::Info, log::Level::Debug)]);

        let login = LoginRequest::default();
        let mut svc = Svc::<_, _, SOUP_BIN_MAX_FRAME_SIZE>::bind(
            addr,
            NonZeroUsize::new(1).unwrap(),
            svc_clbk.clone(),
            SvcProtocolAuto::new(login.username, login.password, login.session_id, io_timeout, max_hbeat_interval),
            Some("soupbintcp/auth/unittest"),
        )
        .unwrap()
        .into_sender_with_spawned_recver_ref();
        let clt = Clt::<_, _, SOUP_BIN_MAX_FRAME_SIZE>::connect(
            addr,
            setup::net::default_connect_timeout(),
            setup::net::default_connect_retry_after(),
            clt_clbk.clone(),
            CltProtocolAuto::new(login.username, login.password, login.session_id, login.sequence_number, io_timeout, max_hbeat_interval, max_hbeat_interval),
            Some("soupbintcp/auth/unittest"),
        )
        .unwrap()
        .into_sender_with_spawned_recver_ref();

        // Connection established
        info!("clt.is_connected(): {:?}", clt.is_connected());
        assert!(clt.is_connected());
        info!("svc.all_connected(): {:?}", svc.all_connected());
        assert!(svc.all_connected());

        sleep(Duration::from_millis(100)); // wait for just the first hbeat to be sent

        // this indicates client sent login request & one hbeat
        assert_eq!(clt_count.sent_count(), 2);
        // this indicates server sent login accepted & one hbeat
        assert_eq!(svc_count.sent_count(), 2);

        // Connection still valid after hbeats
        info!("clt.is_connected(): {:?}", clt.is_connected());
        assert!(clt.is_connected());
        info!("svc.all_connected(): {:?}", svc.all_connected());
        assert!(svc.all_connected());

        drop(svc);
        assert_eq!(svc_count.sent_count(), 3);
        sleep(Duration::from_millis(100)); // wait for just the End of Session
        assert_eq!(clt_count.recv_count(), 3);

        // Connection no longer valid on clt
        info!("clt.is_connected(): {:?}", clt.is_connected());
        assert!(!clt.is_connected());
    }
}

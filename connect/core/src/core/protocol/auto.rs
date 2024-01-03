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
    recv_con_state: ProtocolConnectionState<CltSoupBinTcpRecvConnectionState>,
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
    /// * `clt_max_hbeat_interval` - maximum interval between sending heartbeats, will result in [`Self::conf_heart_beat_interval`] be 2.5 times faster,
    /// so if max is set to 25 seconds then heartbeats will be sent every 10 seconds
    /// * `svc_max_hbeat_interval` - maximum interval between receiving heartbeats, if exceeded [`Self::is_connected`] returns `false`
    pub fn new(
        username: UserName,
        password: Password,
        session_id: SessionId,
        sequence_number: SequenceNumber,
        io_timeout: Duration,
        clt_max_hbeat_interval: Duration,
        svc_max_hbeat_interval: Duration,
    ) -> Self {
        Self {
            username,
            password,
            session_id,
            sequence_number,
            io_timeout,
            max_hbeat_send_interval: clt_max_hbeat_interval,
            recv_con_state: CltSoupBinTcpRecvConnectionState::new(svc_max_hbeat_interval).into(),
            phantom: PhantomData,
        }
    }
}
impl<RecvP: SoupBinTcpPayload<RecvP>, SendP: SoupBinTcpPayload<SendP>> Framer for CltSoupBinTcpProtocolAuto<RecvP, SendP> {
    #[inline(always)]
    fn get_frame_length(bytes: &bytes::BytesMut) -> Option<usize> {
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
    fn on_connect<C: SendNonBlocking<<Self as Messenger>::SendT> + ReSendNonBlocking<<Self as Messenger>::SendT> + RecvNonBlocking<<Self as Messenger>::RecvT> + ConnectionId>(
        &self,
        con: &mut C,
    ) -> Result<(), Error> {
        let mut msg = LoginRequest::new(self.username, self.password, self.session_id, self.sequence_number, self.max_hbeat_send_interval.into()).into();
        match con.send_busywait_timeout(&mut msg, self.io_timeout)? {
            SendStatus::Completed => match con.recv_busywait_timeout(self.io_timeout)? {
                RecvStatus::Completed(Some(SvcSoupBinTcpMsg::LoginAccepted(_msg))) => Ok(()),
                RecvStatus::Completed(msg) => Err(Error::new(ErrorKind::Other, format!("Failed to login: {:?}", msg))),
                RecvStatus::WouldBlock => Err(Error::new(ErrorKind::TimedOut, format!("Did not get LoginAccepted with io-timeout: {:?}", self.io_timeout))),
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
    svc_max_hbeat_interval: Duration,
    recv_con_state: ProtocolConnectionState<SvcSoupBinTcpRecvConnectionState>,
    send_con_state: ProtocolConnectionState<SvcSoupBinTcpSendConnectionState>,
    send_ses_state: ProtocolSessionState<SvcSoupBinTcpSendSessionState<SendP, InMemoryMessageLog<SvcSoupBinTcpMsg<SendP>>>>, // TODO make generic to allow for file based message log
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
    /// * `svc_max_hbeat_interval` - maximum interval between sending heartbeats, will result in [`Self::conf_heart_beat_interval`] be 2.5 times faster, so if max is set to 25 seconds then heartbeats will be sent every 10 seconds
    pub fn new(username: UserName, password: Password, session_id: SessionId, io_timeout: Duration, svc_max_hbeat_interval: Duration) -> Self {
        let session_storage = InMemoryMessageLog::<SvcSoupBinTcpMsg<SendP>>::default();
        let session_state = SvcSoupBinTcpSendSessionState::new(session_storage);
        Self {
            username,
            password,
            session_id,
            io_timeout,
            svc_max_hbeat_interval,
            recv_con_state: SvcSoupBinTcpRecvConnectionState::default().into(),
            send_con_state: SvcSoupBinTcpSendConnectionState::default().into(),
            send_ses_state: ProtocolSessionState::new(session_state),
            phantom: PhantomData,
        }
    }
}
impl<RecvP: SoupBinTcpPayload<RecvP>, SendP: SoupBinTcpPayload<SendP>> Framer for SvcSoupBinTcpProtocolAuto<RecvP, SendP> {
    #[inline(always)]
    fn get_frame_length(bytes: &bytes::BytesMut) -> Option<usize> {
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
    fn on_connect<C: SendNonBlocking<<Self as Messenger>::SendT> + ReSendNonBlocking<<Self as Messenger>::SendT> + RecvNonBlocking<<Self as Messenger>::RecvT> + ConnectionId>(
        &self,
        con: &mut C,
    ) -> Result<(), Error> {
        match con.recv_busywait_timeout(self.io_timeout)? {
            RecvStatus::Completed(Some(CltSoupBinTcpMsg::LoginRequest(msg))) => {
                if msg.username == self.username && msg.password == self.password && (msg.session_id == self.session_id || msg.session_id == SessionId::default()) {
                    let clt_next_sequenced_payload_number: usize = msg.sequence_number.into();

                    let svc_next_sequenced_payload_number = (*self.send_ses_state.lock()).next_sequenced_payload_number();
                    let effective_next_sequence_number = {
                        if clt_next_sequenced_payload_number == 0 {
                            svc_next_sequenced_payload_number
                        } else {
                            clt_next_sequenced_payload_number
                        }
                    };

                    let mut msg = LoginAccepted::new(self.session_id, effective_next_sequence_number.into()).into();
                    match con.send_busywait_timeout(&mut msg, self.io_timeout)? {
                        SendStatus::Completed => {
                            if effective_next_sequence_number < svc_next_sequenced_payload_number {
                                for re_msg in (*self.send_ses_state.lock())
                                    .get_storage()
                                    .iter()
                                    .filter(|msg| matches!(msg, SvcSoupBinTcpMsg::SPayload(_)))
                                    .skip(effective_next_sequence_number - 1)
                                {
                                    if let SendStatus::WouldBlock = con.re_send_busywait_timeout(re_msg, self.io_timeout)? {
                                        return Err(Error::new(ErrorKind::TimedOut, format!("Failed to resend msg: {:?}", re_msg)));
                                    }
                                }
                            }
                            Ok(())
                        }
                        SendStatus::WouldBlock => Err(Error::new(ErrorKind::TimedOut, format!("Failed to send login: {:?}", msg))),
                    }
                } else if msg.session_id != self.session_id {
                    con.send_busywait_timeout(&mut LoginRejected::session_not_available().into(), self.io_timeout)?;
                    Err(Error::new(
                        ErrorKind::NotConnected,
                        format!("Invalid session_id expected: {:?} received: {:?}", self.session_id, msg.session_id),
                    ))
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
        (*self.send_ses_state.lock()).on_sent(msg);
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
        Some(self.svc_max_hbeat_interval.div_f64(2.5))
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
    use std::{num::NonZeroUsize, time::Duration};

    use links_core::unittest::setup;
    use log::info;
    type CltProtocolAuto = CltSoupBinTcpProtocolAuto<SamplePayload, SamplePayload>;
    type SvcProtocolAuto = SvcSoupBinTcpProtocolAuto<SamplePayload, SamplePayload>;
    type UnitMsg = UniSoupBinTcpMsg<SamplePayload, SamplePayload>;

    #[test]
    fn test_protocol() {
        // setup::log::configure_compact(log::LevelFilter::Info);
        setup::log::configure_level(log::LevelFilter::Info);

        const SOUP_BIN_MAX_FRAME_SIZE: usize = SOUPBINTCP_MAX_FRAME_SIZE_EXCLUDING_PAYLOAD_DEBUG;
        let addr = setup::net::rand_avail_addr_port();
        let username: UserName = b"userid".as_slice().into();
        let password: Password = b"passwd".as_slice().into();
        let session_id: SessionId = b"favsession".as_slice().into();
        let io_timeout = setup::net::find_timeout(); // use find because this is used while waiting for reply's from server when using auto protocol

        let max_hbeat_interval_send = Duration::from_secs_f64(2.5);
        let max_hbeat_interval_recv = max_hbeat_interval_send;

        let clt_store = CanonicalEntryStore::<UnitMsg>::new_ref();
        let svc_store = CanonicalEntryStore::<UnitMsg>::new_ref();
        let clt_count = CounterCallback::new_ref();
        let svc_count = CounterCallback::new_ref();
        let clt_clbk = ChainCallback::new_ref(vec![
            StoreCallback::new_ref(clt_store.clone()),
            clt_count.clone(),
            LoggerCallback::with_level_ref(log::Level::Info, log::Level::Debug),
        ]);
        let svc_clbk = ChainCallback::new_ref(vec![
            StoreCallback::new_ref(svc_store.clone()),
            svc_count.clone(),
            LoggerCallback::with_level_ref(log::Level::Info, log::Level::Debug),
        ]);

        let protocol = SvcProtocolAuto::new(username, password, session_id, io_timeout, max_hbeat_interval_send);
        let mut svc = Svc::<_, _, SOUP_BIN_MAX_FRAME_SIZE>::bind(addr, NonZeroUsize::new(1).unwrap(), svc_clbk.clone(), protocol, Some("svc/soupbintcp/auto"))
            .unwrap()
            .into_sender_with_spawned_recver_ref();

        let sequence_number: SequenceNumber = 0_u64.into();
        let protocol = CltProtocolAuto::new(username, password, session_id, sequence_number, io_timeout, max_hbeat_interval_send, max_hbeat_interval_recv);
        let clt = Clt::<_, _, SOUP_BIN_MAX_FRAME_SIZE>::connect(
            addr,
            setup::net::default_connect_timeout(),
            setup::net::default_connect_retry_after(),
            clt_clbk.clone(),
            protocol,
            Some("clt/soupbintcp/auto"),
        )
        .unwrap()
        .into_sender_with_spawned_recver_ref();

        // Connection established LoginAccepted received and within hbeat_interval_recv, hence connection is valid
        info!("clt.is_connected(): {:?}", clt.is_connected());
        assert!(clt.is_connected());
        info!("svc.all_connected(): {:?}", svc.all_connected());
        assert!(svc.all_connected());

        let found = clt_store
            .find_recv(
                "clt/soupbintcp/auto",
                |msg| matches!(msg, UniSoupBinTcpMsg::Svc(SvcSoupBinTcpMsg::HBeat(_))),
                setup::net::optional_find_timeout().into(),
            )
            .unwrap();
        info!("found: {:?}", found);
        let found = svc_store
            .find_recv(
                "svc/soupbintcp/auto",
                |msg| matches!(msg, UniSoupBinTcpMsg::Clt(CltSoupBinTcpMsg::HBeat(_))),
                setup::net::optional_find_timeout().into(),
            )
            .unwrap();
        info!("found: {:?}", found);

        const HAND_SHAKE_COUNT: usize = 2; // clt login request & one hbeat | svc login accepted & one hbeat

        info!("clt_count: {}", clt_count);
        assert_eq!(clt_count.sent_count(), HAND_SHAKE_COUNT); // this indicates client sent login request & one hbeat
        info!("svc_count: {}", svc_count);
        assert_eq!(svc_count.sent_count(), HAND_SHAKE_COUNT); // this indicates server sent login accepted & one hbeat

        info!("clt_count: {}", clt_count);
        assert_eq!(clt_count.recv_count_busywait_timeout(HAND_SHAKE_COUNT, setup::net::find_timeout()), HAND_SHAKE_COUNT); // this indicates client recv login request & one hbeat
        info!("svc_count: {}", svc_count);
        assert_eq!(svc_count.recv_count_busywait_timeout(HAND_SHAKE_COUNT, setup::net::find_timeout()), HAND_SHAKE_COUNT); // this indicates server recv login accepted & one hbeat

        // Connection still valid after hbeats
        info!("clt.is_connected(): {:?}", clt.is_connected());
        assert!(clt.is_connected());
        info!("svc.all_connected(): {:?}", svc.all_connected());
        assert!(svc.all_connected());

        const N_SEQUENCED_PAYLOADS: usize = 10;
        const N_UN_SEQUENCED_PAYLOADS: usize = 10;
        let mut un_sequenced_msg = SvcSoupBinTcpMsg::udata(SamplePayload::default());
        for _ in 1..=N_UN_SEQUENCED_PAYLOADS {
            svc.send_busywait_timeout(&mut un_sequenced_msg, io_timeout).unwrap().unwrap_completed();
        }
        for i in 1..=N_SEQUENCED_PAYLOADS {
            let payload = SamplePayload::new(format!("#{} SPayload", i).as_bytes().into());
            let mut sequenced_msg = SvcSoupBinTcpMsg::sdata(payload);
            svc.send_busywait_timeout(&mut sequenced_msg, io_timeout).unwrap().unwrap_completed();
        }

        assert_eq!(svc_count.sent_count(), HAND_SHAKE_COUNT + N_SEQUENCED_PAYLOADS + N_UN_SEQUENCED_PAYLOADS); // this indicates server sent login accepted & one hbeat + N_SEQUENCED_PAYLOADS which are now in the internal session cache
        assert_eq!(
            clt_count.recv_count_busywait_timeout(HAND_SHAKE_COUNT + N_SEQUENCED_PAYLOADS + N_UN_SEQUENCED_PAYLOADS, setup::net::optional_find_timeout().unwrap()),
            HAND_SHAKE_COUNT + N_SEQUENCED_PAYLOADS + N_UN_SEQUENCED_PAYLOADS
        );
        info!("clt_count: {}", clt_count);
        info!("svc_count: {}", svc_count);

        drop(clt);

        // reconnect
        let clt_store_reconnect = CanonicalEntryStore::<UnitMsg>::new_ref();
        let clt_count_reconnect = CounterCallback::new_ref();
        let clt_clbk_reconnect = ChainCallback::new_ref(vec![
            StoreCallback::new_ref(clt_store_reconnect.clone()),
            clt_count_reconnect.clone(),
            LoggerCallback::with_level_ref(log::Level::Info, log::Level::Debug),
        ]);
        let reconnect_sequence_number = 6_usize;
        let protocol_reconnect = CltProtocolAuto::new(
            username,
            password,
            session_id,
            reconnect_sequence_number.into(),
            io_timeout,
            max_hbeat_interval_send,
            max_hbeat_interval_recv,
        );
        let clt_reconnect = Clt::<_, _, SOUP_BIN_MAX_FRAME_SIZE>::connect(
            addr,
            setup::net::default_connect_timeout(),
            setup::net::default_connect_retry_after(),
            clt_clbk_reconnect.clone(),
            protocol_reconnect,
            Some("clt_reconnect/soupbintcp/auto"),
        )
        .unwrap()
        .into_sender_with_spawned_recver_ref();

        assert!(clt_reconnect.is_connected());
        info!("svc_count: {}", svc_count);
        let clt_reconnect_expected_recv = HAND_SHAKE_COUNT + N_SEQUENCED_PAYLOADS - reconnect_sequence_number + 1;
        info!("clt_count_reconnect: {}", clt_count_reconnect);
        assert_eq!(
            clt_count_reconnect.recv_count_busywait_timeout(clt_reconnect_expected_recv, setup::net::find_timeout()),
            clt_reconnect_expected_recv
        );
        info!("clt_count_reconnect: {}", clt_count_reconnect);

        let found = clt_store_reconnect.find_recv(
            "clt_reconnect/soupbintcp/auto",
            |msg| {
                matches!(
                    msg,
                    UniSoupBinTcpMsg::Svc(SvcSoupBinTcpMsg::SPayload(SPayload {
                        payload ,
                        ..
                    }) ) if payload == &SamplePayload::new(format!("#{} SPayload", 10).as_bytes().into()) // last resent SPayload
                )
            },
            setup::net::optional_find_timeout().into(),
        );

        info!("found: {:?}", found);
        assert!(found.is_some());
        // info!("clt_store_reconnect: {}", clt_store_reconnect);

        drop(svc);

        let found = clt_store_reconnect.find_recv(
            "clt_reconnect/soupbintcp/auto",
            |msg| matches!(msg, UniSoupBinTcpMsg::Svc(SvcSoupBinTcpMsg::EndOfSession(_))),
            setup::net::optional_find_timeout().into(),
        );
        info!("found: {:?}", found);
        assert!(found.is_some());

        assert!(!clt_reconnect.is_connected());
    }
}

use crate::prelude::*;
// use spin::Mutex;
use std::{
    fmt::Debug,
    io::{Error, ErrorKind},
    marker::PhantomData,
    sync::Arc,
    time::Duration,
};

#[derive(Debug, Clone, Default)]
pub struct CltSoupBinTcpProtocolSupervised<RecvP: SoupBinTcpPayload<RecvP>, SendP: SoupBinTcpPayload<SendP>> {
    phantom: PhantomData<(RecvP, SendP)>,
}
impl<RecvP: SoupBinTcpPayload<RecvP>, SendP: SoupBinTcpPayload<SendP>> Framer for CltSoupBinTcpProtocolSupervised<RecvP, SendP> {
    #[inline(always)]
    fn get_frame_length(bytes: &mut bytes::BytesMut) -> Option<usize> {
        SoupBinTcpFramer::get_frame_length(bytes)
    }
}
impl<RecvP: SoupBinTcpPayload<RecvP>, SendP: SoupBinTcpPayload<SendP>> Messenger for CltSoupBinTcpProtocolSupervised<RecvP, SendP> {
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
//do nothing all Protocol methods are already implemented by default and do nothing so that they can be optimized by compiler
impl<RecvP: SoupBinTcpPayload<RecvP>, SendP: SoupBinTcpPayload<SendP>> Protocol for CltSoupBinTcpProtocolSupervised<RecvP, SendP> {}

#[derive(Debug, Clone, Default)]
pub struct SvcSoupBinTcpProtocolSupervised<RecvP: SoupBinTcpPayload<RecvP>, SendP: SoupBinTcpPayload<SendP>> {
    phantom: PhantomData<(RecvP, SendP)>,
}
impl<RecvP: SoupBinTcpPayload<RecvP>, SendP: SoupBinTcpPayload<SendP>> Framer for SvcSoupBinTcpProtocolSupervised<RecvP, SendP> {
    #[inline(always)]
    fn get_frame_length(bytes: &mut bytes::BytesMut) -> Option<usize> {
        SoupBinTcpFramer::get_frame_length(bytes)
    }
}
impl<RecvP: SoupBinTcpPayload<RecvP>, SendP: SoupBinTcpPayload<SendP>> Messenger for SvcSoupBinTcpProtocolSupervised<RecvP, SendP> {
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
impl<RecvP: SoupBinTcpPayload<RecvP>, SendP: SoupBinTcpPayload<SendP>> Protocol for SvcSoupBinTcpProtocolSupervised<RecvP, SendP> {}

/// Resets to None on clone
#[derive(Debug)]
struct SettableOption<T: Clone>(Arc<spin::Mutex<Option<T>>>);
impl<T: Clone> SettableOption<T> {
    fn set(&self, msg: T) {
        self.0.lock().replace(msg);
    }
    fn get(&self) -> Option<T> {
        self.0.lock().clone()
    }
}
impl<T: Clone> Default for SettableOption<T> {
    fn default() -> Self {
        Self(Arc::new(spin::Mutex::new(None)))
    }
}
impl<T: Clone> Clone for SettableOption<T> {
    fn clone(&self) -> Self {
        Self(Arc::new(spin::Mutex::new(None)))
    }
}

#[derive(Debug, Clone)]
pub struct CltSoupBinTcpProtocolAuth<RecvP: SoupBinTcpPayload<RecvP>, SendP: SoupBinTcpPayload<SendP>> {
    login: LoginRequest,
    timeout: Duration,
    login_accepted: SettableOption<LoginAccepted>,
    phantom: PhantomData<(RecvP, SendP)>,
}
impl<RecvP: SoupBinTcpPayload<RecvP>, SendP: SoupBinTcpPayload<SendP>> CltSoupBinTcpProtocolAuth<RecvP, SendP> {
    pub fn new(login: LoginRequest, timeout: Duration) -> Self {
        Self {
            login,
            timeout,
            login_accepted: Default::default(),
            phantom: PhantomData,
        }
    }
}
impl<RecvP: SoupBinTcpPayload<RecvP>, SendP: SoupBinTcpPayload<SendP>> Framer for CltSoupBinTcpProtocolAuth<RecvP, SendP> {
    #[inline(always)]
    fn get_frame_length(bytes: &mut bytes::BytesMut) -> Option<usize> {
        SoupBinTcpFramer::get_frame_length(bytes)
    }
}
impl<RecvP: SoupBinTcpPayload<RecvP>, SendP: SoupBinTcpPayload<SendP>> Messenger for CltSoupBinTcpProtocolAuth<RecvP, SendP> {
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
impl<RecvP: SoupBinTcpPayload<RecvP>, SendP: SoupBinTcpPayload<SendP>> Protocol for CltSoupBinTcpProtocolAuth<RecvP, SendP> {
    fn on_connected<M: Protocol<SendT = Self::SendT, RecvT = Self::RecvT>, C: CallbackRecvSend<M>, const MAX_MSG_SIZE: usize>(&self, clt: &mut Clt<M, C, MAX_MSG_SIZE>) -> Result<(), Error> {
        let mut msg = CltSoupBinTcpMsg::Login(self.login.clone());
        match clt.send_busywait_timeout(&mut msg, self.timeout)? {
            SendStatus::Completed => match clt.recv_busywait_timeout(self.timeout)? {
                RecvStatus::Completed(Some(SvcSoupBinTcpMsg::LoginAccepted(msg))) => {
                    self.login_accepted.set(msg);
                    Ok(())
                }
                RecvStatus::Completed(msg) => Err(Error::new(ErrorKind::Other, format!("Failed to login: {:?}", msg))),
                RecvStatus::WouldBlock => Err(Error::new(ErrorKind::TimedOut, format!("Failed to receive login: {:?}", msg))),
            },
            SendStatus::WouldBlock => Err(Error::new(ErrorKind::TimedOut, format!("Failed to send login: {:?}", msg))),
        }
    }
}

#[derive(Debug, Clone)]
pub struct SvcSoupBinTcpProtocolAuth<RecvP: SoupBinTcpPayload<RecvP>, SendP: SoupBinTcpPayload<SendP>> {
    username: UserName,
    password: Password,
    session_id: SessionId,
    timeout: Duration,
    login_request: SettableOption<LoginRequest>,
    phantom: PhantomData<(RecvP, SendP)>,
}
impl<RecvP: SoupBinTcpPayload<RecvP>, SendP: SoupBinTcpPayload<SendP>> SvcSoupBinTcpProtocolAuth<RecvP, SendP> {
    pub fn new(username: UserName, password: Password, session_id: SessionId, timeout: Duration) -> Self {
        Self {
            username,
            password,
            session_id,
            timeout,
            login_request: Default::default(),
            phantom: PhantomData,
        }
    }
}
impl<RecvP: SoupBinTcpPayload<RecvP>, SendP: SoupBinTcpPayload<SendP>> Framer for SvcSoupBinTcpProtocolAuth<RecvP, SendP> {
    #[inline(always)]
    fn get_frame_length(bytes: &mut bytes::BytesMut) -> Option<usize> {
        SoupBinTcpFramer::get_frame_length(bytes)
    }
}
impl<RecvP: SoupBinTcpPayload<RecvP>, SendP: SoupBinTcpPayload<SendP>> Messenger for SvcSoupBinTcpProtocolAuth<RecvP, SendP> {
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
impl<RecvP: SoupBinTcpPayload<RecvP>, SendP: SoupBinTcpPayload<SendP>> Protocol for SvcSoupBinTcpProtocolAuth<RecvP, SendP> {
    fn on_connected<M: Protocol<SendT = Self::SendT, RecvT = Self::RecvT>, C: CallbackRecvSend<M>, const MAX_MSG_SIZE: usize>(&self, clt: &mut Clt<M, C, MAX_MSG_SIZE>) -> Result<(), Error> {
        match clt.recv_busywait_timeout(self.timeout)? {
            RecvStatus::Completed(Some(CltSoupBinTcpMsg::Login(msg))) => {
                if msg.username == self.username && msg.password == self.password && msg.session_id == self.session_id {
                    self.login_request.set(msg);
                    let mut msg = LoginAccepted::default().into();
                    match clt.send_busywait_timeout(&mut msg, self.timeout)? {
                        SendStatus::Completed => Ok(()),
                        SendStatus::WouldBlock => Err(Error::new(ErrorKind::TimedOut, format!("Failed to send login: {:?}", msg))),
                    }
                } else if msg.session_id != self.session_id {
                    clt.send_busywait_timeout(&mut LoginRejected::session_not_available().into(), self.timeout)?;
                    Err(Error::new(ErrorKind::NotConnected, format!("Invalid session_id msg: {:?}", msg)))
                } else {
                    clt.send_busywait_timeout(&mut LoginRejected::not_authorized().into(), self.timeout)?;
                    Err(Error::new(ErrorKind::NotConnected, format!("Not Authorized msg: {:?}", msg)))
                }
            }
            RecvStatus::Completed(msg) => Err(Error::new(ErrorKind::Other, format!("Expected LoginRequest instead got msg:{:?}", msg))),
            RecvStatus::WouldBlock => Err(Error::new(ErrorKind::TimedOut, format!("Did not get LoginRequest during timeout: {:?}", self.timeout))),
        }
    }
}

#[cfg(test)]
#[cfg(feature = "unittest")]
mod test {

    use crate::prelude::*;
    use std::num::NonZeroUsize;

    use links_core::unittest::setup;
    use log::info;
    type CltProtocolSupervised = CltSoupBinTcpProtocolSupervised<SamplePayload, SamplePayload>;
    type SvcProtocolSupervised = SvcSoupBinTcpProtocolSupervised<SamplePayload, SamplePayload>;
    type CltProtocolAuth = CltSoupBinTcpProtocolAuth<SamplePayload, SamplePayload>;
    type SvcProtocolAuth = SvcSoupBinTcpProtocolAuth<SamplePayload, SamplePayload>;

    #[test]
    fn test_connect_supervised() {
        setup::log::configure_compact(log::LevelFilter::Debug);
        const SOUP_BIN_MAX_FRAME_SIZE: usize = SOUPBINTCP_MAX_FRAME_SIZE_EXCLUDING_PAYLOAD_DEBUG;
        let clt_clbk = CounterCallback::new_ref();
        let svc_clbk = CounterCallback::new_ref();
        let addr = setup::net::rand_avail_addr_port();

        let mut svc = Svc::<_, _, SOUP_BIN_MAX_FRAME_SIZE>::bind(addr, svc_clbk.clone(), NonZeroUsize::new(1).unwrap(), Some(SvcProtocolSupervised::default()), Some("soupbintcp/supervised/unittest"))
            .unwrap()
            .into_spawned_sender();
        let _clt = Clt::<_, _, SOUP_BIN_MAX_FRAME_SIZE>::connect(
            addr,
            setup::net::default_connect_timeout(),
            setup::net::default_connect_retry_after(),
            clt_clbk.clone(),
            Some(CltProtocolSupervised::default()),
            Some("soupbintcp/supervised/unittest"),
        )
        .unwrap()
        .into_spawned_sender();

        // client should not send any messages to perform login
        assert_eq!(clt_clbk.sent_count(), 0);
        // server should also not sent any messages but to make sure connection was established sending a hbeat
        svc.send_busywait_timeout(&mut SvcHeartbeat::default().into(), setup::net::default_connect_timeout()).unwrap().unwrap_completed();
        assert_eq!(svc_clbk.sent_count(), 1);
    }
    #[test]
    fn test_connect_auth() {
        setup::log::configure_compact(log::LevelFilter::Info);

        const SOUP_BIN_MAX_FRAME_SIZE: usize = SOUPBINTCP_MAX_FRAME_SIZE_EXCLUDING_PAYLOAD_DEBUG;
        let addr = setup::net::rand_avail_addr_port();

        let clt_count = CounterCallback::new_ref();
        let svc_count = CounterCallback::new_ref();
        let clt_clbk = ChainCallback::new_ref(vec![clt_count.clone(), LoggerCallback::with_level_ref(log::Level::Info, log::Level::Debug)]);
        let svc_clbk = ChainCallback::new_ref(vec![svc_count.clone(), LoggerCallback::with_level_ref(log::Level::Info, log::Level::Debug)]);

        let login = LoginRequest::default();
        // let clt_ptcl = CltProtocolAuth::new(login.clone(), setup::net::default_connect_timeout());
        // let svc_ptcl = SvcProtocolAuth::new(login.username, login.password, login.session_id, setup::net::default_connect_timeout());
        let _svc = Svc::<_, _, SOUP_BIN_MAX_FRAME_SIZE>::bind(
            addr,
            svc_clbk.clone(),
            NonZeroUsize::new(1).unwrap(),
            Some(SvcProtocolAuth::new(login.username, login.password, login.session_id, setup::net::default_connect_timeout())),
            Some("soupbintcp/auth/unittest"),
        )
        .unwrap()
        .into_spawned_sender();
        let _clt = Clt::<_, _, SOUP_BIN_MAX_FRAME_SIZE>::connect(
            addr,
            setup::net::default_connect_timeout(),
            setup::net::default_connect_retry_after(),
            clt_clbk.clone(),
            Some(CltProtocolAuth::new(login.clone(), setup::net::default_connect_timeout())),
            Some("soupbintcp/auth/unittest"),
        )
        .unwrap()
        .into_spawned_sender();

        // this indicates client sent login request
        assert_eq!(clt_count.sent_count(), 1);
        // this indicates server sent login accepted
        assert_eq!(svc_count.sent_count(), 1);

        // second connection shall fail
        let res = Clt::<_, _, SOUP_BIN_MAX_FRAME_SIZE>::connect(
            addr,
            setup::net::default_connect_timeout(),
            setup::net::default_connect_retry_after(),
            clt_clbk.clone(),
            Some(CltProtocolAuth::new(login.clone(), setup::net::default_connect_timeout())),
            Some("#2-soupbintcp/auth/unittest"),
        ); // TODO ensure this fails by adding rate limiter to svc and max connection check in the acceptor rather then pool or poll, shall poll & pool also have max check if check moved to acceptor
        info!("res: {:?}", res);
        assert!(res.is_err());
        assert!(res.unwrap_err().kind() == std::io::ErrorKind::ConnectionReset);
    }
}

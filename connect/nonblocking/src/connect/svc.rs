use crate::prelude::*;

pub type SvcSoupBinTcpManual<RecvP, SendP, C, const MAX_MSG_SIZE: usize> = Svc<SvcSoupBinTcpProtocolManual<RecvP, SendP>, C, MAX_MSG_SIZE>;
pub type SvcSoupBinTcpAuto<RecvP, SendP, C, const MAX_MSG_SIZE: usize> = Svc<SvcSoupBinTcpProtocolAuto<RecvP, SendP>, C, MAX_MSG_SIZE>;

// pub type SvcSoupBinTcpAcceptorManual<RecvP, SendP, C, const MAX_MSG_SIZE: usize> = SvcAcceptor<SvcSoupBinTcpProtocolManual<RecvP, SendP>, C, MAX_MSG_SIZE>;
// pub type SvcSoupBinTcpAcceptorAuto<RecvP, SendP, C, const MAX_MSG_SIZE: usize> = SvcAcceptor<SvcSoupBinTcpProtocolAuto<RecvP, SendP>, C, MAX_MSG_SIZE>;

pub type SvcSoupBinTcpRecver<RecvP, SendP, C, const MAX_MSG_SIZE: usize> = CltRecver<SvcSoupBinTcpMessenger<RecvP, SendP>, C, MAX_MSG_SIZE>;
pub type SvcSoupBinTcpSender<RecvP, SendP, C, const MAX_MSG_SIZE: usize> = CltSender<SvcSoupBinTcpMessenger<RecvP, SendP>, C, MAX_MSG_SIZE>;

#[cfg(test)]
#[cfg(feature = "unittest")]
mod test {

    use std::num::NonZeroUsize;

    use crate::prelude::*;
    use links_core::unittest::setup;
    use log::info;

    #[test]
    fn test_clt_svc_connected() {
        setup::log::configure_compact(log::LevelFilter::Info);

        let addr = setup::net::rand_avail_addr_port();

        let mut svc = SvcSoupBinTcpManual::<_, _, _, 128>::bind(addr, LoggerCallback::new_ref(), NonZeroUsize::new(1).unwrap(), SvcSoupBinTcpProtocolManual::<Nil, Nil>::default(), Some("soupbintcp/unittest")).unwrap();
        info!("svc: {}", svc);

        let mut clt = CltSoupBinTcpManual::<_, _, _, 128>::connect(
            addr,
            setup::net::default_connect_timeout(),
            setup::net::default_connect_retry_after(),
            LoggerCallback::new_ref(),
            CltSoupBinTcpProtocolManual::<Nil, Nil>::default(),
            Some("soupbintcp/unittest"),
        )
        .unwrap();
        info!("clt: {}", clt);

        svc.accept_into_pool_busywait_timeout(setup::net::default_connect_timeout()).unwrap().unwrap_accepted();
        info!("svc: {}", svc);

        let mut clt_msg = CltSoupBinTcpMsg::Login(LoginRequest::default());
        clt.send_busywait_timeout(&mut clt_msg, setup::net::default_connect_timeout()).unwrap().unwrap_completed();

        let svc_msg = svc.recv_busywait_timeout(setup::net::default_connect_timeout()).unwrap().unwrap_completed_some();

        assert_eq!(clt_msg, svc_msg);
    }
}

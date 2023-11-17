use crate::prelude::*;


pub type SvcSoupBinTcpSupervised<RecvP, SendP, C, const MAX_MSG_SIZE: usize> = Svc<SvcSoupBinTcpMessenger<RecvP, SendP>, C, MAX_MSG_SIZE>;
pub type SvcSoupBinTcpAcceptor<RecvP, SendP, C, const MAX_MSG_SIZE: usize> = SvcAcceptor<SvcSoupBinTcpMessenger<RecvP, SendP>, C, MAX_MSG_SIZE>;
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
        setup::log::configure_level(log::LevelFilter::Info);

        let addr = setup::net::rand_avail_addr_port();

        let mut svc = SvcSoupBinTcpSupervised::<Nil, Nil, _, 128>::bind(addr, LoggerCallback::new_ref(), NonZeroUsize::new(1).unwrap(), Some("soupbintcp/unittest")).unwrap();
        info!("svc: {}", svc);

        let mut clt = CltSoupBinTcpSupervised::<Nil, Nil, _, 128>::connect(addr, setup::net::default_connect_timeout(), setup::net::default_connect_retry_after(), LoggerCallback::new_ref(), Some("soupbintcp/unittest")).unwrap();
        info!("clt: {}", clt);

        svc.pool_accept_busywait_timeout(setup::net::default_connect_timeout()).unwrap().unwrap_accepted();
        info!("svc: {}", svc);

        let mut clt_msg = CltSoupBinTcpMsg::Login(LoginRequest::default());
        clt.send_busywait_timeout(&mut clt_msg, setup::net::default_connect_timeout()).unwrap().unwrap();

        let svc_msg = svc.recv_busywait_timeout(setup::net::default_connect_timeout()).unwrap().unwrap();

        assert_eq!(clt_msg, svc_msg);
    }
}

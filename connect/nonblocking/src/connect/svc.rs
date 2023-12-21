use crate::prelude::*;

pub type SvcSoupBinTcp<P, C, const MAX_MSG_SIZE: usize> = Svc<P, C, MAX_MSG_SIZE>;

pub type SvcSoupBinTcpSender<P, C, const MAX_MSG_SIZE: usize> = SvcSender<P, C, MAX_MSG_SIZE>;
pub type SvcSoupBinTcpSenderRef<P, C, const MAX_MSG_SIZE: usize> = SvcSenderRef<P, C, MAX_MSG_SIZE>;

#[cfg(test)]
#[cfg(feature = "unittest")]
mod test {

    use crate::prelude::*;
    use links_core::unittest::setup;
    use log::info;
    use std::num::NonZeroUsize;

    #[test]
    fn test_clt_svc_connected() {
        setup::log::configure_compact(log::LevelFilter::Info);

        let addr = setup::net::rand_avail_addr_port();

        let mut svc = SvcSoupBinTcp::<_, _, 128>::bind(
            addr,
            NonZeroUsize::new(1).unwrap(),
            LoggerCallback::new_ref(),
            SvcSoupBinTcpProtocolIsConnected::<Nil, Nil>::default(),
            Some("soupbintcp/unittest"),
        )
        .unwrap();
        info!("svc: {}", svc);

        let mut clt = CltSoupBinTcp::<_, _, 128>::connect(
            addr,
            setup::net::default_connect_timeout(),
            setup::net::default_connect_retry_after(),
            LoggerCallback::new_ref(),
            CltSoupBinTcpProtocolIsConnected::<Nil, Nil>::default(),
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

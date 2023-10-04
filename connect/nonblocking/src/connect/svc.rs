use crate::prelude::*;

pub type SvcSoupBinTcp<M, C, const MAX_MSG_SIZE: usize> = Svc<M, C, MAX_MSG_SIZE>;

#[cfg(test)]
#[cfg(feature = "unittest")]

mod test {

    use std::num::NonZeroUsize;

    use crate::{connect::svc::SvcSoupBinTcp, prelude::*};
    use links_core::unittest::setup;
    use log::info;

    #[test]
    fn test_clt_svc_connected() {
        setup::log::configure_level(log::LevelFilter::Info);

        let addr = setup::net::rand_avail_addr_port();

        let mut svc = SvcSoupBinTcp::<_, _, 128>::bind(
            addr,
            DevNullCallback::<SvcSoupBinTcpMessenger<Nil, Nil>>::new_ref(),
            NonZeroUsize::new(1).unwrap(),
            Some("soupbin/unittest"),
        )
        .unwrap();
        info!("svc: {}", svc);

        let mut clt = CltSoupBinTcp::<_, _, 128>::connect(
            addr,
            setup::net::default_connect_timeout(),
            setup::net::default_connect_retry_after(),
            DevNullCallback::<CltSoupBinTcpMessenger<Nil, Nil>>::new_ref(),
            Some("soupbin/unittest"),
        )
        .unwrap();
        info!("clt: {}", clt);

        svc.pool_accept_busywait_timeout(setup::net::default_connect_timeout())
            .unwrap()
            .unwrap();

        let mut clt_msg = CltSoupBinTcpMsg::Login(LoginRequest::default());
        clt.send_busywait_timeout(&mut clt_msg, setup::net::default_connect_timeout())
            .unwrap()
            .unwrap();

        info!("clt_msg: {:?}", clt_msg);

        let svc_msg = svc
            .recv_busywait_timeout(setup::net::default_connect_timeout())
            .unwrap()
            .unwrap();

        info!("svc_msg: {:?}", svc_msg);
    }
}

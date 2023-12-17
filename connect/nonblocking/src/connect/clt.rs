use crate::prelude::*;

pub type CltSoupBinTcp<P, C, const MAX_MSG_SIZE: usize> = Clt<P, C, MAX_MSG_SIZE>;

#[cfg(test)]
#[cfg(feature = "unittest")]
mod test {

    use crate::prelude::*;
    use links_core::unittest::setup;
    use log::info;

    #[test]
    fn test_clt_not_connected() {
        setup::log::configure();

        let addr = setup::net::rand_avail_addr_port();

        let res = CltSoupBinTcp::<_, _, 128>::connect(
            addr,
            setup::net::default_connect_timeout(),
            setup::net::default_connect_retry_after(),
            DevNullCallback::new_ref(),
            CltSoupBinTcpProtocolManual::<Nil, Nil>::default(),
            Some("soupbintcp/unittest"),
        );
        info!("{:?} not connected", res);
        assert!(res.is_err());
    }
}

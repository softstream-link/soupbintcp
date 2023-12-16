use crate::prelude::*;

/// SoupBinTCP client that uses manual [CltSoupBinTcpProtocolManual] protocol.
pub type CltSoupBinTcpManual<RecvP, SendP, C, const MAX_MSG_SIZE: usize> = Clt<CltSoupBinTcpProtocolManual<RecvP, SendP>, C, MAX_MSG_SIZE>;
/// SoupBinTCP client that uses auto [CltSoupBinTcpProtocolAuto] protocol.
pub type CltSoupBinTcpAuto<RecvP, SendP, C, const MAX_MSG_SIZE: usize> = Clt<CltSoupBinTcpProtocolAuto<RecvP, SendP>, C, MAX_MSG_SIZE>;

// pub type CltSoupBinTcpRecver<RecvP, SendP, C, const MAX_MSG_SIZE: usize> = CltRecver<CltSoupBinTcpMessenger<RecvP, SendP>, C, MAX_MSG_SIZE>;
// pub type CltSoupBinTcpSender<RecvP, SendP, C, const MAX_MSG_SIZE: usize> = CltSender<CltSoupBinTcpMessenger<RecvP, SendP>, C, MAX_MSG_SIZE>;

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

        let res = CltSoupBinTcpManual::<_, _, _, 128>::connect(
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

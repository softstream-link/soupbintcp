pub mod setup {
    pub mod model {
        use crate::prelude::*;

        #[rustfmt::skip]
        pub fn svc_msgs_default<P>() -> Vec<SvcSoupBinTcpMsg<P>>
        where
            P: SoupBinTcpPayload<P> + Default,
        {
            vec![
                SvcSoupBinTcpMsg::HBeat(SvcHeartbeat::default()),
                SvcSoupBinTcpMsg::Dbg(Debug::default()),
                SvcSoupBinTcpMsg::LoginAcc(LoginAccepted::default()),
                SvcSoupBinTcpMsg::LoginRej(LoginRejected::not_authorized()),
                SvcSoupBinTcpMsg::End(EndOfSession::default()),
                SvcSoupBinTcpMsg::S(SPayload::new(P::default())),
                SvcSoupBinTcpMsg::U(UPayload::new(P::default())),
            ]
        }

        #[rustfmt::skip]
        pub fn clt_msgs_default<P>() -> Vec<CltSoupBinTcpMsg<P>>
        where
            P: SoupBinTcpPayload<P> + Default,
        {
            vec![
                CltSoupBinTcpMsg::HBeat(CltHeartbeat::default()),
                CltSoupBinTcpMsg::Dbg(Debug::default()),
                CltSoupBinTcpMsg::Login(LoginRequest::default()),
                CltSoupBinTcpMsg::Logout(LogoutRequest::default()),
                CltSoupBinTcpMsg::S(SPayload::new(P::default())),
                CltSoupBinTcpMsg::U(UPayload::new(P::default())),
            ]
        }
    }
}

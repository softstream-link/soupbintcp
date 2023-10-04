pub mod setup {
    pub mod log {
        use std::sync::Once;
        static SETUP: Once = Once::new();
        pub fn configure() {
            configure_level(log::LevelFilter::Trace)
        }
        pub fn configure_level(level: log::LevelFilter) {
            SETUP.call_once(|| {
                use colored::*;
                use std::io::Write;
                let _ = env_logger::builder()
                    .format(|buf, record| {
                        let ts = buf.timestamp_nanos();
                        let level = match record.level() {
                            log::Level::Error => "ERROR".red(),
                            log::Level::Warn => "WARN ".yellow(),
                            log::Level::Info => "INFO ".green(),
                            log::Level::Debug => "DEBUG".blue(),
                            log::Level::Trace => "TRACE".blue(),
                        };
                        let target = record.target();
                        let args = record.args();
                        let thread = std::thread::current();
                        let id = thread.id();
                        let name = thread
                            .name()
                            .unwrap_or(format!("Thread-{id:?}").as_str())
                            .to_owned();
                        writeln!(buf, "{ts} {level} ({name}) {target} {args}")
                    })
                    // .format_timestamp_micro s()
                    .is_test(false) // disables color in the terminal
                    .filter_level(level)
                    .try_init();
            });
        }
    }
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

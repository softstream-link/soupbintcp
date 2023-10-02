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
        use byteserde::prelude::*;

        #[rustfmt::skip]
        pub fn svc_msgs_default<SvcPayload>() -> Vec<SBSvcMsg<SvcPayload>>
        where
            SvcPayload: ByteSerializeStack + ByteDeserializeSlice<SvcPayload> + ByteSerializedLenOf + PartialEq + Clone + Default + std::fmt::Debug,
        {
            vec![
                SBSvcMsg::HBeat(SvcHeartbeat::default()),
                SBSvcMsg::Dbg(Debug::default()),
                SBSvcMsg::LoginAcc(LoginAccepted::default()),
                SBSvcMsg::LoginRej(LoginRejected::not_authorized()),
                SBSvcMsg::End(EndOfSession::default()),
                SBSvcMsg::S(SPayload::new(SvcPayload::default())),
                SBSvcMsg::U(UPayload::new(SvcPayload::default())),
            ]
        }

        #[rustfmt::skip]
        pub fn clt_msgs_default<T>() -> Vec<SBCltMsg<T>>
        where
            T: ByteSerializeStack + ByteDeserializeSlice<T> + ByteSerializedLenOf + PartialEq + Clone + Default + std::fmt::Debug,
        {
            vec![
                SBCltMsg::HBeat(CltHeartbeat::default()),
                SBCltMsg::Dbg(Debug::default()),
                SBCltMsg::Login(LoginRequest::default()),
                SBCltMsg::Logout(LogoutRequest::default()),
                SBCltMsg::S(SPayload::new(T::default())),
                SBCltMsg::U(UPayload::new(T::default())),
            ]
        }
        
    }
}

use soupbintcp_connect_core::{
    core::messenger::CltSoupBinTcpMessenger,
    prelude::{CltSendersPool, SvcPoolAcceptor, SvcSoupBinTcpMessenger, CltRecversPool},
};

pub type SvcSoupBinTcpPoolAcceptor<RecvP, SendP, C, const MAX_MSG_SIZE: usize> = SvcPoolAcceptor<SvcSoupBinTcpMessenger<RecvP, SendP>, C, MAX_MSG_SIZE>;

/// Pool of [crate::prelude::CltSoupBinTcpRecver] connections.
pub type CltSoupBinTcpRecversPool<RecvP, SendP, C, const MAX_MSG_SIZE: usize> = CltRecversPool<CltSoupBinTcpMessenger<RecvP, SendP>, C, MAX_MSG_SIZE>;
/// Pool of [crate::prelude::CltSoupBinTcpSender] connections.
pub type CltSoupBinTcpSendersPool<RecvP, SendP, C, const MAX_MSG_SIZE: usize> = CltSendersPool<CltSoupBinTcpMessenger<RecvP, SendP>, C, MAX_MSG_SIZE>;



/// Pool of [crate::prelude::SvcSoupBinTcpRecver] connections.
pub type SvcSoupBinTcpRecversPool<RecvP, SendP, C, const MAX_MSG_SIZE: usize> = CltRecversPool<SvcSoupBinTcpMessenger<RecvP, SendP>, C, MAX_MSG_SIZE>;
/// Pool of [crate::prelude::SvcSoupBinTcpSender] connections.
pub type SvcSoupBinTcpSendersPool<RecvP, SendP, C, const MAX_MSG_SIZE: usize> = CltSendersPool<SvcSoupBinTcpMessenger<RecvP, SendP>, C, MAX_MSG_SIZE>;

use soupbintcp_connect_core::{
    core::messenger::CltSoupBinTcpProtocolSupervised,
    prelude::{CltSendersPool, SvcPoolAcceptor, SvcSoupBinTcpProtocolSupervised, CltRecversPool},
};

pub type SvcSoupBinTcpPoolAcceptor<RecvP, SendP, C, const MAX_MSG_SIZE: usize> = SvcPoolAcceptor<SvcSoupBinTcpProtocolSupervised<RecvP, SendP>, C, MAX_MSG_SIZE>;

/// Pool of [crate::prelude::CltSoupBinTcpRecver] connections.
pub type CltSoupBinTcpRecversPool<RecvP, SendP, C, const MAX_MSG_SIZE: usize> = CltRecversPool<CltSoupBinTcpProtocolSupervised<RecvP, SendP>, C, MAX_MSG_SIZE>;
/// Pool of [crate::prelude::CltSoupBinTcpSender] connections.
pub type CltSoupBinTcpSendersPool<RecvP, SendP, C, const MAX_MSG_SIZE: usize> = CltSendersPool<CltSoupBinTcpProtocolSupervised<RecvP, SendP>, C, MAX_MSG_SIZE>;



/// Pool of [crate::prelude::SvcSoupBinTcpRecver] connections.
pub type SvcSoupBinTcpRecversPool<RecvP, SendP, C, const MAX_MSG_SIZE: usize> = CltRecversPool<SvcSoupBinTcpProtocolSupervised<RecvP, SendP>, C, MAX_MSG_SIZE>;
/// Pool of [crate::prelude::SvcSoupBinTcpSender] connections.
pub type SvcSoupBinTcpSendersPool<RecvP, SendP, C, const MAX_MSG_SIZE: usize> = CltSendersPool<SvcSoupBinTcpProtocolSupervised<RecvP, SendP>, C, MAX_MSG_SIZE>;

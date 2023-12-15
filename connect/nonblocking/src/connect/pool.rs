// use soupbintcp_connect_core::{
//     core::messenger::CltSoupBinTcpMessenger,
//     prelude::{CltRecver, CltRecversPool, CltSendersPool, SvcSoupBinTcpProtocolSupervised},
// };

// pub type SvcSoupBinTcpPoolAcceptor<P, C, const MAX_MSG_SIZE: usize> = SvcPoolAcceptor<P, C, MAX_MSG_SIZE>;

// /// Pool of [crate::prelude::CltSoupBinTcpRecver] connections.
// pub type CltSoupBinTcpRecversPool<RecvP, SendP, C, const MAX_MSG_SIZE: usize> = CltRecversPool<CltSoupBinTcpMessenger<RecvP, SendP>, C, MAX_MSG_SIZE, CltRecver<>>;
// /// Pool of [crate::prelude::CltSoupBinTcpSender] connections.
// pub type CltSoupBinTcpSendersPool<RecvP, SendP, C, const MAX_MSG_SIZE: usize> = CltSendersPool<CltSoupBinTcpMessenger<RecvP, SendP>, C, MAX_MSG_SIZE>;

// /// Pool of [crate::prelude::SvcSoupBinTcpRecver] connections.
// pub type SvcSoupBinTcpRecversPool<RecvP, SendP, C, const MAX_MSG_SIZE: usize> = CltRecversPool<SvcSoupBinTcpProtocolSupervised<RecvP, SendP>, C, MAX_MSG_SIZE>;
// /// Pool of [crate::prelude::SvcSoupBinTcpSender] connections.
// pub type SvcSoupBinTcpSendersPool<RecvP, SendP, C, const MAX_MSG_SIZE: usize> = CltSendersPool<SvcSoupBinTcpProtocolSupervised<RecvP, SendP>, C, MAX_MSG_SIZE>;

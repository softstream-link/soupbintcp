use soupbintcp_connect_core::prelude::{SvcPoolAcceptor, SvcSoupBinTcpMessenger};

pub type SvcSoupBinTcpPoolAcceptor<RecvP, SendP, C, const MAX_MSG_SIZE: usize> = SvcPoolAcceptor<SvcSoupBinTcpMessenger<RecvP, SendP>, C, MAX_MSG_SIZE>;

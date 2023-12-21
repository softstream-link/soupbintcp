pub mod auto;
pub mod is_connected;
pub mod manual;

use crate::prelude::*;
use std::{
    fmt::Debug,
    time::{Duration, Instant},
};

/// Helper to establish connection status of Clt, uses all recved messages to check.
/// Confirms that last message received with in `max_recv_interval` duration.
/// In addition, confirms that [LoginAccepted] was received and [EndOfSession] was not received.
#[derive(Debug, Clone)]
pub struct CltSoupBinTcpRecvConnectionState {
    max_recv_interval: Duration,
    login_accepted: Option<Instant>,
    login_rejected: Option<Instant>,
    end_of_session: Option<Instant>,
    any_msg_recved: Option<Instant>,
}
impl CltSoupBinTcpRecvConnectionState {
    pub fn new(max_recv_interval: Duration) -> Self {
        Self {
            max_recv_interval,
            login_accepted: None,
            login_rejected: None,
            end_of_session: None,
            any_msg_recved: None,
        }
    }
    #[inline(always)]
    pub fn on_recv<RecvP: SoupBinTcpPayload<RecvP>>(&mut self, msg: &SvcSoupBinTcpMsg<RecvP>) {
        use SvcSoupBinTcpMsg::*;
        let now = Instant::now();
        match msg {
            LoginAccepted(_) => self.login_accepted = Some(now),
            LoginRejected(_) => self.login_rejected = Some(now),
            EndOfSession(_) => self.end_of_session = Some(now),
            _ => {}
        }
        self.any_msg_recved = Some(now);
    }
}
impl ConnectionStatus for CltSoupBinTcpRecvConnectionState {
    /// Will returns `true` if all of below are `true`
    /// * [LoginAccepted] was received
    /// * time elapsed from the last message received is less then `max_recv_interval` which is an argument of [`Self::new`]
    /// * [LoginRejected] was `NOT` received
    /// * [EndOfSession] was `NOT` received
    fn is_connected(&self) -> bool {
        match (self.login_accepted, self.any_msg_recved, self.login_rejected, self.end_of_session) {
            (Some(_), Some(any_msg_recved), None, None) => any_msg_recved.elapsed() < self.max_recv_interval,
            _ => false,
        }
    }
}
impl From<CltSoupBinTcpRecvConnectionState> for ProtocolConnectionState<CltSoupBinTcpRecvConnectionState> {
    fn from(state: CltSoupBinTcpRecvConnectionState) -> Self {
        ProtocolConnectionState::new(state)
    }
}

/// Helper to establish connection status of Svc, uses all recved messages to check.
/// Confirms that last message received with in `max_recv_interval` duration.
#[derive(Debug, Clone, Default)]
pub struct SvcSoupBinTcpRecvConnectionState {
    max_recv_interval: Option<Duration>, // arrives from client in LoginRequest
    any_msg_recved: Option<Instant>,
}
impl SvcSoupBinTcpRecvConnectionState {
    #[inline(always)]
    pub fn on_recv<RecvP: SoupBinTcpPayload<RecvP>>(&mut self, msg: &CltSoupBinTcpMsg<RecvP>) {
        use CltSoupBinTcpMsg::*;
        let now = Instant::now();
        if let Login(msg) = msg {
            self.max_recv_interval = Some(msg.hbeat_timeout_ms.into())
        }
        // match msg {
        //     Login(msg) => self.max_recv_interval = Some(msg.hbeat_timeout_ms.into()),
        //     _ => {}
        // }
        self.any_msg_recved = Some(now);
    }
}
impl ConnectionStatus for SvcSoupBinTcpRecvConnectionState {
    /// Will returns `true` if all of below are `true`
    /// * [LoginRequest] was received
    /// * time elapsed from the last message received is less then `max_recv_interval` which is determine by
    /// [LoginRequest::hbeat_timeout_ms] from client side.
    fn is_connected(&self) -> bool {
        match (self.any_msg_recved, self.max_recv_interval) {
            (Some(any_msg_recved), Some(max_recv_interval)) => any_msg_recved.elapsed() < max_recv_interval,
            _ => false,
        }
    }
}
impl From<SvcSoupBinTcpRecvConnectionState> for ProtocolConnectionState<SvcSoupBinTcpRecvConnectionState> {
    fn from(state: SvcSoupBinTcpRecvConnectionState) -> Self {
        ProtocolConnectionState::new(state)
    }
}

/// Helper to establish connection status of Svc, uses all sent messages to check.
/// Confirms that [LoginAccepted] was sent and [EndOfSession] was not sent.
#[derive(Debug, Clone, Default)]
pub struct SvcSoupBinTcpSendConnectionState {
    login_accepted: Option<Instant>,
    end_of_session: Option<Instant>,
}
impl SvcSoupBinTcpSendConnectionState {
    #[inline(always)]
    pub fn on_sent<SendP: SoupBinTcpPayload<SendP>>(&mut self, msg: &SvcSoupBinTcpMsg<SendP>) {
        use SvcSoupBinTcpMsg::*;
        let now = Instant::now();
        match msg {
            LoginAccepted(_) => self.login_accepted = Some(now),
            EndOfSession(_) => self.end_of_session = Some(now),
            _ => {}
        }
    }
}
impl ConnectionStatus for SvcSoupBinTcpSendConnectionState {
    /// Will returns `true` if all of below are `true`
    /// * [LoginAccepted] was sent
    /// * [EndOfSession] was NOT sent
    fn is_connected(&self) -> bool {
        matches!((self.login_accepted, self.end_of_session), (Some(_), None))
    }
}
impl From<SvcSoupBinTcpSendConnectionState> for ProtocolConnectionState<SvcSoupBinTcpSendConnectionState> {
    fn from(state: SvcSoupBinTcpSendConnectionState) -> Self {
        ProtocolConnectionState::new(state)
    }
}

#[derive(Debug)]
pub struct SvcSoupBinTcpSendSessionState<SendP: SoupBinTcpPayload<SendP>, Storage: ProtocolStorage<Item = SvcSoupBinTcpMsg<SendP>>> {
    sequenced_payload_number: usize,
    storage: Storage,
}
impl<SendP: SoupBinTcpPayload<SendP>, Storage: ProtocolStorage<Item = SvcSoupBinTcpMsg<SendP>>> SvcSoupBinTcpSendSessionState<SendP, Storage> {
    pub fn new(storage: Storage) -> Self {
        Self { sequenced_payload_number: 0, storage }
    }
    #[inline(always)]
    pub fn on_sent(&mut self, msg: &SvcSoupBinTcpMsg<SendP>) {
        if let SvcSoupBinTcpMsg::SPayload(_) = msg {
            self.sequenced_payload_number += 1;
        }
        self.storage.store(msg.clone());
    }
    #[inline(always)]
    pub fn current_sequence_payload_number(&self) -> usize {
        self.sequenced_payload_number
    }
    #[inline(always)]
    pub fn next_sequenced_payload_number(&self) -> usize {
        self.sequenced_payload_number + 1
    }
    #[inline(always)]
    pub fn get_storage(&self) -> &Storage {
        &self.storage
    }
}
// impl<SendP: SoupBinTcpPayload<SendP>,  Storage: ProtocolStorage<Item = SvcSoupBinTcpMsg<SendP>> > From<SvcSoupBinTcpSendSessionState<SendP, Storage> for ProtocolSessionState<SvcSoupBinTcpSendSessionState<SendP, Storage>> {
//     fn from(state: SvcSoupBinTcpSendSessionState<SendP, Storage>) -> Self {
//         ProtocolSessionState::new(state)
//     }
// }

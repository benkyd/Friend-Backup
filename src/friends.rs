// friends.rs
// Here we have the sate machine for each friend.
// The negotiation between friends and client
// should be a state machine

// P2P
// - Both sides send `Hello { node }` on connect. Nobody is client/server.
// - Either side may propose from `Idle`.

use std::{ops::Neg, sync::WaitTimeoutResult};

use serde::{Deserialize, Serialize};

const DEFAULT_CHUNK_SIZE: u32 = 64 * 1024;

pub type NodeID = [u8; 32];


#[derive(Debug, Clone, Copy)]
pub enum Direction {
    Inbound,
    Outbound,
}

#[derive(Debug)]
pub enum Proposal {
    Transfer { dir: Direction },
    ChunkSize(u32),
}

#[derive(Debug)]
pub enum Message {
    Hello { node: NodeID },
    Propose { proposal: Proposal },
    Accept,
    Reject,
    Done,
    Fail,
    Bye,
}

#[derive(Debug)]
pub enum State {
    Idle,
    Negotiating { pending: Option<Proposal>, chunk_size: u32 },
    Transferring { dir: Direction, chunk_size: u32 },
    Close,
}

#[derive(Debug)]
pub enum ProtocolError {
    Unexpected(Message),
}

pub struct Friend {
    me: NodeID,
    them: Option<NodeID>,
    state: State,
}

impl Friend {
    pub fn new(me: NodeID) -> Self {
        Friend { me, them: None, state: State::Idle }
    }

    pub fn state(&self) -> &State {
        &self.state
    }

    // Idle -> Negotiating -> (Transferring ->) Close -> Idle
    pub fn step(&mut self, msg: Message) -> Result<(), ProtocolError> {
        let current = std::mem::replace(&mut self.state, State::Close);

        self.state = match (current, msg) {
            (State::Idle, Message::Hello { node }) => {
                self.them = Some(node);
                State::Negotiating { pending: None, chunk_size: DEFAULT_CHUNK_SIZE }
            }

            (State::Negotiating { chunk_size, .. }, Message::Propose { proposal }) => {
                State::Negotiating { pending: Some(proposal), chunk_size }
            }

            (State::Negotiating { pending: Some(Proposal::Transfer { dir }), chunk_size }, Message::Accept) => {
                State::Transferring { dir, chunk_size }
            }

            (State::Negotiating { pending: Some(Proposal::ChunkSize(n)), .. }, Message::Accept) => {
                State::Negotiating { pending: None, chunk_size: n }
            }

            (State::Negotiating { .. }, Message::Reject | Message::Fail | Message::Bye) => State::Close,

            (State::Transferring { .. }, Message::Done | Message::Fail) => State::Close,

            (State::Close, Message::Bye) => {
                self.them = None;
                State::Idle
            }

            (state, msg) => {
                self.state = state;
                return Err(ProtocolError::Unexpected(msg));
            }
        };

        Ok(())
    }
}


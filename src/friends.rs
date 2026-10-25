// friends.rs
// Here we have the sate machine for each friend.
// The negotiation between friends and client
// should be a state machine

// P2P
// - Both sides send `Hello { node }` on connect. Nobody is client/server.
// - Either side may propose from `Idle`.

// This can be used as below:
// let friend = Friend::new(1, 2, Message::Hello { node: 2 }) // Idle by default
// let is_negotiating = Friend::<Negotiating>::from(friend);
// let is_transferring = Friend::<Transferring>::from(is_negotiating);
// let is_closed = Friend::<Close>::from(is_transferring);
//
// or
//
// let friend = Friend::new(1, 2, Message::Hello { node: 2 }) // Idle by default
// let is_negotiating = Friend::<Negotiating>::from(friend);
// let is_closed = Friend::<Close>::from(is_negotiating);

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
        use Message as M;
        use State as S;

        let current = std::mem::replace(&mut self.state, S::Close);

        self.state = match (current, msg) {
            (S::Idle, M::Hello { node }) => {
                self.them = Some(node);
                S::Negotiating { pending: None, chunk_size: DEFAULT_CHUNK_SIZE }
            }

            (S::Negotiating { chunk_size, .. }, M::Propose { proposal }) => {
                S::Negotiating { pending: Some(proposal), chunk_size }
            }

            (S::Negotiating { pending: Some(Proposal::Transfer { dir }), chunk_size }, M::Accept) => {
                S::Transferring { dir, chunk_size }
            }

            (S::Negotiating { pending: Some(Proposal::ChunkSize(n)), .. }, M::Accept) => {
                S::Negotiating { pending: None, chunk_size: n }
            }

            (S::Negotiating { .. }, M::Reject | M::Fail | M::Bye) => S::Close,

            (S::Transferring { .. }, M::Done | M::Fail) => S::Close,

            (S::Close, M::Bye) => {
                self.them = None;
                S::Idle
            }

            (state, msg) => {
                self.state = state;
                return Err(ProtocolError::Unexpected(msg));
            }
        };

        Ok(())
    }
}


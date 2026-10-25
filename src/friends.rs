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

pub type NodeID = [u8; 32];

enum Direction {
    Inbound,
    Outbound,
}

enum Proposal {
    Transfer { dir: Direction },
    ChunkSize(u32),
}

enum Message {
    Hello { node: NodeID },
    Propose { propsal: Proposal },
    Accept,
    Reject,
    Done,
    Fail,
    Bye,
}

pub struct Friend<S> {
    me: NodeID,
    them: Option<NodeID>,
    state: S,
}

// 'From' Trait implemented to enforce state change direction at compile time, currently just preserves the entire message between state changes, making no changes.
// Currently enforced as follows:
//
// Idle -> Negotiating -> Close -> Idle
//              or
// Idle -> Negotiating -> Transferring -> Close -> Idle
//
// ToDo: Implement message changes between states.

impl Friend<Idle> {
    fn new(me: NodeID, them: Option<NodeID>, message: Message) -> Self {
        Friend {
            me,
            them,
            state: Idle::new(message),
        }
    }
}

struct Idle {
    message: Message,
}

impl Idle {
    pub fn new(start_message: Message) -> Self {
        Idle {
            message: start_message,
        }
    }
}

struct Negotiating {
    message: Message,
}

impl From<Friend<Idle>> for Friend<Negotiating> {
    fn from(val: Friend<Idle>) -> Friend<Negotiating> {
        Friend {
            me: val.me,
            them: val.them,
            state: Negotiating {
                message: val.state.message,
            },
        }
    }
}

impl From<Friend<Negotiating>> for Friend<Close> {
    fn from(val: Friend<Negotiating>) -> Friend<Close> {
        Friend {
            me: val.me,
            them: val.them,
            state: Close {
                message: val.state.message,
            },
        }
    }
}

struct Transferring {
    message: Message,
}

impl From<Friend<Negotiating>> for Friend<Transferring> {
    fn from(val: Friend<Negotiating>) -> Friend<Transferring> {
        Friend {
            me: val.me,
            them: val.them,
            state: Transferring {
                message: val.state.message,
            },
        }
    }
}

struct Close {
    message: Message,
}

impl From<Friend<Transferring>> for Friend<Close> {
    fn from(val: Friend<Transferring>) -> Friend<Close> {
        Friend {
            me: val.me,
            them: val.them,
            state: Close {
                message: val.state.message,
            },
        }
    }
}

impl From<Friend<Close>> for Friend<Idle> {
    fn from(val: Friend<Close>) -> Friend<Idle> {
        Friend {
            me: val.me,
            them: val.them,
            state: Idle {
                message: val.state.message,
            },
        }
    }
}

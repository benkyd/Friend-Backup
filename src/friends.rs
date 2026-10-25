// friends.rs
// Here we have the sate machine for each friend.
// The negotiation between friends and client
// should be a state machine

// P2P
// - Both sides send `Hello { node }` on connect. Nobody is client/server.
// - Either side may propose from `Idle`.

use serde::{Serialize, Deserialize};

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
    Propose,
    Accept,
    Reject,
    Done,
    Fail,
    Bye,
}

enum State {
    Idle,
    Negotiating,
    Transfering,
    Close,
}

pub struct Friend {
    me: NodeID,
    them: Option<NodeID>,
    state: State,
}

impl Friend {
    pub fn new() {

    }

    pub fn step() {

    }
}


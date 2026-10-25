// friends.rs
// Here we have the sate machine for each friend.
// The negotiation between friends and client
// should be a state machine

// P2P
// - Both sides send `Hello { node }` on connect. Nobody is client/server.
// - Either side may propose from `Idle`.

pub type NodeID = [u8; 32];

pub enum State {
    Idle,
    Negotiating,
    Propose,
}

pub struct Friend {
    me: NodeID,
    them: Option<NodeID>,
}

impl Friend {
    pub fn new() {

    }

    pub fn step() {

    }
}


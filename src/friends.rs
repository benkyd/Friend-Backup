// friends.rs
// P2P
// - Both sides send `Hello { node }` on connect. Nobody is client/server.
// - Either side may propose from `Idle`.

use blake3;
use serde::{Deserialize, Serialize};

const DEFAULT_CHUNK_SIZE: u32 = 64 * 1024;

pub type NodeID = [u8; 32];

// upgradable and extendable
#[derive(Debug, Serialize, Deserialize)]
pub enum Features {
    DumbBlob, // replace entire blob every time
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Header {}
#[derive(Debug, Serialize, Deserialize)]
pub struct BlobInfo {
    seq: u64
}

#[derive(Debug)]
pub enum Proposal {
    Transfer,
    ChunkSize(u32),
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Message {
    // Handshake and dial
    Hello { node: NodeID, features: Vec<Features> },
    Welcome { node: NodeID, features: Vec<Features> },

    // Hi, i would like to backup X GB, do we still have quota for this?
    // what SORT of backup shall i send you and HOW shall you store it
    Status,
    StatusReport { grant: u64, used: u64, blobs: Vec<BlobInfo>},

    Delete { seq: u64 },
    Deleted { seq: u64, freed: u64 },

    // File transport (over QUINN / QUIC)
    Offer { header: Header },
    Accept,
    Reject { reason: u64 },
    Done,
    Stored { blake3: [u8; 32]},

    // Restore backup
    Fetch { seq: u64 },

    // Data
    Chunk { size: u64 },

    // Close
    Bye,
}

#[derive(Debug)]
pub enum ProtocolError {
    Unexpected(Message),
}

pub struct Friend {
    me: NodeID,
    them: Option<NodeID>,
}

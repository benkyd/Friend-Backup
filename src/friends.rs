// friends.rs

use anyhow::Result;
use serde::{Deserialize, Serialize};
use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};

const MAX_FRAME: u32 = 64 * 1024;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Features {
    DumbBlob, // replace entire blob every time
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Header {
    pub version: u16,
    pub seq: u64,
    pub size: u64,
    pub blake3: [u8; 32],
    pub created: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlobInfo {
    pub seq: u64,
}

// postcard encodes variant *index*: append new variants at the end only.
#[derive(Debug, Serialize, Deserialize)]
pub enum Message {
    // Handshake
    Hello { features: Vec<Features> },
    Welcome { features: Vec<Features> },

    // Query
    Status,
    StatusReport { grant: u64, used: u64, blobs: Vec<BlobInfo> },

    Delete { seq: u64 },
    Deleted { seq: u64, freed: u64 },

    // Transfer (data goes on a separate uni stream)
    Offer { header: Header },
    Accept,
    Reject { reason: u64 },
    Done,
    Stored { blake3: [u8; 32] },

    // Restore
    Fetch { seq: u64 },

    // Close
    Bye,
}

#[derive(Debug, thiserror::Error)]
pub enum ProtocolError {
    #[error("unexpected message: {0:?}")]
    Unexpected(Message),
    #[error("frame too large: {0} bytes (max {MAX_FRAME})")]
    FrameTooLarge(u32),
}

pub async fn send_msg<W: AsyncWrite + Unpin>(w: &mut W, msg: &Message) -> Result<()> {
    let buf = postcard::to_allocvec(msg)?;
    w.write_u32(buf.len() as u32).await?;
    w.write_all(&buf).await?;
    Ok(())
}

pub async fn recv_msg<R: AsyncRead + Unpin>(r: &mut R) -> Result<Message> {
    let len = r.read_u32().await?;
    if len > MAX_FRAME {
        return Err(ProtocolError::FrameTooLarge(len).into());
    }
    let mut buf = vec![0u8; len as usize];
    r.read_exact(&mut buf).await?;
    Ok(postcard::from_bytes(&buf)?)
}

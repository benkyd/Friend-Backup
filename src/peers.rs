// peers.rs
// author: Ben Kyd
// created: 2026
//
// Here we manage connections (incoming and outbound)
// using Iroh, we also define an Iroh protocol for message passing
// and lay out infrastructure for fast BLOB transport over multiple
// QUIC UDP streams

use crate::configuration::{self, DebugRole};
use crate::friends::{Features, Message, ProtocolError, recv_msg, send_msg};

use anyhow::{Context, Result, bail};
use iroh::{
    Endpoint, EndpointId, SecretKey,
    endpoint::{Connection, presets},
    protocol::{AcceptError, ProtocolHandler, Router},
};

const ALPN: &[u8] = b"friend/transport/v0";

#[derive(Debug, thiserror::Error)]
pub enum SessionError {
    #[error("peer unreachable")]
    Unreachable(#[source] iroh::endpoint::ConnectError),

    #[error("peer rejected offer: {reason:?}")]
    Rejected { reason: String, free: u64 },

    #[error("stored hash doesn't match")]
    Integrity,

    #[error("protocol error during {stage}")]
    Protocol { stage: &'static str, #[source] source: anyhow::Error },
}

#[derive(Debug, Clone)]
struct Receiver;

impl ProtocolHandler for Receiver {
    async fn accept(&self, conn: Connection) -> Result<(), AcceptError> {
        match handle_inbound(&conn).await {
            Ok(()) => {
                conn.closed().await; // dialer reads last, dialer closes
                Ok(())
            }
            Err(e) => {
                eprintln!("inbound session failed: {e:#}");
                conn.close(1u32.into(), b"protocol error");
                Err(std::io::Error::other(e).into())
            }
        }
    }
}

async fn handle_inbound(conn: &Connection) -> Result<()> {
    let (mut send, mut recv) = conn.accept_bi().await.context("accepting control stream")?;

    match recv_msg(&mut recv).await.context("reading Hello")? {
        Message::Hello { features } => {
            println!("< Hello {features:?}");
            send_msg(
                &mut send,
                &Message::Welcome {
                    features: vec![Features::DumbBlob],
                },
            )
            .await
            .context("sending Welcome")?;
        }
        other => return Err(ProtocolError::Unexpected(other).into()),
    }

    send.finish().context("finishing control stream")?;
    Ok(())
}

async fn dial(endpoint: &Endpoint, peer: EndpointId) -> Result<()> {
    let conn = endpoint
        .connect(peer, ALPN)
        .await
        .with_context(|| format!("connecting to {peer}"))?;
    let (mut send, mut recv) = conn.open_bi().await.context("opening control stream")?;

    send_msg(
        &mut send,
        &Message::Hello {
            features: vec![Features::DumbBlob],
        },
    )
    .await
    .context("sending Hello")?;

    match recv_msg(&mut recv).await.context("reading Welcome")? {
        Message::Welcome { features } => println!("> Welcome {features:?}"),
        other => return Err(ProtocolError::Unexpected(other).into()),
    }

    send.finish().context("finishing control stream")?;
    conn.close(0u32.into(), b"bye");
    Ok(())
}

async fn bind() -> Result<Endpoint> {
    let endpoint = Endpoint::builder(presets::N0)
        .secret_key(SecretKey::generate()) // TODO: load from file
        .bind()
        .await
        .context("binding endpoint")?;
    endpoint.online().await;
    println!("> our endpoint id: {}", endpoint.id());
    Ok(endpoint)
}

pub async fn listen(
    _settings: &configuration::Settings,
    settings_override: &configuration::SettingsOverride,
) -> Result<()> {
    let endpoint = bind().await?;

    match settings_override.role {
        Some(DebugRole::TestReceiver) => {
            let router = Router::builder(endpoint.clone())
                .accept(ALPN, Receiver)
                .spawn();
            tokio::signal::ctrl_c()
                .await
                .context("waiting for ctrl-c")?;
            router.shutdown().await.context("shutting down router")?;
        }

        Some(DebugRole::TestSender) => {
            let peer: EndpointId = settings_override
                .ticket
                .as_deref()
                .context("--ticket is required for test-sender")?
                .parse()
                .context("--ticket is not a valid endpoint id")?;

            let result = dial(&endpoint, peer).await;
            endpoint.close().await; // close cleanly on success *and* failure
            result?;
        }

        _ => {
            bail!("No role");
        }
    }

    Ok(())
}

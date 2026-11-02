// peers.rs
// author: Ben Kyd
// created: 2026
//
// Here we manage connections (incoming and outbound)
// using Iroh, we also define an Iroh protocol for message passing
// and lay out infrastructure for fast BLOB transport over multiple
// QUIC UDP streams

use crate::configuration;
use crate::friends;

use std::error::Error;

use iroh::{Endpoint, SecretKey, endpoint::presets, protocol::Router};

const ALPN: &[u8] = b"friend/transport/v0";

pub async fn recv() {}

pub async fn listen(
    settings: &configuration::Settings,
    settings_override: &configuration::SettingsOverride,
) -> Result<(), Box<dyn Error>> {

    match settings_override.role {
        configuration::DebugRole::TestReceiver => {}
        configuration::DebugRole::TestSender => {}
    }

    let secret = SecretKey::generate(); // First time launch?

    let endpoint = Endpoint::builder(presets::N0)
        .secret_key(secret)
        // Bind the endpoint to the socket.
        .bind()
        .await?;

    println!("> our endpoint id: {}", endpoint.id());

    // let router = Router::builder(endpoint).accept(ALPN, recv).spawn();

    Ok(())
}

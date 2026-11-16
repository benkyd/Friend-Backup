mod configuration;
mod friends;
mod peers;
mod grants;

use anyhow::{Context, Result};
use clap::Parser;
use config::Config;

#[tokio::main]
async fn main() -> Result<()> {
    let settings: configuration::Settings = Config::builder()
        .add_source(config::File::with_name("./settings.toml"))
        .add_source(config::Environment::with_prefix("FRIEND"))
        .build()
        .context("loading settings.toml / FRIEND_* env")?
        .try_deserialize()
        .context("settings.toml doesn't match Settings")?;

    let settings_override = configuration::SettingsOverride::parse();

    println!("Friend Backup Starting Up...");

    if settings_override.ticket.is_some() {
        println!("Running with overridden settings: {settings_override:?}");
    }

   let _ = grants::blobs_by_peer( "123", &settings.storage.path);

    // Here we should establish connections to configured peers (and setup retries)
    // Then we should listen for incoming connections ourself, for the client state machine
    peers::listen(&settings, &settings_override).await
}

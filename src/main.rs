mod configuration;
mod friends;
mod peers;

use config::Config;
use tokio;

#[tokio::main]
async fn main() {
    let settings = Config::builder()
        .add_source(config::Environment::with_prefix("FRIEND"))
        .add_source(config::File::with_name("./settings.toml"))
        .build()
        .unwrap()
        .try_deserialize::<configuration::Settings>()
        .unwrap(); // This is Option<Settings>? i am confused by this

    println!("Friend Backup Starting Up...");
    println!("With Configuration: {:?}", settings);

    // Here we should establish connections to configured peers (and setup retries)
    // Then we should listen for incoming connections ourself, for the client state machine
    let _ = peers::listen(settings).await;
}

mod configuration;
mod friends;
mod peers;

use config::Config;
use std::env;
use std::error::Error;
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let settings = Config::builder()
        .add_source(config::Environment::with_prefix("FRIEND"))
        .add_source(config::File::with_name("./settings.toml"))
        .build()
        .unwrap()
        .try_deserialize::<configuration::Settings>()
        .unwrap(); // This is Option<Settings>? i am confused by this

    let mut args = std::env::args().skip(1);
    let role_option = args.next().unwrap();

    let mut settings_override = configuration::SettingsOverride {
        used: false,
        role: -1,
        ticket: Option::None,
    };

    match role_option.as_str() {
        "receiver" => {
            settings_override.used = true;
            settings_override.role = configuration::DebugRole::TestReceiver;
            settings_override.ticket = Option::None
        }
        "sender" => {
            settings_override.used = true;
            settings_override.role = configuration::DebugRole::TestSender;
            settings_override.ticket = Some(args.next().unwrap());
        }
        _ => (),
    }

    println!("Friend Backup Starting Up...");
    println!("With Configuration: {:?}", settings);

    if settings_override.used {
        println!("Running with overwritten settings: {:?}", settings_override);
    }
    // Here we should establish connections to configured peers (and setup retries)
    // Then we should listen for incoming connections ourself, for the client state machine
    let _ = peers::listen(&settings, &settings_override).await;
    Ok(())
}

use config::Config;
use serde::Deserialize;
use std::collections::HashMap;
use std::net::IpAddr;

#[derive(Deserialize, Debug)]
struct Settings {
    listen: u32,
    encryption: String,
    friends: HashMap<String, Friend>,
}

#[derive(Deserialize, Debug)]
struct Friend {
    ip: IpAddr,
}

fn main() {
    let settings = Config::builder()
        .add_source(config::Environment::with_prefix("FRIEND"))
        .add_source(config::File::with_name("./settings.toml"))
        .build()
        .unwrap()
        .try_deserialize::<Settings>();

    println!("Friend Backup Starting Up...");
    println!("With Configuration: {:?}", settings);

}

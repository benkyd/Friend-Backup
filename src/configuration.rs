use serde::Deserialize;
use std::collections::HashMap;
use std::net::IpAddr;

#[derive(Deserialize, Debug)]
pub struct Settings {
    listen: u32,
    encryption: String,
    friends: HashMap<String, Friend>,
}

#[derive(Deserialize, Debug)]
pub struct Friend {
    ip: IpAddr,
}

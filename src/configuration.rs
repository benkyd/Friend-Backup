use serde::Deserialize;
use std::collections::HashMap;
use std::net::IpAddr;

#[derive(Deserialize, Debug)]
pub struct Settings {
    pub listen: u32,
    pub encryption: String,
    pub friends: HashMap<String, Friend>,
}

#[derive(Deserialize, Debug)]
pub struct Friend {
    pub ip: IpAddr,
}

use serde::Deserialize;
use std::collections::HashMap;
use std::net::IpAddr;

#[derive(Debug)]
pub enum DebugRole {
    TestReceiver,
    TestSender
}

#[derive(Debug)]
pub struct SettingsOverride {
    pub used: bool,
    pub role: DebugRole,
    pub ticket: Option<String>
}

#[derive(Deserialize, Debug)]
pub struct Settings {
    pub listen: u32,
    pub encryption: String,
    pub schedule: Schedule,
    pub backup: Backup,
    pub friends: HashMap<String, Friend>,
    pub storage: Storage,
}

#[derive(Deserialize, Debug)]
pub struct Schedule {
    pub cron: String,
}

#[derive(Deserialize, Debug)]
pub struct Backup {
    pub sources: Vec<String>,
    pub exclude: Vec<String>,
    pub encrypt_to: String,
    pub zstd_level: i32,
}

#[derive(Deserialize, Debug)]
pub struct Friend {
    pub id: String,
}

#[derive(Deserialize, Debug)]
pub struct Storage {
    pub path: String,
    pub max: String,
    pub grants: HashMap<String, String>,
}


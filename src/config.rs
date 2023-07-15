use std::{fs::File, io::Read};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub ip: String,
    pub port: String,
    pub password: String,
    pub server_log: bool,
    pub jobs: Vec<CronJob>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CronJob {
    pub name: String,
    pub commands: Vec<String>,
    pub cron: String,
}

impl Config {
    pub fn from_file(path: String) -> Self {
        let mut config_file = File::open(path).expect("Failed to open config file.");
        let mut config = String::default();
        config_file.read_to_string(&mut config).unwrap();
        serde_yaml::from_str::<Config>(&config).unwrap()
    }
}

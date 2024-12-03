use serde::Deserialize;
use std::fs;

#[derive(Deserialize)]
pub struct ServerConfig {
    pub server: ServerSettings,
    pub camera_config: CameraConfig,
}

#[derive(Deserialize)]
pub struct ServerSettings {
    pub host: String,
    pub port: u16,
    pub log_level: String,
}

#[derive(Deserialize)]
pub struct CameraConfig {
    pub config_file: String,
}

pub fn load_server_config(filename: &str) -> ServerConfig {
    let config_str = fs::read_to_string(filename).expect("Unable to read config file");
    let config: ServerConfig = serde_yaml::from_str(&config_str).expect("Failed to parse server config");
    config
}


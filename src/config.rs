use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Deserialize, Serialize, Clone)]
pub struct CameraDetails {
    pub name: String,
    pub url: String,  // URL для RTSP или индекс для встроенной камеры
}

#[derive(Deserialize)]
pub struct CameraConfig {
    pub cameras: Vec<CameraDetails>,
}

pub fn load_config(file_path: &str) -> CameraConfig {
    let content = fs::read_to_string(file_path).expect("Не удалось прочитать файл конфигурации");
    serde_yaml::from_str(&content).expect("Ошибка парсинга YAML")
}

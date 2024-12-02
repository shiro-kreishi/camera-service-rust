use opencv::{prelude::*, videoio, core, imgcodecs};
use opencv::videoio::{CAP_ANY, CAP_FFMPEG};
use opencv::core::Vector;
use serde::{Serialize, Deserialize};
use std::fs;

#[derive(Serialize, Deserialize)]
pub struct CameraDetails {
    pub name: String,
    pub url: String,  // URL для RTSP или индекс для встроенной камеры
}

#[derive(Serialize, Deserialize)]
pub struct CameraConfig {
    pub cameras: Vec<CameraDetails>,
}

pub fn load_config(file_path: &str) -> CameraConfig {
    let content = fs::read_to_string(file_path).expect("Не удалось прочитать файл конфигурации");
    serde_yaml::from_str(&content).expect("Ошибка парсинга YAML")
}

pub fn get_camera_image(camera_index: usize, cameras: &Vec<CameraDetails>) -> Option<Vec<u8>> {
    let camera = match cameras.get(camera_index) {
        Some(c) => c,
        None => return None,
    };

    let mut frame = core::Mat::default();
    let mut capture = if camera.url.starts_with("rtsp://") {
        videoio::VideoCapture::from_file(&camera.url, CAP_FFMPEG).unwrap()
    } else {
        let index: i32 = camera.url.parse().unwrap_or(0);
        videoio::VideoCapture::new(index, CAP_ANY).unwrap()
    };

    if !capture.is_opened().unwrap() {
        return None;
    }

    capture.read(&mut frame).unwrap();

    if frame.empty() {
        return None;
    }

    let mut buffer = Vector::<u8>::new(); // Используем Vector<u8>, а не Vec<u8>
    imgcodecs::imencode(".jpg", &frame, &mut buffer, &Vector::<i32>::new()).unwrap();
    Some(buffer.to_vec()) // Преобразуем обратно в Vec<u8>, чтобы вернуть тип Option<Vec<u8>>
}

pub fn get_camera_count(cameras: &Vec<CameraDetails>) -> usize {
    cameras.len()
}

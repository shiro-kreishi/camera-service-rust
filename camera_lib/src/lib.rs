extern crate opencv;
extern crate serde;
extern crate serde_yaml;

use opencv::prelude::*;
use opencv::videoio::{self, VideoCapture, CAP_ANY, CAP_FFMPEG};
use opencv::imgcodecs;
use opencv::core;
use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize)]
pub struct CameraConfig {
    pub cameras: Vec<CameraDetails>,
}

#[derive(Debug, Deserialize)]
pub struct CameraDetails {
    pub name: String,
    pub url: String,  // URL для RTSP или индекс для встроенной камеры
}

pub struct Camera {
    capture: VideoCapture,
}

impl Camera {
    /// Создание новой камеры по URL или индексу
    pub fn new(url: &str) -> opencv::Result<Self> {
        let capture = if url.starts_with("rtsp://") {
            VideoCapture::from_file(url, CAP_FFMPEG)?
        } else {
            let index: i32 = url.parse().unwrap_or(0);
            VideoCapture::new(index, CAP_ANY)?
        };

        if !capture.is_opened()? {
            Err(opencv::Error::new(
                opencv::core::StsError,
                format!("Не удалось открыть камеру по адресу: {}", url),
            ))
        } else {
            Ok(Self { capture })
        }
    }

    /// Захват кадра и сохранение в файл
    pub fn capture_frame(&mut self, output_path: &str) -> opencv::Result<()> {
        let mut frame = core::Mat::default();
        self.capture.read(&mut frame)?;

        if frame.empty() {
            return Err(opencv::Error::new(
                opencv::core::StsError,
                "Не удалось получить кадр.",
            ));
        }

        imgcodecs::imwrite(output_path, &frame, &opencv::types::VectorOfi32::new())?;
        Ok(())
    }

    /// Завершение работы с камерой
    pub fn release(&mut self) {
        self.capture.release().expect("Не удалось закрыть камеру");
    }
}

/// Функция для загрузки конфигурации из YAML
pub fn load_config(file_path: &str) -> CameraConfig {
    let content = fs::read_to_string(file_path).expect("Не удалось прочитать файл конфигурации");
    serde_yaml::from_str(&content).expect("Ошибка парсинга YAML")
}

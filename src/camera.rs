use opencv::{prelude::*, videoio::{CAP_FFMPEG, VideoCapture}, core::{Mat, Vector}};
use std::{sync::{Arc, Mutex}, thread, time::Duration};
use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct CameraDetails {
    pub name: String,
    pub url: String,  // URL для RTSP или индекс для встроенной камеры
}

pub struct Camera {
    last_frame: Arc<Mutex<Option<Mat>>>,
    is_running: Arc<Mutex<bool>>,
}

impl Camera {
    pub fn new(rtsp_link: &str) -> Self {
        let mut capture = VideoCapture::from_file(rtsp_link, CAP_FFMPEG).unwrap();
        let last_frame = Arc::new(Mutex::new(None));
        let is_running = Arc::new(Mutex::new(true));

        let last_frame_clone = Arc::clone(&last_frame);
        let is_running_clone = Arc::clone(&is_running);

        thread::spawn(move || {
            while *is_running_clone.lock().unwrap() {
                let mut frame = Mat::default();
                if capture.grab().unwrap() && capture.retrieve(&mut frame, 0).unwrap() {
                    let mut frame_guard = last_frame_clone.lock().unwrap();
                    *frame_guard = Some(frame.clone());
                } else {
                    thread::sleep(Duration::from_millis(100));
                }
            }
        });

        Camera { last_frame, is_running }
    }

    pub fn get_frame(&self) -> Option<Mat> {
        let frame_guard = self.last_frame.lock().unwrap();
        frame_guard.clone()
    }

    pub fn release(&self) {
        let mut is_running = self.is_running.lock().unwrap();
        *is_running = false;
    }
}

pub fn get_camera_image(camera_index: usize, cameras: &Vec<Camera>) -> Option<Vec<u8>> {
    let camera = cameras.get(camera_index)?;
    let frame = camera.get_frame()?;

    let mut buffer = Vector::<u8>::new();  // Используем Vector<u8>, а не Vec<u8>
    opencv::imgcodecs::imencode(".jpg", &frame, &mut buffer, &opencv::core::Vector::<i32>::new()).unwrap();
    Some(buffer.to_vec())  // Преобразуем обратно в Vec<u8>, чтобы вернуть тип Option<Vec<u8>>
}

pub fn get_camera_count(cameras: &Vec<Camera>) -> usize {
    cameras.len()
}

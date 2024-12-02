extern crate camera_lib;
use camera_lib::{Camera, load_config};

fn main() {
    let config = load_config("config.yml");

    for camera_details in config.cameras {
        println!("Подключение к камере: {}", camera_details.name);

        let mut camera = Camera::new(&camera_details.url)
            .expect("Не удалось подключиться к камере");

        let output_file = format!("{}_snapshot.jpg", camera_details.name);
        camera.capture_frame(&output_file)
            .expect("Не удалось сделать снимок");

        println!("Снимок сохранён в файл: {}", output_file);

        camera.release();
    }
}

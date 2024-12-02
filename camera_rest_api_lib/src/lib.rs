mod camera_lib;

use actix_web::{web, HttpServer, App, HttpResponse, Responder};
use camera_lib::{load_config, get_camera_image, get_camera_count, CameraDetails};
use std::sync::{Arc, Mutex};
use log::{info, debug, error, warn};

#[derive(Clone)]
pub struct AppState {
    cameras: Arc<Mutex<Vec<CameraDetails>>>,
}

pub async fn get_image(camera_index: web::Path<usize>, state: web::Data<AppState>) -> impl Responder {
    let cameras = state.cameras.lock().unwrap();
    debug!("Запрашиваем изображение с камеры по индексу {}", camera_index);

    match get_camera_image(*camera_index, &cameras) {
        Some(image_data) => {
            debug!("Изображение получено с камеры {}", camera_index);
            HttpResponse::Ok()
                .content_type("image/jpeg")
                .body(image_data)
        }
        None => {
            error!("Не удалось получить изображение с камеры {}", camera_index);
            HttpResponse::NotFound().body("Камера не найдена или не удалось захватить изображение")
        }
    }
}

pub async fn get_cameras_count(state: web::Data<AppState>) -> impl Responder {
    let cameras = state.cameras.lock().unwrap();
    debug!("Запрашиваем количество камер");

    // Получаем количество камер синхронно
    let count = get_camera_count(&cameras);

    HttpResponse::Ok().json(count)  // Отправляем количество камер как JSON
}

pub async fn get_cameras(state: web::Data<AppState>) -> impl Responder {
    let cameras = state.cameras.lock().unwrap();
    debug!("Запрашиваем список камер");

    HttpResponse::Ok().json(&*cameras)  // Отправляем список камер как JSON
}

pub async fn run_server(config_path: &str) -> std::io::Result<()> {
    setup_logger();

    info!("Инициализация сервера с конфигурацией из файла: {}", config_path);

    let config = load_config(config_path);
    let app_state = web::Data::new(AppState {
        cameras: Arc::new(Mutex::new(config.cameras)),
    });

    info!("Сервер запускается на порту 8080");

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .route("/image/{camera_index}", web::get().to(get_image))
            .route("/camera_count", web::get().to(get_cameras_count))
            .route("/cameras", web::get().to(get_cameras))
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}

fn setup_logger() {
    use std::env;
    use env_logger::Builder;

    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "debug");
    }

    Builder::from_env(env_logger::Env::default().default_filter_or("debug"))
        .init();
}

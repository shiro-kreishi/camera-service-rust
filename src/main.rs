mod camera;
mod config;

use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use std::sync::{Arc, Mutex};
use log::{debug, info, error};
use crate::camera::{Camera, get_camera_image};
use crate::config::{CameraDetails, load_config};

#[derive(Clone)]
pub struct AppState {
    cameras: Arc<Mutex<Vec<Camera>>>,              // Для камер с кадрами
    cameras_detailed: Arc<Mutex<Vec<CameraDetails>>>, // Для подробной информации о камерах
}

async fn get_image(state: web::Data<AppState>, index: web::Path<usize>) -> impl Responder {
    let cameras = state.cameras.lock().unwrap();
    if let Some(image) = get_camera_image(*index, &cameras) {
        info!("Получен запрос на изображение для камеры с индексом {}", index);
        HttpResponse::Ok()
            .content_type("image/jpeg")
            .body(image)
    } else {
        error!("Не удалось получить изображение для камеры с индексом {}", index);
        HttpResponse::NotFound().body("Камера не найдена или ошибка получения кадра")
    }
}

async fn get_cameras_count(state: web::Data<AppState>) -> impl Responder {
    let cameras = state.cameras.lock().unwrap();
    let count = cameras.len();
    debug!("Запрашивается количество камер: {}", count);
    HttpResponse::Ok().json(count)
}

async fn get_cameras(state: web::Data<AppState>) -> impl Responder {
    // Извлекаем данные из MutexGuard
    let cameras_detailed = state.cameras_detailed.lock().unwrap();
    info!("Запрашивается список всех камер.");
    HttpResponse::Ok().json(&*cameras_detailed)  // Сериализуем данные, извлекая их из MutexGuard
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "debug");
    }
    env_logger::init();

    // Загружаем конфигурацию и создаем список камер
    let config = load_config("config.yml");

    // Создаем список камер с кадрами
    let cameras: Vec<Camera> = config.cameras.iter()
        .map(|details| Camera::new(&details.url))
        .collect();

    // Создаем список подробной информации о камерах
    let cameras_detailed: Vec<CameraDetails> = config.cameras.clone();

    // Состояние приложения
    let state = web::Data::new(AppState {
        cameras: Arc::new(Mutex::new(cameras)),
        cameras_detailed: Arc::new(Mutex::new(cameras_detailed)),
    });

    debug!("Сервер запущен на http://127.0.0.1:8080");

    // Запуск HTTP-сервера
    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .route("/image/{index}", web::get().to(get_image))
            .route("/camera-count", web::get().to(get_cameras_count))
            .route("/cameras", web::get().to(get_cameras))  // Новый маршрут для списка камер
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}

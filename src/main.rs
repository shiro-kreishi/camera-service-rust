mod camera;
mod config;
mod server_config;

use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use std::sync::{Arc, Mutex};
use log::{debug, info, error};
use crate::camera::{Camera, get_camera_image};
use crate::config::{CameraDetails, load_config};
use crate::server_config::load_server_config;

#[derive(Clone)]
pub struct AppState {
    cameras: Arc<Mutex<Vec<Camera>>>,              // Для камер с кадрами
    cameras_detailed: Arc<Mutex<Vec<CameraDetails>>>, // Для подробной информации о камерах
    config_path: String, // Путь до конфигурационного файла
}

async fn get_image(state: web::Data<AppState>, index: web::Path<usize>) -> impl Responder {
    debug!("Запрос на получение изображения для камеры с индексом {}", index);

    let cameras = state.cameras.lock().unwrap();
    if let Some(image) = get_camera_image(*index, &cameras) {
        debug!("Изображение для камеры с индексом {} получено успешно", index);
        HttpResponse::Ok()
            .content_type("image/jpeg")
            .body(image)
    } else {
        error!("Ошибка получения изображения для камеры с индексом {}", index);
        HttpResponse::NotFound().body("Камера не найдена или ошибка получения кадра")
    }
}

async fn get_cameras_count(state: web::Data<AppState>) -> impl Responder {
    debug!("Запрос на получение количества камер");

    let cameras = state.cameras.lock().unwrap();
    let count = cameras.len();
    info!("Количество камер: {}", count);

    HttpResponse::Ok().json(count)
}

async fn get_cameras(state: web::Data<AppState>) -> impl Responder {
    debug!("Запрос на получение списка всех камер");

    let cameras_detailed = state.cameras_detailed.lock().unwrap();
    info!("Возвращаем список камер: {}", cameras_detailed.len());

    HttpResponse::Ok().json(&*cameras_detailed)  // Сериализуем данные, извлекая их из MutexGuard
}

async fn refresh_cameras(state: web::Data<AppState>) -> impl Responder {
    debug!("Запрос на обновление списка камер");

    // Перезагружаем конфигурацию из пути, указанного в AppState
    let config = load_config(&state.config_path);

    // Создаем новый список камер с кадрами
    let cameras: Vec<Camera> = config.cameras.iter()
        .map(|details| Camera::new(&details.url))
        .collect();

    // Создаем новый список подробной информации о камерах
    let cameras_detailed: Vec<CameraDetails> = config.cameras.clone();

    // Обновляем состояние приложения
    {
        let mut cameras_guard = state.cameras.lock().unwrap();
        *cameras_guard = cameras;
    }
    {
        let mut cameras_detailed_guard = state.cameras_detailed.lock().unwrap();
        *cameras_detailed_guard = cameras_detailed;
    }

    info!("Список камер успешно обновлен");

    HttpResponse::Ok().body("The list of cameras has been updated")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "debug");
    }
    env_logger::init();

    // Загружаем конфигурацию сервера
    let server_config = load_server_config("config.yml");

    // Загружаем конфигурацию для камер
    let config = load_config(&server_config.camera_config.config_file);

    let cameras: Vec<Camera> = config.cameras.iter()
        .map(|details| Camera::new(&details.url))
        .collect();

    let cameras_detailed: Vec<CameraDetails> = config.cameras.clone();

    // Состояние приложения
    let state = web::Data::new(AppState {
        cameras: Arc::new(Mutex::new(cameras)),
        cameras_detailed: Arc::new(Mutex::new(cameras_detailed)),
        config_path: server_config.camera_config.config_file.clone(),
    });

    debug!("Server started at http://{}:{}", server_config.server.host, server_config.server.port);

    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .route("/image/{index}", web::get().to(get_image))
            .route("/camera_count", web::get().to(get_cameras_count))
            .route("/cameras", web::get().to(get_cameras))
            .route("/refresh", web::get().to(refresh_cameras))  // Новый маршрут для обновления списка камер
    })
        .bind(format!("{}:{}", server_config.server.host, server_config.server.port))?
        .run()
        .await
}

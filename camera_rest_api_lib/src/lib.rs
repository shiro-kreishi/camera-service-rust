mod camera_lib;

use actix_web::{web, HttpServer, App, HttpResponse, Responder};
use tokio::task;
use tokio::sync::Mutex;
use std::sync::Arc;
use log::{debug, error, info};
use camera_lib::{load_config, get_camera_image, get_camera_count, CameraDetails};

#[derive(Clone)]
pub struct AppState {
    cameras: Arc<Mutex<Vec<CameraDetails>>>,
}

pub async fn get_image(camera_index: web::Path<usize>, state: web::Data<AppState>) -> impl Responder {
    debug!("Получение изображения с камеры: {}", camera_index);
    let camera_index = *camera_index;
    let cameras = state.cameras.clone(); // Передаем в асинхронную задачу

    let result = task::spawn(async move {
        debug!("Начата обработка камеры: {}", camera_index);
        let cameras = cameras.lock().await;
        get_camera_image(camera_index, &cameras)
    })
        .await
        .unwrap(); // Ожидание завершения задачи

    match result {
        Some(image_data) => {
            debug!("Успешно получено изображение с камеры: {}", camera_index);
            HttpResponse::Ok().content_type("image/jpeg").body(image_data)
        }
        None => {
            error!("Не удалось получить изображение с камеры: {}", camera_index);
            HttpResponse::NotFound().body("Камера не найдена или не удалось захватить изображение")
        }
    }
}

pub async fn get_cameras_count(state: web::Data<AppState>) -> impl Responder {
    let cameras = state.cameras.lock().await;
    let count = get_camera_count(&cameras);
    debug!("Количество камер: {}", count);
    HttpResponse::Ok().json(count)
}

pub async fn run_server(config_path: &str) -> std::io::Result<()> {
    let config = load_config(config_path);
    let app_state = web::Data::new(AppState {
        cameras: Arc::new(Mutex::new(config.cameras)),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .route("/image/{camera_index}", web::get().to(get_image))
            .route("/camera_count", web::get().to(get_cameras_count))
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}

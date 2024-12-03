mod camera;
mod config;

use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use std::sync::{Arc, Mutex};
use log::{debug};
use crate::camera::{Camera, get_camera_image, get_camera_count};
use crate::config::load_config;

#[derive(Clone)]
pub struct AppState {
    cameras: Arc<Mutex<Vec<Camera>>>,
}

async fn get_image(state: web::Data<AppState>, index: web::Path<usize>) -> impl Responder {
    let cameras = state.cameras.lock().unwrap();
    if let Some(image) = get_camera_image(*index, &cameras) {
        HttpResponse::Ok()
            .content_type("image/jpeg")
            .body(image)
    } else {
        HttpResponse::NotFound().body("Камера не найдена или ошибка получения кадра")
    }
}

async fn get_cameras_count(state: web::Data<AppState>) -> impl Responder {
    let cameras = state.cameras.lock().unwrap();
    HttpResponse::Ok().json(cameras.len())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "debug");
    }
    env_logger::init();

    let config = load_config("config.yml");
    let cameras: Vec<Camera> = config.cameras.iter()
        .map(|details| Camera::new(&details.url))
        .collect();

    let state = web::Data::new(AppState {
        cameras: Arc::new(Mutex::new(cameras)),
    });

    debug!("Сервер запущен на http://127.0.0.1:8080");

    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .route("/image/{index}", web::get().to(get_image))
            .route("/camera_count", web::get().to(get_cameras_count))
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}

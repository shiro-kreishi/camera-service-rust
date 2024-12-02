mod camera_lib;

use actix_web::{web, HttpServer, App, HttpResponse, Responder};
use camera_lib::{load_config, get_camera_image, get_camera_count, CameraDetails};
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct AppState {
    cameras: Arc<Mutex<Vec<CameraDetails>>>,
}

pub async fn get_image(camera_index: web::Path<usize>, state: web::Data<AppState>) -> impl Responder {
    let cameras = state.cameras.lock().unwrap();
    match get_camera_image(*camera_index, &cameras) {
        Some(image_data) => HttpResponse::Ok()
            .content_type("image/jpeg")
            .body(image_data),
        None => HttpResponse::NotFound().body("Камера не найдена или не удалось захватить изображение"),
    }
}

pub async fn get_cameras_count(state: web::Data<AppState>) -> impl Responder {
    let cameras = state.cameras.lock().unwrap();
    HttpResponse::Ok().json(get_camera_count(&cameras))
}

pub async fn get_cameras(state: web::Data<AppState>) -> impl Responder {
    let cameras = state.cameras.lock().unwrap();
    HttpResponse::Ok().json(&*cameras)
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
            .route("/camera-count", web::get().to(get_cameras_count))
            .route("/cameras", web::get().to(get_cameras))
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}

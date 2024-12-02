use camera_rest_api_lib::{run_server};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    run_server("config.yml").await
}

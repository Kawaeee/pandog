mod converter;
use crate::converter::convert_file;

use env_logger;
use std::path::PathBuf;
use actix_web::{get, post, web, App, HttpServer, HttpResponse, Responder, middleware::Logger};


#[derive(serde::Deserialize)]
struct ConvertRequest {
    input_file_path: String,
    input_format: String,
    output_file_path: String,
    output_format: String,
}

#[derive(serde::Deserialize, serde::Serialize)]
struct ConvertResponse {
    success: bool,
    status_message: Option<String>,
    output_file_path: Option<String>,
}

#[post("/convert")]
async fn convert_route(request: web::Json<ConvertRequest>) -> impl Responder{

    let payload = request.into_inner();

    let input_file_path = PathBuf::from(&payload.input_file_path);
    let input_format = &payload.input_format;
    let output_file_path = PathBuf::from(&payload.output_file_path);
    let output_format = &payload.output_format;

    match convert_file(&input_file_path, input_format, &output_file_path, output_format) {
        Ok(result) => {
            let request_status = format!("Successfully converted file to {:?}", result);
            HttpResponse::Ok().json(ConvertResponse{ success: true, output_file_path: Some(result.to_str().unwrap().to_string()), status_message: Some(request_status) })
        },
        Err(error) => {
            let request_status = format!("Failed to convert file: {:?}", error);
            HttpResponse::BadRequest().json(ConvertResponse{ success: false, output_file_path: None, status_message: Some(request_status) })
        },
    }
}

#[get("/")]
async fn hello_route() -> impl Responder {
    HttpResponse::Ok().body("Welcome to pandoc web service!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Access logs are printed with the INFO level so ensure it is enabled by default
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    HttpServer::new(|| {
        App::new().wrap(Logger::new("%a %t %r %s %b \"%{Referer}i\" \"%{User-Agent}i\" %T")).service(hello_route).service(convert_route)
    })
    .bind(("0.0.0.0", 7878))?
    .run()
    .await
}
mod converter;
use crate::converter::convert_file;

use env_logger;
use std::path::PathBuf;
use actix_web::{post, App, HttpServer, HttpResponse, middleware::Logger};
use actix_multipart::{Multipart, MultipartError};
use futures_util::StreamExt;
use std::io::Write;
use std::fs::File;
use tempfile::Builder;
use std::collections::{HashMap, HashSet};


const INPUT_VALID_FORMATS: [&str; 6] = ["csv", "html", "ipynb", "latex", "markdown", "tsv"];
const OUTPUT_VALID_FORMATS: [&str; 5] = ["html", "ipynb", "latex", "markdown", "plain"];

#[post("/convert")]
async fn convert_route(mut payload: Multipart) -> Result<HttpResponse, MultipartError> {
    let mut fields: HashMap<String, String> = HashMap::new();

    let directory = Builder::new().prefix("pandoc-processor-").rand_bytes(8).tempdir().unwrap();
    let input_tmp_path = directory.path().join("input");
    let output_tmp_path = directory.path().join("output");

    while let Some(item) = payload.next().await {
        let mut field = item?;
        let field_name = String::from(field.name());
    
        let text_fields = vec!["input_format", "output_format"];
        let file_fields = vec!["input_file"];
        
        if text_fields.contains(&field_name.as_str()){
            let mut text_value = String::new();

            while let Some(chunk) = field.next().await {
                let data = chunk?;
                text_value.push_str(std::str::from_utf8(&data).unwrap());
            }

            fields.insert(field_name.to_string(), text_value);
        } else if file_fields.contains(&field_name.as_str()){
            let mut input_tmp_file = File::create(input_tmp_path.clone()).unwrap();

            while let Some(chunk) = field.next().await {
                let data = chunk?;
                input_tmp_file.write_all(&data).unwrap();
            }

            fields.insert(field_name.to_string(), input_tmp_path.to_str().unwrap().to_string());
        }
    }

    // Preparing tmp output file
    let _output_tmp_file = File::create(output_tmp_path.clone()).unwrap();
    fields.insert(String::from("output_file"), output_tmp_path.to_str().unwrap().to_string());

    let input_file_path = PathBuf::from(fields.get("input_file").unwrap());
    let output_file_path = PathBuf::from(fields.get("output_file").unwrap());
    let input_format = fields.get("input_format").unwrap();
    let output_format = fields.get("output_format").unwrap();

    let validation_input_format = input_format.to_string();
    let validation_output_format = output_format.to_string();

    let input_valid_formats: HashSet<String> = INPUT_VALID_FORMATS.iter().map(|s| s.to_string()).collect();
    let output_valid_formats: HashSet<String> = OUTPUT_VALID_FORMATS.iter().map(|s| s.to_string()).collect();
    
    if !input_valid_formats.contains(&validation_input_format) {
        return Err(MultipartError::UnsupportedField("Invalid input format".to_string()));
    }

    if !output_valid_formats.contains(&validation_output_format) {
        return Err(MultipartError::UnsupportedField("Invalid output format".to_string()));
    }

    match convert_file(&input_file_path, input_format, &output_file_path, output_format) {
        Ok(_) => {
            // TODO AS AN ENHANCEMENT: support all types, this will not work on binary file type
            let response_body = std::fs::read_to_string(&output_file_path)
                .unwrap_or_else(|_| String::from("Unable to read output file"));
    
            Ok(HttpResponse::Ok().body(response_body))
        },
        Err(error) => {
            Err(MultipartError::UnsupportedField(format!("{:?}",error)))
        },
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Access logs are printed with the INFO level so ensure it is enabled by default
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    HttpServer::new(|| {
        App::new().wrap(Logger::new("%a %t %r %s %b \"%{Referer}i\" \"%{User-Agent}i\" %T")).service(convert_route)
    })
    .bind(("0.0.0.0", 7878))?
    .run()
    .await
}

use axum::{
    extract::{Multipart, Query}, http::{Method, StatusCode}, routing::{get, post}, serve::Listener, Router,
    response::{Response, IntoResponse},
};

use std::{
    env::temp_dir, fs::{File, OpenOptions}, io::{Read, Seek, SeekFrom, Write}, net::TcpListener, string::ParseError
};

use log::{warn, error, info};

async fn infer() -> &'static str {
    "Hello"
}

pub async fn upload_chunk (mut multipart: Multipart) -> impl IntoResponse {

let mut filename = String::new();
let mut chunk_total = 0;
let mut chunk_index = 0;
let mut chunk_data = Vec::new();

while let Some(field) = match multipart.next_field().await {

    Ok(f) => f,
    Err(err) => {
        eprintln!("Error reading multipart field {:?}");
        return StatusCode::BAD_REQUEST;
    } 
    {
        let field_name = field.name().unwrap_or_default().to_string();
        match field_name.as_string() {
            "fileName" => filename = sanitize_filename(&field.text().await.unwrap_or_default()),
            "chunkTotal" => chunk_total = field.text().await.unwrap_or_default().parse().unwrap_or(0),
            "chunkIndex" => chunk_index = field.text().await.unwrap_or_default().parse().unwrap_or(0),
            "chunk" => chunk_data = field.bytes().await.unwrap_or_else(|_| Vec::new()).to_vec(),
            _ => {}
        }
    }
    if filename.is_empty() || chunk_data.is_empty() {
        return StatusCode::BAD_REQUEST;
    }
    let temp = format!("./upload/tmp/{}", filename);
    fs::create_dir_all(&temp_dir).unwrap_or_else(|_| {});
    let chunk_path = format!("{}/chunk_{}", temp, chunk_index);
    let mut file = File::create(&chunk_path).unwrap();

    }
    StatusCode::OK

}

#[tokio::main]
async fn main() {
    let app = Router::new()
        //.route("/", get(home))
        .route("/upload", post(upload_chunk));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
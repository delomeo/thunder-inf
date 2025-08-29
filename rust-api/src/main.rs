use axum::{routing::get, Router};
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

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/infer", get(infer));
    
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
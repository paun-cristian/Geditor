extern crate dotenv;

use axum::{
    Router, 
    routing::{get, get_service},
    response::*};

use tower_http::{trace::TraceLayer, services:: ServeDir};

use std::fs;
use sqlx::mysql::MySqlPool;
use std::net::SocketAddr;
use tracing::{Level, info};
use tracing_subscriber;

pub mod auth;
use crate::auth::*;
use dotenv::dotenv;
use std::env;

#[tokio::main]
async fn main() {

    check_database_url();

    tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .init();
    
    info!("Starting server");
    // Create a MySQL connection pool
    let connection = Connection {
        _pool : MySqlPool::connect("mysql://root:Thefilthycunt777@localhost/mydb")
            .await
            .unwrap(),
        addr : SocketAddr::from(([127, 0, 0, 1], 3000)),
    };

// todo add a service for the static html file
    let app = Router::new()
        .route("/register", get(|| async { read_html("static/index.html").await })
                        .post(User::register))
        .nest_service("/static", get_service(ServeDir::new("static")))
        .layer(TraceLayer::new_for_http());

    let listener = tokio::net::TcpListener::bind(&connection.addr)
        .await
        .unwrap();
    info!("Listening on: {}", connection.addr);

    axum::serve(listener, app).await.unwrap();
}

async fn read_html(url : &str) -> impl IntoResponse {
    let file = fs::read_to_string(url)
        .expect("Failed to read file");
    Html(file)
}

pub fn check_database_url() {
    dotenv().ok();
    match env::var("DATABASE_URL") {
        Ok(url) => println!("DATABASE_URL is set to: {}", url),
        Err(e) => println!("DATABASE_URL is not set: {}", e),
    }
}

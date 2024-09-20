use axum::{
    extract::*,
    response::*};

use serde::Deserialize;
use sqlx::mysql::MySqlPool;
use std::net::SocketAddr;
use tracing::error;
pub mod users;

#[derive(Deserialize)]
pub struct RegisterForm {
    username: Option<String>,
    email: Option<String>,
    password: Option<String>,
}

pub struct Connection {
    pub _pool : MySqlPool,
    pub addr : SocketAddr,
}


pub struct User {
    id: u8,
    name: String,
    email: String,
    password: String,
}

impl User {
    fn new(id: u8, name: String, 
        email: String, password: String) -> User {
        User {
            id,
            name,
            email,
            password,
        }
    }
    pub async fn register(Form(form) : Form<RegisterForm>) -> impl IntoResponse {
        match(form.username, form.email, form.password) {
            (Some(username), Some(email), Some(password)) => {
                let user = User::new(rand::random::<u8>(), 
                                    username.to_string(),
                                    email.to_string(),
                                    password.to_string());
                users::save_user(&user).await.unwrap();
                Html(format!("Welcome, {}", user.name))
            }
            _ => {
                error!("Invalid form");
                Html("Invalid form".to_string())
            }
        }
    }
}



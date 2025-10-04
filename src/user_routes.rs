use axum::{Router,routing::{get}};


pub async fn get_users() {}
pub async fn get_user() {}

pub fn user_router() -> Router {
    Router::new()
    .route("/users", get(get_users))
    .route("/user/:id", get(get_user))
}
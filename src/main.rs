use std::net::SocketAddr;
use axum::{extract::{ConnectInfo}, routing::get, Router};

pub mod response;
pub mod error;
pub mod user_routes;



#[tokio::main(flavor = "current_thread")]
async fn main() {
     let app = init_router();
     let addr = "0.0.0.0:2000";
     let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
     axum::serve(listener, app).await.unwrap();
}


fn init_router() -> Router {
Router::new()
   .route("/", get(hello_handler))
   .merge(user_routes::user_router())
}

async fn hello_handler(ConnectInfo(addr) : ConnectInfo<SocketAddr>) -> String {
    format!("Hello user -> {}",addr.ip())
}
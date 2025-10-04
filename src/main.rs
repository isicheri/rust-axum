use std::{net::SocketAddr};
use axum::{extract::{ConnectInfo}, routing::get, Router};
use sqlx::{PgPool,postgres::PgPoolOptions};
use std::sync::Arc;
use dotenv::dotenv;


pub mod response;
pub mod error;
pub mod user_routes;



struct AppState {
    db:PgPool
}


#[tokio::main(flavor = "current_thread")]
async fn main() {

    env_loader();

     let  db_cn_string = std::env::var("DATABASE_URL").unwrap();

     let pool = PgPoolOptions::new().max_connections(5).connect(&db_cn_string).await.expect("cant connect to database");

     let app_state = Arc::new(AppState { db: pool  });

     let app = init_router().with_state(app_state);
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

fn env_loader() {
    dotenv().ok();
}
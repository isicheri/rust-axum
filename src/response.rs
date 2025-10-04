use axum::{http::StatusCode, response::{IntoResponse, Response}, Json};
use serde::Serialize;


#[derive(Serialize)]
pub struct Message {
    message: String
}

pub enum ApiResponse {
    Ok,
    Created,
    JsonData(Vec<Message>)
}


impl IntoResponse for  ApiResponse {
    
fn into_response(self) -> Response {
    match self  {
        ApiResponse::Ok => (StatusCode::OK).into_response(),
        ApiResponse::Created => (StatusCode::CREATED).into_response(),
        ApiResponse::JsonData(messages) => (StatusCode::OK,Json(messages)).into_response(),
    }
}

}
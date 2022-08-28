use crate::shared::user_schema::User;
use axum::{response::IntoResponse, Json};
use serde_json::json;

///create a new user
pub async fn sign_up(Json(payload): Json<User>) -> impl IntoResponse {
    Json(json!({
        "success":true,
        "message":"user successfully created".to_string(),
        "data":Some(payload)
    }))
}

///login a new user
pub async fn login(Json(payload): Json<User>) -> impl IntoResponse {
    //destructure the request body
    let User {
        email, password, ..
    } = payload;
    Json(json!({
        "email":email,
        "password": password
    }))
}

///reset user password
pub async fn reset_password(Json(payload): Json<User>) -> impl IntoResponse {
    //destructure the request body
    let User { email, .. } = payload;
    Json(json!({
        "email":email,
    }))
}

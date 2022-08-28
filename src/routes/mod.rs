// pub mod sign_up;
use crate::shared::{api_response::ApiResponse, user_schema::User};
use axum::{response::IntoResponse, Json};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Person {
    pub name: String,
    pub age: u32,
}

//sign up
pub async fn sign_up(Json(payload): Json<User>) -> impl IntoResponse {
    let res = ApiResponse::<User> {
        success: true,
        message: "user created".to_string(),
        data: Some(payload),
        error: None,
        status_code: Some(200),
    };

    Json(res)
}

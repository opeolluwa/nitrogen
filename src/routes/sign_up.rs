// pub mod sign_up;
use crate::shared::api_response::ApiResponse;
use axum::{response::IntoResponse, Json};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Person {
    pub name: String,
    pub age: u32,
}

pub async fn sign_up() -> impl IntoResponse {
    let pete = Person {
        name: "Peter".to_string(),
        age: 34,
    };

    let mut res = ApiResponse::<Person> {
        success: true,
        message: "user created".to_string(),
        payload: Some(pete),
        error: None,
        status_code: Some(200),
    };

    Json(res)
}

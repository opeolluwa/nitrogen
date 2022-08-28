use crate::shared::api_response::ApiResponse;
use axum::Json;

pub struct Person {
    pub name: String,
    pub age: u32,
}

pub fn sign_up() -> Json<ApiResponse<Person>> {
    let pete = Person {
        name: "Peter".to_string(),
        age: 34,
    };
    let res = ApiResponse::<Person> {
        success: true,
        message: "user created".to_string(),
        payload: Some(pete),
        error: None,
        status_code: Some(200),
    };

    Json(res)
}

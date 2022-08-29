use crate::{config::database::mongodb, shared::user_schema::User};
use axum::{http::StatusCode, response::IntoResponse, Json};
use bcrypt::{hash, DEFAULT_COST};
// use mongodb::bson::{doc, Document};
use mongodb::bson::oid::ObjectId;
use serde_json::json;

//the user objects
///create a new user
pub async fn sign_up(Json(payload): Json<User>) -> impl IntoResponse {
    //destructure the request
    let User {
        firstname,
        lastname,
        email,
        password,
        ..
    } = payload;
    let database = mongodb().await;
    let collection = database.collection::<User>("user");

    //construct a new user form the request payload
    //  let hashed = hash(password, DEFAULT_COST).unwrap();
    let user = User {
        // id:ObjectId(),
        firstname,
        lastname,
        email,
        password,
    };
    //TODO: validate the user object, first check if user with email already exists
    // let error :Vec<String>;

    //create new user
    let user = collection.insert_one(user, None).await;
    (
        StatusCode::CREATED,
        Json(json!({
            "success":true,
            "message":"user successfully created".to_string(),
            // "data":Some(&user)
        })),
    )
}

///login a new user
pub async fn login(Json(payload): Json<User>) -> impl IntoResponse {
    //destructure the request body
    let hashed = hash("hunter2", DEFAULT_COST);
    println!("{:?}", hashed);
    let User {
        email, password, ..
    } = payload;
    Json(json!({
        "email":email,
        "password": password,
        // "hash":hashed
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

//get the user profile
pub async fn user_profile(Json(_payload): Json<User>) -> impl IntoResponse {}

//update user profile
pub async fn update_user_profile(Json(_payload): Json<User>) -> impl IntoResponse {}

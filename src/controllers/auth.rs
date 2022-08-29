use crate::{
    config::database::mongodb,
    shared::{jwt_schema::JwtSchema, user_schema::User},
};
use axum::{http::StatusCode, response::IntoResponse, Json};
use bcrypt::{hash, verify};
use mongodb::bson::doc;
use serde_json::json;
// use jsonwebtoken::{encode, decode, Header, Algorithm, Validation, EncodingKey, DecodingKey};

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

    //TODO: validate the user object, first check if user with email already exists
    // let error: Vec<String>;
    /*  if assert_eq!(firstname.is_empty(), true) {
        error.push("Firstname cannot be empty".to_string());
    } */

    //construct a new user form the validated request payload
    let hashed_password = hash(password, 12).unwrap();
    let user = User {
        firstname: firstname,
        lastname: lastname,
        email: email,
        password: hashed_password,
    };

    //create new user
    collection.insert_one(user, None).await.unwrap();
    (
        StatusCode::OK,
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
    let User {
        email,
        password: user_password,
        ..
    } = payload;

    //find user by email
    let database = mongodb().await;
    let collection = database.collection::<User>("user");
    let result = collection
        .find_one(doc! { "email": &email }, None)
        .await
        .unwrap();

    //try to destructure the found object
    let (firstname, email, password) = if let Some(User {
        firstname,
        email,
        password,
        ..
    }) = result
    {
        (firstname, email, password)
    } else {
        //if no user was found return 404 error
        return (
            StatusCode::NOT_FOUND,
            Json(json!({
                "success":false,
                "message":"no use with provided credentials was found".to_string(),
                "data":None::<User>
            })),
        );
    };

    //check for correctness of password, if correct send access token
    let _jwt_payload = JwtSchema { email, firstname };

    //compare the password
    let is_correct_password = verify(user_password, &password);
    // println!("the password result is {:?}", correct_password);
    match is_correct_password {
        Ok(_) => (
            //encrypt the user data
            StatusCode::CREATED,
            Json(json!({
                "success":true,
                "message":"user successfully created".to_string(),
                "data":json!({
                    "token":"tt"
                })
            })),
        ),
        Err(_) => (
            StatusCode::UNAUTHORIZED,
            Json(json!({
                "success":false,
                "message":"invalid email or password".to_string(),
                "data":None::<User>
            })),
        ),
    }
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

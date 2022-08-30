/*
* define a sample user object
* the fields are optional by default
*/
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Serialize, Validate, Deserialize)]
pub struct User {
    // pub _id:Option<String>,
    #[validate(length(min = 1, message = "firstname cannot be empty"))]
    pub firstname: Option<String>,
    #[validate(length(min = 1, message = "lastname cannot be empty"))]
    pub lastname: Option<String>,
    #[validate(email)]
    pub email: Option<String>,
    #[validate(length(min = 8, message = "password must be at lease 8 characters"))]
    pub password: String,
}


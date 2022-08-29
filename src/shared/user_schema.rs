/*
* define a sample user object
* the fields are optional by default
*/
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    // pub _id:Option<String>,
    pub firstname: Option<String>,
    pub lastname: Option<String>,
    pub email: Option<String>,
    pub password: String,
}

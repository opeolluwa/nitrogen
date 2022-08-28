/*
* define a sample user object
* the fields are optional by default
*/
use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub firstname: Option<String>,
    pub lastname: Option<String>,
    pub fullname: Option<String>,
    pub email: Option<String>,
    pub username: Option<String>,
    #[serde(rename = "phoneNumber")]
    pub phone_number: Option<String>,
}

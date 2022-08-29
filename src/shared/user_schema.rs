/*
* define a sample user object
* the fields are optional by default
*/
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    // pub id: Option<ObjectId>,
    pub firstname: Option<String>,
    pub lastname: Option<String>,
    // pub fullname: Option<String>,
    pub email: Option<String>,
    // pub username: Option<String>,
    // #[serde(rename = "phoneNumber", skip_serializing_if="Option::is_none")]
    // pub phone_number: Option<String>,
    pub password: Option<String>,
}

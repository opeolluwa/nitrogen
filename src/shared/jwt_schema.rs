use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct JwtSchema {
    // pub id: String,
    pub email: Option<String>,
    pub firstname: Option<String>,
}

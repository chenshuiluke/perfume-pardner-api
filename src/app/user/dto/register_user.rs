use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct RegisterUser {
    pub username: String,
    pub email: String,
    pub password: String,
}

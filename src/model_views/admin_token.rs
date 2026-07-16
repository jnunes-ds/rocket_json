use rocket::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct AdminToken {
    pub id: u32,
    pub name: String,
    pub email: String,
    pub token: String,
}
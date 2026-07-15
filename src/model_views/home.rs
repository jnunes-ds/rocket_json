use rocket::serde::Serialize;

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Home {
    pub message: String,
    pub endpoints: Vec<String>,
}
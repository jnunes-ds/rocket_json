use rocket::serde::Serialize;

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Resource {
    pub id: u32,
    pub title: String,
    pub description: String,
}
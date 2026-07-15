use rocket::serde::json::Json;
use crate::model_views::home::Home;

#[get("/")]
pub fn index() -> Json<Home> {
    Json(
        Home {
            message: "Welcome to the home page".to_string(),
            endpoints: vec![
                "/resource".to_string(),
            ]
        }
    )
}
use rocket::http::Status;
use rocket::response::status;
use rocket::serde::json::Json;
use crate::model_views::error_json::ErrorJson;
use crate::model_views::home::Home;

#[get("/")]
pub fn index() -> Json<Home> {
    Json(
        Home {
            message: "Welcome to the home page".to_string(),
            endpoints: vec![
                "/resources".to_string(),
            ]
        }
    )
}

#[get("/unauthorized")]
pub fn unauthorized() -> status::Custom<Json<ErrorJson>> {
    status::Custom(Status::Unauthorized, Json(ErrorJson {
        message: "You are not authorized to access this resource".to_string()
    }))
}
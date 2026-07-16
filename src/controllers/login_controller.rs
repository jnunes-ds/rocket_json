use rocket::http::Status;
use rocket::response::status;
use rocket::serde::json::Json;
use crate::dto::login_dto::LoginDTO;
use crate::model_views::admin_token::AdminToken;
use crate::model_views::error_json::ErrorJson;
use crate::services::admin_service;

#[post("/login", data = "<login_dto_json>")]
pub fn login(login_dto_json: Json<LoginDTO>) -> Result<
    status::Custom<Json<AdminToken>>,
    status::Custom<Json<ErrorJson>>
> {
    let login_dto = login_dto_json.into_inner();

    match admin_service::login(login_dto.email, login_dto.password) {
        Ok(admin_token) => Ok(status::Custom(Status::Ok, Json(admin_token))),
        Err(error) => Err(status::Custom(Status::BadRequest, Json(ErrorJson {message: error})))
    }
}
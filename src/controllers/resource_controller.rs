use rocket::serde::json::Json;
use crate::models::resource::Resource;
use crate::services::resource_service::get_resources_list;

#[get("/resource")]
pub fn index() -> Json<Vec<Resource>> {
    let resources = get_resources_list();

    Json(resources)
}
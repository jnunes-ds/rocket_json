use rocket::Error;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::response::status;
use crate::dto::resource_dto::ResourceDTO;
use crate::model_views::error_json::ErrorJson;
use crate::models::resource::Resource;
use crate::services::resource_service;

#[get("/resources")]
pub fn index() -> Json<Vec<Resource>> {
    let resources = resource_service::get_resources_list();

    Json(resources)
}

#[post("/resources", data = "<resource_dto_json>")]
pub fn create(resource_dto_json: Json<ResourceDTO>) ->  Result<
    status::Custom<Json<Resource>>,
    status::Custom<Json<ErrorJson>>
> {
    let resource = resource_dto_json.into_inner();

    match resource_service::create_resource(resource) {
        Ok(resource) => Ok(status::Custom(Status::Created, Json(resource))),
        Err(err) => Err(
            status::Custom(Status::BadRequest, Json(ErrorJson { message: err }))
        ),
    }
}

#[put("/resources/<id>", data = "<resource_dto_json>")]
pub fn update(id: u32, resource_dto_json: Json<ResourceDTO>) ->  Result<
    status::Custom<Json<Resource>>,
    status::Custom<Json<ErrorJson>>
> {
    let resource = resource_dto_json.into_inner();

    match resource_service::update_resource(id, resource) {
        Ok(resource) => Ok(status::Custom(Status::Ok, Json(resource))),
        Err(err) => Err(
            status::Custom(Status::BadRequest, Json(ErrorJson { message: err }))
        ),
    }
}

#[get("/resources/<id>")]
pub fn get_by_id(id: u32) ->  Result<
    status::Custom<Json<Resource>>,
    status::Custom<Json<ErrorJson>>
> {

    match resource_service::get_resource_by_id(id) {
        Ok(resource) => Ok(status::Custom(Status::Ok, Json(resource))),
        Err(err) => Err(
            status::Custom(Status::BadRequest, Json(ErrorJson { message: err }))
        ),
    }
}

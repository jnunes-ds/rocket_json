mod controllers;
mod models;
mod model_views;

#[macro_use] extern crate rocket;
use controllers::home_controller::index as home_index;
use controllers::resource_controller::resource_index;

#[launch]
async fn rocket() -> _ {
    rocket::build().mount("/", routes![home_index, resource_index])
}



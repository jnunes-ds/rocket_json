mod controllers;
mod models;
mod model_views;
mod services;

#[macro_use] extern crate rocket;
use controllers::{home_controller, resource_controller};

#[launch]
async fn rocket() -> _ {
    rocket::build().mount("/", routes![
        home_controller::index,
        resource_controller::index
    ])
}



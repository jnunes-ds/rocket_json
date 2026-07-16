mod controllers;
mod models;
mod model_views;
mod services;
mod dto;
mod middlewares;

#[macro_use] extern crate rocket;
use controllers::{ home_controller, resource_controller, login_controller };

#[launch]
async fn rocket() -> _ {
    rocket::build()
        .attach(middlewares::auth_guard::JwtFairing)
        .mount("/", routes![
            home_controller::index,
            home_controller::unauthorized,
            login_controller::login,

            resource_controller::index,
            resource_controller::create,
            resource_controller::update,
            resource_controller::get_by_id,
            resource_controller::delete_by_id,
        ])
}



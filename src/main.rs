mod controllers;

#[macro_use] extern crate rocket;

use rocket::{Build, Rocket};
use rocket::serde::{json::Json, Serialize };

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct Resource {
    id: u32,
    title: String,
    description: String,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct Home {
    message: String,
    endpoints: Vec<String>,
}

#[get("/")]
fn home() -> Json<Home> {
    Json(
        Home {
            message: "Welcome to the home page".to_string(),
            endpoints: vec![
                "/resource".to_string(),
            ]
        }
    )
}

#[get("/resource")]
fn resource_index() -> Json<Vec<Resource>> {
    let resources = vec![
        Resource {
            id: 1,
            title: "Sample Resource 1".to_string(),
            description: "This is the first sample resource.".to_string()
        },
        Resource {
            id: 2,
            title: "Sample Resource 2".to_string(),
            description: "This is the second sample resource.".to_string()
        },
        Resource {
            id: 3,
            title: "Sample Resource 3".to_string(),
            description: "This is the third sample resource.".to_string()
        },
    ];

    Json(resources)
}

#[launch]
async fn rocket() -> _ {
    rocket::build().mount("/", routes![home, resource_index])
}



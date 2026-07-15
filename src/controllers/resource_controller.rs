use rocket::serde::json::Json;
use crate::models::resource::Resource;

#[get("/resource")]
pub fn index() -> Json<Vec<Resource>> {
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
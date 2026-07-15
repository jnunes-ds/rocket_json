use std::io::ErrorKind;
use rocket::Error;
use rand::random;
use crate::dto::resource_dto::ResourceDTO;
use crate::models::resource::Resource;

pub fn get_resources_list() -> Vec<Resource> {
    // Getting from database
    vec![
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
    ]
}

pub fn create_resource(resource_dto: ResourceDTO) -> Result<Resource, String> {
    let resource = Resource {
        id: random(),
        title: resource_dto.title,
        description: resource_dto.description,
    };
    // Use repository to save it on db
    println!("ID: {}", resource.id);
    println!("TITLE: {}", resource.title);
    println!("DESCRIPTION: {}", resource.description);

    if resource.id > 0 {
        Ok(resource)
    } else {
        Err("Error creating resource".to_string())
    }
}
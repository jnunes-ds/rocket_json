use rand::random;
use rocket::Error;
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

pub fn update_resource(id: u32, resource_dto: ResourceDTO) -> Result<Resource, String> {
    let resource = Resource {
        id,
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

pub fn get_resource_by_id(id: u32) -> Result<Resource, String> {
    // Getting from database
    let resourses = get_resources_list();

    match resourses.iter().find(|r| r.id == id) {
        Some(r) => Ok(Resource {
            id: r.id,
            title: r.title.clone(),
            description: r.description.clone()
        }),
        None => {
            Err("Não foi possível encontrar o recurso".to_string())
        }
    }
}

pub fn delete_resource_by_id(id: u32) -> Result<(), String> {
    // Getting from database
    let resourses = get_resources_list();

    match resourses.iter().find(|r| r.id == id) {
        Some(r) => {
            Ok(()) 
        },
        None => {
            Err(format!("O recurso {} remover não existe", id))
        }
    }
}
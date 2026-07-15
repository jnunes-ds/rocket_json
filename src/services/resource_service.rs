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
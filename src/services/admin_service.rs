use crate::model_views::admin_token::AdminToken;
use crate::models::admin::Admin;
use crate::services::jwt_service::generate_jwt_token;

pub fn get_admin_by_email_password(email: String, password: String) -> Option<Admin> {
    let admin = Admin {
        id: 1,
        name: "Fake Admin".to_string(),
        email: "fake@mail.com".to_string(),
        password: "fake".to_string()
    };
    if email == admin.email && password == admin.password {
        Some(admin)
    } else {
        None
    }
}

// Fake login
pub fn login(email: String, password: String) -> Result<AdminToken, String> {
    let adm = get_admin_by_email_password(email, password);
    match adm {
        Some(adm) => Ok(
            AdminToken {
                id: adm.id,
                name: adm.name,
                email: adm.email,
                token: generate_jwt_token(adm.id),
            }
        ),
        None => Err("Invalid credentials".to_string())
    }
}
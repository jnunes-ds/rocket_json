use rocket::fairing::{Fairing, Info, Kind};
use rocket::{Data, Request};
use rocket::http::Method;
use rocket::http::uri::Origin;
use crate::services::jwt_service::verify_token;

pub struct JwtFairing;

#[rocket::async_trait]
impl Fairing for JwtFairing {
    fn info(&self) -> Info {
        Info {
            name: "JWT Authentication Fairing",
            kind: Kind::Request
        }
    }

    async fn on_request(&self, req: &mut Request<'_>, _data: &mut Data<'_>) {
        let open_routes = ["/login", "/"];

        let request_path = req.uri().path();

        if open_routes.contains(&request_path.as_str()) {
            return;
        }

        let valid_token = req.headers().get_one("Authorization")
            .and_then(|header| header.strip_prefix("Bearer "))
            .map(|token| verify_token(token))
            .unwrap_or(false);

        if !valid_token {
            if let Ok(uri) = Origin::parse("/unauthorized") {
                req.set_uri(uri);
                req.set_method(Method::Get)
            }
        }
    }
}
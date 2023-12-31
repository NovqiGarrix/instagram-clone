use actix_web::web::{ServiceConfig, scope};
use super::auth::auth_routes::get_auth_routes;

pub fn get_v1_routes(cfg: &mut ServiceConfig) {
    cfg.service(scope("/auth").configure(get_auth_routes));
}
use actix_web::web::ServiceConfig;

use super::auth_controller::{signup_handler, sign_in_handler, get_new_token_handler, get_me_handler};

pub fn get_auth_routes(cfg: &mut ServiceConfig) {
    cfg.service(signup_handler)
        .service(sign_in_handler)
        .service(get_new_token_handler)
        .service(get_me_handler);
}
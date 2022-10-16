use actix_web::{web::scope, Scope};

use crate::controllers::hello_controller::{delete_hi, get_hi, patch_hi, post_hi};

pub fn hello_router() -> Scope {
    scope("/hello")
        .service(get_hi)
        .service(post_hi)
        .service(patch_hi)
        .service(delete_hi)
}

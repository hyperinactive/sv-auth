use actix_web::{delete, get, patch, post};

#[get("")]
pub async fn get_hi() -> String {
    "get hi".to_string()
}

#[post("")]
pub async fn post_hi() -> String {
    "post hi".to_string()
}

#[patch("")]
pub async fn patch_hi() -> String {
    "patch hi".to_string()
}

#[delete("")]
pub async fn delete_hi() -> String {
    "delete hi".to_string()
}

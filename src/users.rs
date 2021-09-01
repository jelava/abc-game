use actix_web::{HttpResponse, post, web};
use serde::Serialize;
use std::sync::Mutex;

// Creating a user

pub struct UsersState {
    pub users: Mutex<Vec<String>>
}

#[derive(Serialize)]
struct UsersResponse {
    user_id: usize
}

#[post("/users/{name}")]
pub async fn create_user(web::Path(name): web::Path<String>, data: web::Data<UsersState>) -> HttpResponse {
    if let Ok(mut users) = data.users.lock() {
        users.push(name);

        HttpResponse::Created()
            .json(UsersResponse { user_id: users.len() - 1 })
    } else {
        HttpResponse::InternalServerError()
            .body("HTTP 500: Failed to acquire lock")
    }
}
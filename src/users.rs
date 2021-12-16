use crate::{error::Error, HttpResult, State};
use actix_web::{post, web, HttpResponse};
use serde::Serialize;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/users").service(create_user));
}

pub struct User {
    pub name: String,
}

#[derive(Serialize)]
struct UsersResponse {
    user_id: usize,
}

#[post("/{name}")]
async fn create_user(
    web::Path(name): web::Path<String>,
    data: web::Data<State>,
) -> HttpResult {
    let user_id = {
        let mut users = data.users.lock().await;
        users.insert_new(User { name })?
    };

    Ok(HttpResponse::Created().json(UsersResponse { user_id }))
}

impl State {
    pub async fn check_user_id(&self, user_id: usize) -> Result<(), Error> {
        let users = self.users.lock().await;
        users.get(user_id).map(|_| ())
    }
}

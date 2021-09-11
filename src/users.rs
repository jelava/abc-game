use actix_web::{HttpResponse, post, web};
use crate::{error::Error, State};
use serde::Serialize;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/users")
            .service(create_user)
    );
}

pub struct User {
    pub  name: String
    // maybe keep track of total score here?
}

#[derive(Serialize)]
struct UsersResponse {
    user_id: usize
}

#[post("/{name}")]
async fn create_user(web::Path(name): web::Path<String>, data: web::Data<State>) -> HttpResponse {
    let user_id = {
        let mut users = data.users.lock().await;
        users.push(User { name });
        users.len() - 1
    };

    HttpResponse::Created()
        .json(UsersResponse { user_id })
}

impl State {
    pub async fn check_user_id(&self, user_id: usize) -> Result<(), Error> {
        let users = self.users.lock().await;
    
        if user_id < users.len() {
            Ok(())
        } else {
            Err(Error::NonexistentUserId(user_id))
        }
    }
}

use actix_web::{get, HttpResponse, post, web};
use crate::{error::Error, sse, State};
use futures::{channel::mpsc::channel, join};
use serde::Serialize;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/users")
            .service(create_user)
            .service(get_event_stream)
    );
}

pub struct UnconnectedUser {
    pub name: String
}

pub struct ConnectedUser {
    pub name: String,
    pub sender: sse::EventSender,
    //current_game_id: Option<usize>
    // maybe keep track of total score here?
}

#[derive(Serialize)]
struct UsersResponse {
    user_id: usize
}

#[post("/{name}")]
async fn create_user(web::Path(name): web::Path<String>, data: web::Data<State>) -> Result<HttpResponse, Error> {
    let user_id = {
        let mut users = data.unconnected_users.lock().await;
        users.insert_new(UnconnectedUser { name })?
    };

    Ok(HttpResponse::Ok()
        .json(UsersResponse { user_id }))
}

#[get("/{user_id}/events")]
async fn get_event_stream(web::Path(user_id): web::Path<usize>, data: web::Data<State>) -> Result<HttpResponse, Error> {
    let (sender, receiver) = channel::<Result<web::Bytes, Error>>(1024); // TODO should not be hard coded!
    let (mut unconnected_users, mut connected_users) = join!(data.unconnected_users.lock(), data.connected_users.lock());
    let unconnected_user = unconnected_users.remove(user_id)?;

    connected_users.insert_existing(user_id, ConnectedUser {
        name: unconnected_user.name,
        sender: sse::EventSender(sender)
    })?;

    Ok(HttpResponse::Ok()
        .header("content-type", "text/event-stream")
        .streaming(receiver))
}

impl State {
    pub async fn check_user_id(&self, user_id: usize) -> Result<(), Error> {
        let users = self.connected_users.lock().await;
        users.get(user_id)
            .map(|_| ())
    }
}

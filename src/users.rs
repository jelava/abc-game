use crate::{error::Error, sse, State};
use actix_web::{get, web, HttpResponse};
use serde::Serialize;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/users")
            .service(create_user)
            .service(get_event_stream)
            .service(sse_test),
    );
}

pub struct UnconnectedUser {
    pub name: String,
}

pub struct ConnectedUser {
    pub name: String,
    pub sender: sse::EventSender,
    //current_game_id: Option<usize>
    // maybe keep track of total score here?
}

#[derive(Serialize)]
struct UsersResponse {
    user_id: usize,
}

#[get("/{name}")]
async fn create_user(
    web::Path(name): web::Path<String>,
    data: web::Data<State>,
) -> Result<HttpResponse, Error> {
    let (mut sender, receiver) = sse::event_channel(); // TODO should not be hard coded!

    let user_id = {
        let mut users = data.connected_users.lock().await;
        users.insert_new(ConnectedUser {
            name,
            sender: sender.clone(),
        })?
    };

    sender.try_send(sse::Event::UserCreated(user_id))?;

    Ok(HttpResponse::Ok()
        .header("content-type", "text/event-stream")
        .streaming(receiver))
}

#[get("/{user_id}/events")]
async fn get_event_stream(
    web::Path(user_id): web::Path<usize>,
    data: web::Data<State>,
) -> Result<HttpResponse, Error> {
    let (sender, receiver) = sse::event_channel();

    let mut users = data.connected_users.lock().await;
    let user = users.get_mut(user_id)?;
    user.sender = sender;

    Ok(HttpResponse::Ok()
        .header("content-type", "text/event-stream")
        .streaming(receiver))
}

#[get("/{user_id}/sse_test")]
async fn sse_test(
    web::Path(user_id): web::Path<usize>,
    data: web::Data<State>,
) -> Result<HttpResponse, Error> {
    let mut users = data.connected_users.lock().await;
    let user = users.get_mut(user_id)?;
    user.sender
        .try_send_bytes(web::Bytes::from("event: sseTest\ndata: whatever\n\n"))?;

    Ok(HttpResponse::Ok().finish())
}

impl State {
    pub async fn check_connected_user_id(&self, user_id: usize) -> Result<(), Error> {
        let users = self.connected_users.lock().await;
        users.get(user_id).map(|_| ())
    }
}

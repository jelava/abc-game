use crate::{error::Error, sse, State};
use actix_web::{get, web, HttpResponse};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/lobby").service(join_lobby));
}

#[get("")]
async fn join_lobby(
    web::Path(name): web::Path<String>,
    data: web::Data<State>,
) -> Result<HttpResponse, Error> {
    let (sender, receiver) = sse::event_channel();

    data.lobby.lock()
        .await
        .push(sender);

    Ok(HttpResponse::Ok()
        .header("content-type", "text/event-stream")
        .streaming(receiver))
}

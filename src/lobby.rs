use crate::{HttpResult, sse, State};
use actix_web::{get, patch, web, HttpResponse};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/lobby").service(join_lobby));
}

#[get("/join/{user_id}")]
async fn join_lobby(
    web::Path(user_id): web::Path<usize>,
    data: web::Data<State>,
) -> HttpResult {
    data.check_user_id(user_id).await?;

    let (sender, receiver) = sse::event_channel();
    data.lobby.lock().await.insert_existing(user_id, sender)?;

    Ok(HttpResponse::Ok()
        .header("content-type", "text/event-stream")
        .streaming(receiver))
}

#[patch("/leave/{user_id}")]
async fn leave_lobby(
    web::Path(user_id): web::Path<usize>,
    data: web::Data<State>,
) -> HttpResult {
    let sender = data.lobby.lock().await.remove(user_id)?;
    Ok(HttpResponse::Ok().finish())
}

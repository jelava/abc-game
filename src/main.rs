mod error;
mod games;
mod users;

use actix_files::Files;
use actix_web::{App, HttpServer, middleware, web};
use crate::{games::Game, users::User};
use futures::lock::Mutex;
use std::default::Default;

#[derive(Default)]
pub struct State {
    games: Mutex<Vec<Game>>,
    users: Mutex<Vec<User>>
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
    let data = web::Data::new(State::default());

    HttpServer::new(move || {
        log::info!("Building app");

        App::new()
            .wrap(middleware::Logger::default())
            .app_data(data.clone())
            .configure(users::config)
            .configure(games::config)
            .service(
                Files::new("/", "./static")
                    .index_file("index.html")
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

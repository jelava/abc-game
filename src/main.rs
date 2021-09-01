use actix_files::Files;
use actix_web::{App, HttpResponse, HttpServer, middleware, post, put, web};
use serde::Serialize;
use std::sync::Mutex;

mod users;

// Creating (hosting) a game

struct Game {
    host_id: usize,
    name: String,
    players: Vec<usize>
}

struct GamesState {
    games: Mutex<Vec<Game>>
}

#[derive(Serialize)]
struct GamesResponse {
    game_id: usize
}

#[post("/games/{name}/host/{host_id}")]
async fn host_game(web::Path(name): web::Path<String>, web::Path(host_id): web::Path<usize>, data: web::Data<GamesState>) -> HttpResponse {
    if let Ok(mut games) = data.games.lock() {
        games.push(Game { host_id, name, players: Vec::new() });

        HttpResponse::Created()
            .json(GamesResponse { game_id: games.len() - 1 })
    } else {
        HttpResponse::InternalServerError()
            .body("HTTP 500: Failed to acquire lock")
    }
}

// Joining an existing game

#[put("/games/{game_id}/join/{player_id}")]
async fn join_game(web::Path(game_id): web::Path<usize>, web::Path(player_id): web::Path<usize>, data: web::Data<GamesState>) -> HttpResponse {
    if let Ok(mut games) = data.games.lock() {
        games[game_id].players.push(player_id);

        HttpResponse::Ok()
            .json(GamesResponse { game_id: games.len() - 1 })
    } else {
        HttpResponse::InternalServerError()
            .body("HTTP 500: Failed to acquire lock")
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();


    HttpServer::new(|| {
        let users_data = web::Data::new(users::UsersState { users: Mutex::new(Vec::new()) });
        let games_data = web::Data::new(GamesState { games: Mutex::new(Vec::new()) });
    
        App::new()
            .wrap(middleware::Logger::default())
            .service(
                web::scope("/")
                    .app_data(users_data.clone())
                    .service(users::create_user)
            )
            .service(
                web::scope("/")
                    .app_data(games_data.clone())
                    .service(host_game)
                    .service(join_game)
                    //.service(list_games)
            )
            .service(
                Files::new("/", "./static")
                    .index_file("index.html")
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

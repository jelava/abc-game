use actix_web::{get, HttpResponse, patch, post, web};
use crate::{error::Error, State};
use futures::{FutureExt, join, try_join};
use serde::Serialize;
use std::collections::HashSet;

const LETTERS: [char; 26] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z'
];

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/games")
            .service(host_game)
            .service(join_game)
            .service(get_games)
    );
}

/// State related to a game running on the server. Games are stored in a vector behind a mutex,
/// so there is no need for additional mutexes on the initials and players fields (because only
/// one thread should ever be allowed to access the `Game` struct from the  at a time).
pub struct Game {
    name: String,
    host_name: String,
    config: GameConfig,
    initials: Vec<(char, char)>,
    players: HashSet<usize>
}

impl Game {
    fn new(name: String, host_name: String, config: GameConfig) -> Self {
        let mut game = Self {
            name,
            host_name: host_name,
            config,
            initials: Vec::with_capacity(config.num_initials),
            players: HashSet::new()
        };
        
        game.generate_initials();
        game
    }

    fn generate_initials(&mut self) {
        for i in 0..self.config.num_initials {
            self.initials.push((LETTERS[i % self.config.num_initials], LETTERS[i % self.config.num_initials]));
        }
    }
}

#[derive(Copy, Clone)]
struct GameConfig {
    num_initials: usize
}

impl Default for GameConfig {
    fn default() -> Self {
        Self { num_initials: 26 }
    }
}

// Hosting a new game

#[derive(Serialize)]
struct HostGameResponse {
    game_id: usize
}

#[post("/{name}/host/{host_id}")]
async fn host_game<'a>(web::Path((name, host_id)): web::Path<(String, usize)>, data: web::Data<State>) -> Result<HttpResponse, Error> {
    let (users, mut games) = join!(data.users.lock(), data.games.lock());
    
    let host_name = users.get(host_id)
        .ok_or(Error::NonexistentUserId(host_id))?
        .name
        .clone();
    
    let new_game = Game::new(name, host_name, GameConfig::default());
    
    games.push(new_game);
    let game_id = games.len() - 1;

    Ok(HttpResponse::Created()
        .json(HostGameResponse { game_id }))
}

// Joining an existing game

#[derive(Serialize)]
struct JoinGameResponse<'a> {
    initials: &'a Vec<(char, char)>
}

#[patch("/{game_id}/join/{player_id}")]
async fn join_game<'a>(web::Path((game_id, player_id)): web::Path<(usize, usize)>, data: web::Data<State>) -> Result<HttpResponse, Error> {
    let (_, mut games) = try_join!(data.check_user_id(player_id), data.games.lock().map(|games| Ok(games)))?;

    let game = games.get_mut(game_id)
        .ok_or(Error::NonexistentGameId(game_id))?;
    
    game.players.insert(player_id);

    Ok(HttpResponse::Ok()
        .json(JoinGameResponse { initials: &game.initials }))
}

// Get a list of existing games

#[derive(Serialize)]
struct GameInfo<'a> {
    name: &'a str,
    host_name: &'a str,
    player_count: usize
}

#[derive(Serialize)]
struct GetGamesResponse<'a> {
    games: Vec<GameInfo<'a>>
}

impl<'a> From<&'a Game> for GameInfo<'a> {
    fn from(game: &'a Game) -> Self {
        Self {
            name: game.name.as_str(),
            host_name: game.host_name.as_str(),
            player_count: game.players.len()
        }
    }
}

#[get("")]
async fn get_games<'a>(data: web::Data<State>) -> HttpResponse {
    let games = data.games.lock().await;

    let game_infos = games.iter()
        .map(|game| game.into())
        .collect();

    HttpResponse::Ok()
        .json(GetGamesResponse { games: game_infos })
}

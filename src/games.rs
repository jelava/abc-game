use actix_web::{get, HttpResponse, patch, post, put, web};
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
            .service(start_game)
    );
}

/// State related to a game running on the server. Games are stored in a vector behind a mutex,
/// so there is no need for additional mutexes on the initials and players fields (because only
/// one thread should ever be allowed to access the `GameData` struct at a time).
pub struct OpenGame {
    name: String,
    host_name: String,
    config: GameConfig,
    //initials: Vec<(char, char)>,
    players: HashSet<usize>
}

pub struct ActiveGame {
    config: GameConfig,
    initials: Vec<(char, char)>,
    players: HashSet<usize>
}

// Open game - joining
// start: open -> active
// Active game - get initials, type names
// timer end: active -> scoring
// Scoring game - 
// all scores submitted: scoring -> finished
// Finished game - show scores
// all players get scores: remove finished game

pub struct ScoringGame {
    // TODO!
}

pub struct FinishedGame {
    // TODO!
}

impl From<OpenGame> for ActiveGame {
    fn from(game: OpenGame) -> Self {
        Self {
            config: game.config,
            initials: generate_initials(game.config.num_initials),
            players: game.players
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
    let (users, mut games) = join!(data.connected_users.lock(), data.open_games.lock());

    let host_name = users.get(host_id)?
        .name
        .clone();

    let new_game = OpenGame {
        name,
        host_name,
        config: GameConfig::default(),
        players: HashSet::new()
    };

    let game_id = games.insert_new(new_game)?;

    Ok(HttpResponse::Created()
        .json(HostGameResponse { game_id }))
}

// Joining an existing game

#[patch("/{game_id}/join/{player_id}")]
async fn join_game<'a>(web::Path((game_id, player_id)): web::Path<(usize, usize)>, data: web::Data<State>) -> Result<HttpResponse, Error> {
    let (_, mut games) = try_join!(
        data.check_user_id(player_id),
        data.open_games
            .lock()
            .map(|games| Ok(games))
    )?;

    let game = games.get_mut(game_id)?;
    
    game.players.insert(player_id);

    Ok(HttpResponse::Ok()
        .finish())
}

// Get a list of existing games

#[derive(Serialize)]
struct GameInfo<'a> {
    name: &'a str,
    host_name: &'a str,
    player_count: usize
}

impl<'a> From<&'a OpenGame> for GameInfo<'a> {
    fn from(game: &'a OpenGame) -> Self {
        Self {
            name: game.name.as_str(),
            host_name: game.host_name.as_str(),
            player_count: game.players.len()
        }
    }
}

#[derive(Serialize)]
struct GetGamesResponse<'a> {
    games: Vec<GameInfo<'a>>
}

#[get("")]
async fn get_games<'a>(data: web::Data<State>) -> HttpResponse {
    let games = data.open_games.lock().await;

    let game_infos = games.iter()
        .map(|game| game.into())
        .collect();

    HttpResponse::Ok()
        .json(GetGamesResponse { games: game_infos })
}

// Starting a game

#[put("/${game_id}/start")]
async fn start_game<'a>(web::Path(game_id): web::Path<usize>, data: web::Data<State>) -> Result<HttpResponse, Error> {
    let (mut open_games, mut active_games, mut users) = join!(
        data.open_games.lock(),
        data.active_games.lock(),
        data.connected_users.lock()
    );

    let game = open_games.remove(game_id)?;
    active_games.insert_existing(game_id, game.into())?;

    for user in users.iter_mut() {
        todo!();
        user.sender.try_send(Ok(web::Bytes::from("event: startGame\ndata: TODO\n\n")))?;
    }

    Ok(HttpResponse::Ok()
        .finish())
}

fn generate_initials(num_initials: usize) -> Vec<(char, char)> {
    let mut initials = Vec::new();

    for i in 0..num_initials {
        initials.push((LETTERS[i % 26], LETTERS[i % 26]))
    }

    initials
}

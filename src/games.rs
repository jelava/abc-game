use crate::{error::Error, HttpResult, sse, State};
use actix_web::{get, patch, post, web, HttpResponse, ResponseError};
use futures::join;
use serde::Serialize;
use std::collections::HashSet;

const LETTERS: [char; 26] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
];

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/games")
            .service(get_games)
            .service(join_game)
            .service(host_game)
            .service(start_game),
    );
}

/// State related to a game running on the server. Games are stored in a vector behind a mutex,
/// so there is no need for additional mutexes on the initials and players fields (because only
/// one thread should ever be allowed to access an `OpenGame` struct at a time).
pub struct OpenGame {
    name: String,
    host_name: String,
    config: GameConfig,
    //initials: Vec<(char, char)>,
    players: HashSet<usize>,
}

pub struct ActiveGame {
    config: GameConfig,
    initials: Vec<(char, char)>,
    players: HashSet<usize>,
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
            players: game.players,
        }
    }
}

fn generate_initials(num_initials: usize) -> Vec<(char, char)> {
    let mut initials = Vec::new();

    for i in 0..num_initials {
        initials.push((LETTERS[i % 26], LETTERS[i % 26]))
    }

    initials
}

#[derive(Copy, Clone)]
struct GameConfig {
    num_initials: usize,
}

impl Default for GameConfig {
    fn default() -> Self {
        Self { num_initials: 26 }
    }
}

// Hosting a new game

#[derive(Serialize)]
struct HostGameResponse {
    game_id: usize,
}

#[post("/{name}/host/{host_id}")]
async fn host_game<'a>(
    web::Path((name, host_id)): web::Path<(String, usize)>,
    data: web::Data<State>,
) -> HttpResult {
    let (users, mut games) = join!(data.connected_users.lock(), data.open_games.lock());

    let host_name = users.get(host_id)?.name.clone();

    let new_game = OpenGame {
        name,
        host_name,
        config: GameConfig::default(),
        players: HashSet::new(),
    };

    let game_id = games.insert_new(new_game)?;

    Ok(HttpResponse::Created().json(HostGameResponse { game_id }))
}

// Joining an open game

#[derive(Serialize)]
pub struct GameInfo<'a> {
    game_id: usize,
    name: &'a str,
    host_name: &'a str,
    player_count: usize,
}

impl<'a> GameInfo<'a> {
    fn new(game_id: usize, game: &'a OpenGame) -> Self {
        Self {
            game_id,
            name: game.name.as_str(),
            host_name: game.host_name.as_str(),
            player_count: game.players.len(),
        }
    }
}

#[patch("/{game_id}/join/{player_id}")]
async fn join_game<'a>(
    web::Path((game_id, player_id)): web::Path<(usize, usize)>,
    data: web::Data<State>,
) -> Result<HttpResponse, Error> {
    let (mut users, mut games) = join!(data.connected_users.lock(), data.open_games.lock());

    let game = games.get_mut(game_id)?;
    game.players.insert(player_id);

    for user in users.values_mut() {
        let event = sse::Event::GameOpened(GameInfo::new(game_id, game));
        user.sender.try_send(event)?;
    }

    Ok(HttpResponse::Ok().finish())
}

// Get a list of open games

#[derive(Serialize)]
struct GetGamesResponse<'a> {
    games: Vec<GameInfo<'a>>,
}

#[get("")]
async fn get_games<'a>(data: web::Data<State>) -> HttpResponse {
    let games = data.open_games.lock().await;
    let game_infos = games
        .iter()
        .map(|(game_id, game)| GameInfo::new(*game_id, game))
        .collect();

    HttpResponse::Ok().json(GetGamesResponse { games: game_infos })
}

// Starting a game

#[patch("/{game_id}/start")]
async fn start_game<'a>(
    web::Path(game_id): web::Path<usize>,
    data: web::Data<State>,
) -> Result<HttpResponse, Error> {
    let (mut open_games, mut active_games, mut users) = join!(
        data.open_games.lock(),
        data.active_games.lock(),
        data.connected_users.lock()
    );

    let game: ActiveGame = open_games.remove(game_id)?.into();

    for (user_id, user) in users.iter_mut() {
        let event = if game.players.contains(user_id) {
            sse::Event::StartGame(&game.initials)
        } else {
            sse::Event::GameClosed(game_id)
        };

        // TODO: do not return early here
        // The game shouldn't be 'lost' between states because of an SSE error
        user.sender.try_send(event)?;
    }

    active_games.insert_existing(game_id, game)?;

    Ok(HttpResponse::Ok().finish())
}

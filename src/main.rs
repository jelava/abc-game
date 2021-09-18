mod error;
mod games;
mod sse;
mod users;

use actix_files::Files;
use actix_web::{App, HttpServer, middleware, web};
use crate::{error::*, games::*, users::*};
use futures::lock::Mutex;
use std::{collections::HashMap, default::Default};

pub struct State {
    unconnected_users: Mutex<IdMap<UnconnectedUser>>,
    connected_users: Mutex<IdMap<ConnectedUser>>,
    open_games: Mutex<IdMap<OpenGame>>,
    active_games: Mutex<IdMap<ActiveGame>>
}

pub struct IdMap<V> {
    next_id: usize,
    map: HashMap<usize, V>,
    duplicate_id_error: Error,
    nonexistent_id_error: Error
}

impl<V> IdMap<V> {
    fn new(duplicate_id_error: Error, nonexistent_id_error: Error) -> Self {
        Self {
            next_id: 0,
            map: HashMap::new(),
            duplicate_id_error,
            nonexistent_id_error
        }
    }
    
    fn get(&self, id: usize) -> Result<&V, Error> {
        self.map.get(&id)
            .ok_or(self.nonexistent_id_error)
    }

    fn get_mut(&mut self, id: usize) -> Result<&mut V, Error> {
        self.map.get_mut(&id)
            .ok_or(self.nonexistent_id_error)
    }

    fn insert_new(&mut self, value: V) -> Result<usize, Error> {
        let id = self.next_id;
        self.next_id += 1;
        let maybe_old_value = self.map.insert(id, value);

        match maybe_old_value {
            Some(_) => Err(self.duplicate_id_error),
            None => Ok(id)
        }
    }

    fn insert_existing(&mut self, id: usize, value: V) -> Result<(), Error> {
        match self.map.insert(id, value) {
            Some(_) => Err(self.duplicate_id_error),
            None => Ok(())
        }
    }

    fn remove(&mut self, id: usize) -> Result<V, Error> {
        self.map.remove(&id)
            .ok_or(self.nonexistent_id_error)
    }

    fn iter(&self) -> std::collections::hash_map::Values<'_, usize, V> {
        self.map.values()
    }

    fn iter_mut(&mut self) -> std::collections::hash_map::ValuesMut<'_, usize, V> {
        self.map.values_mut()
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    let duplicate_user_id_error = Error::Internal(InternalError::DuplicateUserId);
    let nonexistent_user_id_error = Error::NonexistentUserId;
    let duplicate_game_id_error = Error::Internal(InternalError::DuplicateGameId);
    let nonexistent_game_id_error = Error::NonexistentGameId;

    let data = web::Data::new(State {
        unconnected_users: Mutex::new(IdMap::new(duplicate_user_id_error, nonexistent_user_id_error)),
        connected_users: Mutex::new(IdMap::new(duplicate_user_id_error, nonexistent_user_id_error)),
        open_games: Mutex::new(IdMap::new(duplicate_game_id_error, nonexistent_game_id_error)),
        active_games: Mutex::new(IdMap::new(duplicate_game_id_error, nonexistent_game_id_error))
    });

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .app_data(data.clone())
            .configure(users::config)
            .configure(games::config)
            .service(
                Files::new("/", "./static")
                    .index_file("createUser.html")
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

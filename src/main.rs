mod error;
mod games;
mod lobby;
mod sse;
mod users;

use crate::{error::*, games::*, lobby::*, sse::EventSender, users::*};
use actix_files::Files;
use actix_web::{middleware, web, App, HttpResponse, HttpServer};
use futures::lock::Mutex;
use std::{collections::HashMap, default::Default};

pub type HttpResult = Result<HttpResponse, Error>;

pub struct State {
    users: Mutex<IdMap<User>>,
    lobby: Mutex<IdMap<EventSender>>,
    open_games: Mutex<IdMap<OpenGame>>,
    active_games: Mutex<IdMap<ActiveGame>>,
}

#[derive(Default)]
pub struct IdMap<V> {
    next_id: usize,
    map: HashMap<usize, V>,
}

impl<V> IdMap<V> {
    fn new() -> Self {
        Self {
            next_id: 0,
            map: HashMap::new(),
        }
    }

    fn get(&self, id: usize) -> Result<&V, Error> {
        self.map.get(&id).ok_or(Error::NonexistentId(id))
    }

    fn get_mut(&mut self, id: usize) -> Result<&mut V, Error> {
        self.map.get_mut(&id).ok_or(Error::NonexistentId(id))
    }

    fn insert_new(&mut self, value: V) -> Result<usize, Error> {
        let id = self.next_id;
        self.next_id += 1;
        let maybe_old_value = self.map.insert(id, value);

        match maybe_old_value {
            Some(_) => Err(Error::Internal(InternalError::DuplicateId(id))),
            None => Ok(id),
        }
    }

    fn insert_existing(&mut self, id: usize, value: V) -> Result<(), Error> {
        match self.map.insert(id, value) {
            Some(_) => Err(Error::Internal(InternalError::DuplicateId(id))),
            None => Ok(()),
        }
    }

    fn remove(&mut self, id: usize) -> Result<V, Error> {
        self.map.remove(&id).ok_or(Error::NonexistentId(id))
    }

    fn values(&self) -> std::collections::hash_map::Values<'_, usize, V> {
        self.map.values()
    }

    fn values_mut(&mut self) -> std::collections::hash_map::ValuesMut<'_, usize, V> {
        self.map.values_mut()
    }

    fn iter(&self) -> std::collections::hash_map::Iter<'_, usize, V> {
        self.map.iter()
    }

    fn iter_mut(&mut self) -> std::collections::hash_map::IterMut<'_, usize, V> {
        self.map.iter_mut()
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    let data = web::Data::new(State {
        //unconnected_users: Mutex::new(IdMap::new()),
        users: Mutex::new(IdMap::new()),
        lobby: Mutex::new(IdMap::new()),
        open_games: Mutex::new(IdMap::new()),
        active_games: Mutex::new(IdMap::new()),
    });

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .app_data(data.clone())
            .configure(users::config)
            .configure(lobby::config)
            .configure(games::config)
            .service(Files::new("/", "./static").index_file("index.html"))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

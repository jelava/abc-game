use crate::{error::Error, games::GameInfo};
use actix_web::web::Bytes;
use futures::{
    channel::mpsc::{channel, Receiver, Sender},
    Stream,
};
use log::{error, info};
use std::{
    pin::Pin,
    task::{Context, Poll},
};

// TODO: this should be configurable?
const CHANNEL_BUFFER_SIZE: usize = 512;

#[derive(Clone)]
pub struct EventSender(pub Sender<Bytes>);

pub struct EventReceiver(Receiver<Bytes>);

pub fn event_channel() -> (EventSender, EventReceiver) {
    let (sender, receiver) = channel(CHANNEL_BUFFER_SIZE);
    (EventSender(sender), EventReceiver(receiver))
}

impl EventSender {
    pub fn try_send(&mut self, event: Event) -> Result<(), Error> {
        let name = event.name();
        let Self(sender) = self;

        match event.into() {
            Ok(bytes) => match sender.try_send(bytes) {
                Ok(_) => {
                    info!("Sent event: {}", name);
                    Ok(())
                }
                Err(send_error) => {
                    error!("Failed to send event: {:?}", send_error);
                    Err(send_error.into())
                }
            },
            Err(serialization_error) => {
                error!("Failed to serialize event: {:?}", serialization_error);
                Err(serialization_error.into())
            }
        }
    }

    /*
    fn try_send_bytes(&mut self, bytes: Bytes) -> Result<(), Error> {
        let Self(sender) = self;
        sender.try_send(bytes).map_err(|error| error.into())
    }
    */
}

impl Stream for EventReceiver {
    type Item = Result<Bytes, actix_web::Error>;

    fn poll_next(mut self: Pin<&mut Self>, context: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        match Pin::new(&mut self.0).poll_next(context) {
            Poll::Ready(Some(bytes)) => Poll::Ready(Some(Ok(bytes))),
            Poll::Ready(None) => Poll::Ready(None),
            Poll::Pending => Poll::Pending,
        }
    }
}

pub enum Event<'a> {
    GameClosed(usize),
    GameOpened(GameInfo<'a>),
    StartGame(&'a Vec<(char, char)>),
    UserCreated(usize),
}

impl<'a> Event<'a> {
    fn name(&self) -> &'static str {
        use Event::*;

        match self {
            GameClosed(_) => "gameClosed",
            GameOpened(_) => "gameOpened",
            UserCreated(_) => "userCreated",
            StartGame(_) => "startGame",
        }
    }

    fn data(&self) -> serde_json::Result<String> {
        use Event::*;

        match self {
            GameClosed(game_id) => Ok(format!("{}", game_id)),
            GameOpened(game_info) => serde_json::to_string(game_info),
            UserCreated(user_id) => Ok(format!("{}", user_id)),
            StartGame(initials) => {
                let mut data = String::new();

                for (first_initial, last_initial) in initials.iter() {
                    data.push(*first_initial);
                    data.push(*last_initial);
                    data.push(' ');
                }

                Ok(data)
            }
        }
    }
}

impl<'a> From<Event<'a>> for serde_json::Result<Bytes> {
    fn from(event: Event) -> Self {
        Ok(format!("event: {}\ndata: {}\n\n", event.name(), event.data()?).into())
    }
}

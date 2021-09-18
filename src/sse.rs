use actix_web::web::Bytes;
use crate::error::Error;
use futures::channel::mpsc::{Sender, TrySendError};

pub struct EventSender(pub Sender<Result<Bytes, Error>>);

impl EventSender {
    pub fn try_send(&mut self, event: Event) -> Result<(), TrySendError<Result<Bytes, Error>>> {
        let Self(sender) = self;
        sender.try_send(Ok(event.into()))
    }
}

pub enum Event<'a> {
    StartGame(&'a Vec<(char, char)>)
}

impl<'a> Event<'a> {
    fn name(&self) -> &'static str {
        use Event::*;

        match self {
            StartGame(_) => "startGame"
        }
    }

    fn data(&self) -> String {
        use Event::*;

        let mut data = String::new();

        match self {
            StartGame(initials) => {
                for (first_initial, last_initial) in initials.iter() {
                    data.push(*first_initial);
                    data.push(*last_initial);
                    data.push(' ');
                }
            }
        }

        data
    }
}

impl<'a> From<Event<'a>> for Bytes {
    fn from(event: Event) -> Self {
        format!("event: {}\ndata: {}\n\n", event.name(), event.data()).into()
    }
}

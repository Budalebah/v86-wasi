mod address;
mod configuration;
mod dev;
mod error;
mod platform;
mod poll;
mod token;
mod event;
mod interest;

pub use error::{Error, Result};

pub use platform::Fd;

pub use address::{EtherAddr, IntoAddress};

pub use configuration::Configuration;

pub use interest::Interest;

pub use event::{Event, Events};

pub use token::Token;

pub use poll::Poll;

pub use dev::{Tap, Device};

#[derive(Clone, Debug)]
pub enum Model {
    Tap,
    Tun
}

impl Default for Model {
    fn default() -> Self {
        Self::Tap
    }
}


use std::fmt;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Player {
    None,
    CPU,
    Human,
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Player::None => write!(f, "None"),
            &Player::CPU => write!(f, "CPU"),
            &Player::Human => write!(f, "Human"),
        }
    }
}

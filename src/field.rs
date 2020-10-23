use crate::error::GameError;
use std::convert::TryFrom;
use std::fmt;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Field {
    Empty = 0,
    Nought = 1,
    Cross = 2,
}

impl Field {
    pub fn reverse(&self) -> Field {
        match self {
            &Field::Nought => Field::Cross,
            &Field::Cross => Field::Nought,
            _ => panic!("Cant reverese empty"),
        }
    }
}

impl fmt::Display for Field {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Field::Empty => write!(f, "[ ]"),
            &Field::Nought => write!(f, "[0]"),
            &Field::Cross => write!(f, "[X]"),
        }
    }
}

impl From<u8> for Field {
    fn from(v: u8) -> Field {
        match v {
            x if x == Field::Nought as u8 => Field::Nought,
            x if x == Field::Cross as u8 => Field::Cross,
            _ => Field::Empty,
        }
    }
}

// impl TryFrom<u8> for Field {
//     type Error = GameError;

//     fn try_from(v: u8) -> Result<Self, Self::Error> {
//         match v {
//             x if x == Field::Empty as u8 => Ok(Field::Empty),
//             x if x == Field::Nought as u8 => Ok(Field::Nought),
//             x if x == Field::Cross as u8 => Ok(Field::Cross),
//             _ => Err(GameError::new("cannot unpack field byte".into())),
//         }
//     }
// }

use std::fmt;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Copy, Clone)]
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

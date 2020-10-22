use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct GameError {
  message: String,
}

impl GameError {
  pub fn new(message: String) -> GameError {
    GameError { message }
  }
}

impl fmt::Display for GameError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "GameError: {}", self.message)
  }
}

impl Error for GameError {}

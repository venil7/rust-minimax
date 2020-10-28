use std::error::Error;
use std::fmt;
use std::str::Utf8Error;

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

impl From<Utf8Error> for GameError {
  fn from(e: Utf8Error) -> GameError {
    GameError::new(format!("{}", e))
  }
}

impl From<std::io::Error> for GameError {
  fn from(e: std::io::Error) -> GameError {
    GameError::new(format!("{}", e))
  }
}

impl From<std::array::TryFromSliceError> for GameError {
  fn from(e: std::array::TryFromSliceError) -> GameError {
    GameError::new(format!("{}", e))
  }
}

impl Error for GameError {}

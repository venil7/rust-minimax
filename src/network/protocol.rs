use crate::error::GameError;
use crate::field::Field;
use crate::player::Player;

const REQUEST_NEW_GAME: u8 = 0xfa;
const REQUEST_MOVE: u8 = 0xfb;
const RESPONSE_GAME_STATE: u8 = 0xfc;
const RESPONSE_ERROR: u8 = 0xfd;

#[derive(Debug)]
pub enum Frame {
  RequestNewGame,
  RequestMove(u8),
  ResponseGameState { fields: Vec<Field>, winner: Player },
  ResponseError(String),
  Null,
}

impl Frame {
  pub fn parse(buffer: &[u8]) -> Result<(Frame, usize), GameError> {
    match buffer[0] {
      REQUEST_NEW_GAME => Ok((Frame::RequestNewGame, 1)),
      REQUEST_MOVE => Ok((Frame::RequestMove(buffer[1]), 2)),
      RESPONSE_GAME_STATE => {
        if buffer.len() < 11 {
          Err(GameError::new("game state not long enough".into()))
        } else {
          let fields: Vec<Field> = buffer[1..=9].iter().map(|f| (*f).into()).collect();
          let winner: Player = buffer[10].into();
          Ok((Frame::ResponseGameState { fields, winner }, 1 + 9 + 1))
        }
      }
      RESPONSE_ERROR => {
        let len = buffer[1] as usize;
        let message = std::str::from_utf8(&buffer[2..2 + len])?.to_owned();
        Ok((Frame::ResponseError(message), len + 2))
      }
      _ => Err(GameError::new("protocol error".into())),
    }
  }

  pub fn serialize(&self) -> Vec<u8> {
    match self {
      Frame::Null => vec![],
      Frame::RequestNewGame => vec![REQUEST_NEW_GAME],
      Frame::RequestMove(idx) => vec![REQUEST_MOVE, *idx],
      Frame::ResponseGameState { fields, winner } => {
        let mut response = vec![0; 1 + 9 + 1];
        response[0] = RESPONSE_GAME_STATE;
        response[1..=9].clone_from_slice(&fields.iter().map(|f| *f as u8).collect::<Vec<u8>>());
        response[10] = *winner as u8;
        response
      }
      Frame::ResponseError(message) => {
        let msg_bytes = message.as_bytes();
        let msg_bytes_len = msg_bytes.len() as u8;
        let mut response = vec![0; 1 + 1 + msg_bytes_len as usize];
        response[0] = RESPONSE_ERROR;
        response[1] = msg_bytes_len as u8;
        response[2..].copy_from_slice(msg_bytes);
        response
      }
    }
  }
}

mod test {
  use super::*;
  // use crate::field::Field;
  // use crate::player::Player;

  #[test]
  fn test_serialize_parse_new_game() {
    let buffer = Frame::RequestNewGame.serialize();
    let output = Frame::parse(&buffer);
    match output {
      Ok((Frame::RequestNewGame, _)) => (),
      _ => panic!("failed to (de)serialize req new game"),
    }
  }
  #[test]
  fn test_serialize_parse_req_move() {
    let buffer = Frame::RequestMove(4).serialize();
    let output = Frame::parse(&buffer);
    match output {
      Ok((Frame::RequestMove(4), _)) => (),
      _ => panic!("failed to (de)serialize req move "),
    }
  }
  #[test]
  fn test_serialize_parse_game_state() {
    let fields: Vec<Field> = vec![
      Field::Empty,
      Field::Cross,
      Field::Nought,
      Field::Empty,
      Field::Cross,
      Field::Nought,
      Field::Empty,
      Field::Cross,
      Field::Nought,
    ];
    let winner = Player::None;
    let buffer = Frame::ResponseGameState { fields, winner }.serialize();
    let output = Frame::parse(&buffer);
    match output {
      Ok((
        Frame::ResponseGameState {
          fields,
          winner: Player::None,
        },
        _,
      )) => {
        if fields.len() != 9 {
          panic!("failed at fields length")
        }
      }
      _ => panic!("failed to (de)serialize game state "),
    }
  }

  #[test]
  fn test_serialize_parse_response_error() {
    let buffer = Frame::ResponseError("some arbitrary text".into()).serialize();
    let output = Frame::parse(&buffer);
    match output {
      Ok((Frame::ResponseError(message), _)) => {
        if message != "some arbitrary text" {
          panic!("text doesnt match")
        }
      }
      _ => panic!("failed to (de)serialize res error "),
    }
  }
}

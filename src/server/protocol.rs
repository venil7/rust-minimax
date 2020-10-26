use crate::error::GameError;
use crate::field::Field;
use crate::player::Player;

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
      0xAA => Ok((Frame::RequestNewGame, 1)),
      0xAB => Ok((Frame::RequestMove(buffer[1]), 2)),
      0xAC => {
        let fields: Vec<Field> = buffer[1..=9].iter().map(|f| (*f).into()).collect();
        let winner: Player = buffer[10].into();
        Ok((Frame::ResponseGameState { fields, winner }, 1 + 9 + 1))
      }
      0xAD => {
        let len = buffer[1];
        let message = std::str::from_utf8(&buffer[2..2 + len as usize])?.to_owned();
        Ok((Frame::ResponseError(message), len as usize + 2))
      }
      _ => Err(GameError::new("protocol error".into())),
    }
  }

  pub fn serialize(&self) -> Vec<u8> {
    match self {
      Frame::Null => vec![],
      Frame::RequestNewGame => vec![0xAA],
      Frame::RequestMove(idx) => vec![0xAB, *idx],
      Frame::ResponseGameState { fields, winner } => {
        let mut response = vec![0; 1 + 9 + 1];
        response[0] = 0xAC;
        response[1..=9].clone_from_slice(&fields.iter().map(|f| *f as u8).collect::<Vec<u8>>());
        response[10] = *winner as u8;
        response
      }
      Frame::ResponseError(message) => {
        let msg_bytes = message.as_bytes();
        let msg_bytes_len = msg_bytes.len() as u8;
        let mut response = vec![0xAD + 1 + msg_bytes_len];
        response[1] = msg_bytes_len as u8;
        response[2..].copy_from_slice(msg_bytes);
        response
      }
    }
  }
}

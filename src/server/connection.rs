use crate::error::GameError;
use crate::server::protocol::Frame;
use std::error::Error;
use tokio::io::AsyncReadExt;
use tokio::net::TcpStream;

pub struct Connection {
  stream: TcpStream,
  buffer: Vec<u8>,
  index: usize,
}

impl Connection {
  pub fn new(stream: TcpStream) -> Connection {
    Connection {
      stream,
      buffer: vec![0; 64],
      index: 0,
    }
  }

  pub async fn read_frame(&mut self) -> Result<Option<Frame>, Box<dyn Error>> {
    loop {
      if let Ok(frame) = self.parse_frame() {
        return Ok(Some(frame));
      }
      let bytes_read = self.stream.read(&mut self.buffer[self.index..]).await?;
      if 0 == bytes_read {
        if self.buffer.is_empty() {
          return Ok(None);
        } else {
          return Err(Box::new(GameError::new("connection reset by peer".into())));
        }
      }
      self.index += bytes_read;
    }
  }

  pub async fn write_frame(&mut self, _frame: &Frame) -> Result<(), GameError> {
    unimplemented!("write_frame");
  }

  pub fn parse_frame(&mut self) -> Result<Frame, GameError> {
    unimplemented!("parse_frame");
  }
}

use crate::error::GameError;
use crate::network::protocol::Frame;
use std::net::SocketAddr;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

pub struct Connection {
  stream: TcpStream,
  buffer: Vec<u8>,
  index: usize,
  address: SocketAddr,
}

const BUFFER_SIZE: usize = 64;

impl Connection {
  pub fn new(stream: TcpStream, address: SocketAddr) -> Connection {
    Connection {
      stream,
      buffer: vec![0; BUFFER_SIZE],
      index: 0,
      address,
    }
  }

  pub fn client_id(&self) -> String {
    format!("{}", self.address)
  }

  pub async fn read_frame(&mut self) -> Result<Frame, GameError> {
    loop {
      if let Ok(frame) = self.parse_frame() {
        return Ok(frame);
      }
      let bytes_read = self.stream.read(&mut self.buffer[self.index..]).await?;
      if 0 == bytes_read {
        // if self.buffer.is_empty() {
        //   return Ok(None);
        // } else {
        return Err(GameError::new("connection reset by peer".into()));
        // }
      }
      self.index += bytes_read;
    }
  }

  pub async fn write_frame(&mut self, frame: &Frame) -> Result<(), GameError> {
    let bytes = frame.serialize();
    let bytes_written = self.stream.write(&bytes[..]).await?;
    if bytes.len() == bytes_written {
      Ok(())
    } else {
      Err(GameError::new(
        "Could not write all bytes to the stream".into(),
      ))
    }
  }

  pub fn parse_frame(&mut self) -> Result<Frame, GameError> {
    let (frame, size) = Frame::parse(&self.buffer)?;
    self.buffer.drain(0..size);
    self.buffer.resize(BUFFER_SIZE, 0);
    self.index -= size;
    Ok(frame)
  }
}

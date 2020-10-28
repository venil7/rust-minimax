use crate::board::Board;
use crate::error::GameError;
use crate::network::connection::Connection;
use crate::network::protocol::Frame;
use crate::player::Player;
use tokio::net::TcpStream;

pub struct Client {
  connection: Connection,
  board: Board,
  winner: Player,
}

impl Client {
  pub async fn connect(uri: &str) -> Result<Client, GameError> {
    let stream = TcpStream::connect(uri).await?;
    let addr = stream.local_addr()?;
    let connection = Connection::new(stream, addr);
    Ok(Client {
      connection,
      board: Board::new(),
      winner: Player::None,
    })
  }

  async fn read_response(&mut self) -> Result<Frame, GameError> {
    let frame = self.connection.read_frame().await?;

    match frame {
      Frame::ResponseError(msg) => Err(GameError::new(msg)),
      frame => Ok(frame),
    }
  }

  pub fn board(&self) -> Board {
    self.board.clone()
  }

  async fn write_frame(&mut self, frame: &Frame) -> Result<(), GameError> {
    self.connection.write_frame(frame).await
  }

  pub async fn new_game(&mut self) -> Result<Frame, GameError> {
    self.write_frame(&Frame::RequestNewGame).await?;
    self.read_response().await
  }

  pub async fn request_move(&mut self, idx: u8) -> Result<(), GameError> {
    let frame = &Frame::RequestMove(idx);
    self.write_frame(frame).await?;
    let frame = self.read_response().await?;
    if let Frame::ResponseGameState { fields, winner } = frame {
      self.board = Board::from_vec(&fields)?;
      self.winner = winner;
    }
    Ok(())
  }
}

use crate::board::Board;
use crate::field::Field;
use crate::player::Player;
use crate::server::connection::Connection;
use crate::server::protocol::Frame;
use crate::state::State;
use std::collections::HashMap;
use std::error::Error;
use tokio::sync::mpsc; //::{Receiver, Sender};
use tokio::sync::oneshot; //::{Receiever, Sender};

pub type Message = (String, Frame, oneshot::Sender<Frame>);

pub struct GameServer {
  games: HashMap<String, Board>,
}

impl GameServer {
  pub fn new() -> GameServer {
    GameServer {
      games: HashMap::new(),
    }
  }

  pub async fn respond(&mut self, receiver: &mut mpsc::Receiver<Message>) {
    while let Some((client_id, frame, respond)) = receiver.recv().await {
      let response_frame = match frame {
        // User requested new game
        Frame::RequestNewGame => {
          let board = Board::new();
          self.games.insert(client_id, board.clone());
          Frame::ResponseGameState {
            fields: board.to_vec(),
            winner: Player::None,
          }
        }
        // User requested a move
        Frame::RequestMove(position) => {
          let board = self.games[&client_id].clone();
          match board
            .set(position as usize, Field::Cross)
            .and_then(|user_board| user_board.cpu())
          {
            Ok(cpu_board) => {
              let state = cpu_board.state();
              self.games.insert(client_id, cpu_board.clone());
              Frame::ResponseGameState {
                fields: board.to_vec(),
                winner: (match state {
                  State::GameOver(winner) => winner,
                  _ => Player::None,
                }),
              }
            }
            Err(error) => Frame::ResponseError(format!("{}", error)),
          }
        }

        _ => Frame::Null,
      };

      respond.send(response_frame).unwrap();
    }
  }

  pub async fn process(
    connection: &mut Connection,
    transmitter: mpsc::Sender<Message>,
  ) -> Result<(), Box<dyn Error>> {
    while let Ok(frame) = connection.read_frame().await {
      let (oneshot_transmitter, oneshot_receiver) = oneshot::channel();
      let message: Message = (connection.client_id(), frame, oneshot_transmitter);
      transmitter.send(message).await?;
      let response_frame = oneshot_receiver.await?;
      connection.write_frame(&response_frame).await?;
    }

    Ok(())
  }
}

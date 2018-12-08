use crate::board::*;
use crate::field::*;
use crate::player::*;
use wasm_bindgen::prelude::*;

type OnMoveEvent = fn(Board) -> ();
type OnGameOverEvent = fn(Player) -> ();

#[wasm_bindgen]
pub struct Game {
  on_move: OnMoveEvent,
  on_game_over: OnGameOverEvent,
  board: Board,
}

#[wasm_bindgen]
impl Game {
  pub fn new(on_move: OnMoveEvent, on_game_over: OnGameOverEvent) -> Game {
    Game {
      on_move: on_move,
      on_game_over: on_game_over,
      board: Board::new(),
    }
  }

  pub fn make_move(mut self, position: usize, field: Field) -> Result<usize, String> {
    let board = self.board.set(position, field)?;
    self.board = board.cpu();
    if self.board.state().game_over {
      (self.on_game_over)(self.board.state().winner);
    } else {
      (self.on_move)(self.board);
    }
    Ok(position)
  }
}

use crate::board::*;
use crate::field::*;

use js_sys;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Game {
  board: Board,
  on_cpu_move: js_sys::Function,
  on_game_over: js_sys::Function,
}

#[wasm_bindgen]
impl Game {
  pub fn new(on_move: js_sys::Function, on_game_over: js_sys::Function) -> Game {
    Game {
      board: Board::new(),
      on_cpu_move: on_move,
      on_game_over: on_game_over,
    }
  }

  pub fn make_move(mut self, position: usize, field: Field) {
    let result = self.board.set(position, field);
    match result {
      Ok(board) => self.board = board.cpu(),
      _ => (),
    }
    let state = self.board.state();
    let this = JsValue::NULL;

    if state.game_over {
      let _ = self.on_game_over.call0(&this);
    } else {
      let index = JsValue::from(self.board.last_move);
      let _ = self.on_cpu_move.call1(&this, &index);
    }
  }
}

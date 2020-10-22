use crate::board::*;
use crate::field::*;

use js_sys;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Game {
  board: Board,
}

#[wasm_bindgen]
impl Game {
  pub fn new() -> Game {
    Game {
      board: Board::new(),
    }
  }

  pub fn make_move(&self, position: usize, field: Field, callback: js_sys::Function) {
    let result = self.board.set(position, field);
    match result {
      Ok(board) => {
        let board = board.cpu();
        let game = Game { board };
        let _state = game.board.state();
        let this = JsValue::NULL;

        let winner = JsValue::from(/*state.winner*/ 0 as u8);
        let last_index = JsValue::from(0 as u8 /*game.board.last_move*/);
        let game = JsValue::from(game);

        let _ = callback.call3(&this, &last_index, &winner, &game);
      }
      _ => (),
    }
  }
}

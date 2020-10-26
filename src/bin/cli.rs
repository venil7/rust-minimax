use darkruby_tictactoe::board::Board;
use darkruby_tictactoe::field::Field;
use darkruby_tictactoe::state::State;
use std::error::Error;
use std::io;

fn main() -> Result<(), Box<dyn Error>> {
  let mut board = Board::new();
  println!("{}", board);

  loop {
    println!("Your move: [0-8]: ");
    let mut position = String::new();
    io::stdin().read_line(&mut position)?;

    let position: usize = match position.trim().parse() {
      Ok(num) => num,
      Err(_) => continue,
    };

    board = match board.set(position, Field::Cross) {
      Ok(new_board) => new_board,
      Err(game_error) => {
        println!("Error: {}", game_error);
        continue;
      }
    };

    board = board.cpu()?;

    println!("{}", board);

    if let State::GameOver(winner) = board.state() {
      println!("{} wins the game", winner);
      break;
    }
  }

  Ok(())
}

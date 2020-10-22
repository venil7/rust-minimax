use darkruby_tictactoe::state::State;
use darkruby_tictactoe::*;
use std::io;

fn main() {
  let mut game = board::Board::new();
  println!("{}", game);

  loop {
    println!("Your move: [0-8]: ");
    let mut position = String::new();
    io::stdin()
      .read_line(&mut position)
      .expect("failed to read line");

    let position: usize = match position.trim().parse() {
      Ok(num) => num,
      Err(_) => continue,
    };

    game = match game.set(position, field::Field::Cross) {
      Ok(new_game) => new_game,
      Err(message) => {
        println!("Error: {}", message);
        continue;
      }
    };

    game = game.cpu();

    println!("{}", game);

    if let State::GameOver(winner) = game.state() {
      println!("{} wins the game", winner);
      break;
    }
  }
}

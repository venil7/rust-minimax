mod field;
mod board;
mod state;
mod player;
use std::io;

fn main() {
    let mut game = board::Board::new();

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

        println!("{}", game);

        match game.state() {
            state @ state::State { game_over: true, .. } => {
                println!("{} wins the game", state.winner);
                break;
            }
            _ => continue,
        }
    }
}

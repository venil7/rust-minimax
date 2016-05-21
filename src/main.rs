mod field;
mod board;

fn main() {
    let board = board::Board::new();
    println!("board --> {}", board);
}

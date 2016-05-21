mod field;
mod board;
mod state;
mod player;

fn main() {
    let board1 = board::Board::new();
    let board2 = board1.set(1, field::Field::Cross);
    let board3 = board2.set(0, field::Field::Nought);
    println!("{}", board1);
    println!("{}", board2);
    println!("{}", board3);

    let state3 = board3.state();
    println!("{}", state3.winner);
}

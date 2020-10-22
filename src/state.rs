use crate::player;

#[derive(Debug)]
pub enum State {
    GameOver(player::Player),
    GameContinues { possible_moves: Vec<usize> },
}

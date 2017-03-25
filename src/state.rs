use player;

#[derive(Debug)]
#[allow(dead_code)]
pub struct State {
    pub game_over: bool,
    pub possible_moves: Vec<usize>,
    pub winner: player::Player,
}

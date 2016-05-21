use std::fmt;
use field::Field;
use state::State;
use player::Player;

const LENGTH: usize = 9;

pub struct Board {
    fields: [Field; LENGTH],
}

impl Board {
    pub fn new() -> Board {
        Board { fields: [Field::Empty; LENGTH] }
    }

    pub fn set(&self, position: usize, field: Field) -> Board {
        let mut fields = self.fields.clone();
        fields[position] = field;
        Board { fields: fields }
    }

    pub fn state(&self) -> State {
        let mut state = State {
            game_over: false,
            possible_moves: vec![],
            winner: Player::None,
        };

        // calculate winning state
        let combinations = [[0, 1, 2], [3, 4, 5], [6, 7, 8], [0, 3, 6], [1, 4, 7], [2, 5, 8],
                            [0, 4, 8], [2, 4, 6]];

        for comb in combinations.into_iter() {
            match (self.fields[comb[0]], self.fields[comb[1]], self.fields[comb[2]]) {
                (Field::Cross, Field::Cross, Field::Cross) => {
                    state.game_over = true;
                    state.winner = Player::Human;
                    break;
                }
                (Field::Nought, Field::Nought, Field::Nought) => {
                    state.game_over = true;
                    state.winner = Player::CPU;
                    break;
                }
                _ => continue,
            }
        }

        // calculate possible moves
        if !state.game_over {
            for idx in 0..LENGTH {
                match self.fields[idx] {
                    Field::Empty => {
                        state.possible_moves.push(idx);
                    }
                    _ => continue,
                }
            }
        }

        state
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
               "{} {} {}\n{} {} {}\n{} {} {}\n",
               self.fields[0],
               self.fields[1],
               self.fields[2],
               self.fields[3],
               self.fields[4],
               self.fields[5],
               self.fields[6],
               self.fields[7],
               self.fields[8])
    }
}
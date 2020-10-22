use crate::error::GameError;
use crate::eval::Eval;
use crate::field::Field;
use crate::player::Player;
use crate::state::State;
use rayon::prelude::*;
use std::fmt;

const LENGTH: usize = 9;
pub type Fields = [Field; LENGTH];

pub struct Board {
    fields: Fields,
}

impl Board {
    pub fn new() -> Board {
        Board {
            fields: [Field::Empty; LENGTH],
        }
    }
    pub fn from_fields(fields: &Fields) -> Board {
        Board {
            fields: fields.clone(),
        }
    }

    pub fn set(&self, position: usize, field: Field) -> Result<Board, GameError> {
        if position >= LENGTH {
            return Err(GameError::new("Out of range".into()));
        }

        let mut fields = self.fields.clone();
        match fields[position] {
            Field::Empty if position < LENGTH => {
                fields[position] = field;
                Ok(Board::from_fields(&fields))
            }
            _ => Err(GameError::new("Field is already set".into())),
        }
    }

    pub fn state(&self) -> State {
        // calculate winning state
        let combinations = [
            [0, 1, 2],
            [3, 4, 5],
            [6, 7, 8],
            [0, 3, 6],
            [1, 4, 7],
            [2, 5, 8],
            [0, 4, 8],
            [2, 4, 6],
        ];

        for comb in combinations.iter() {
            match (
                self.fields[comb[0]],
                self.fields[comb[1]],
                self.fields[comb[2]],
            ) {
                (Field::Cross, Field::Cross, Field::Cross) => {
                    return State::GameOver(Player::Human);
                }
                (Field::Nought, Field::Nought, Field::Nought) => {
                    return State::GameOver(Player::Human);
                }
                _ => continue,
            }
        }

        // calculate possible moves
        let mut possible_moves = vec![];
        for idx in 0..LENGTH {
            match self.fields[idx] {
                Field::Empty => {
                    possible_moves.push(idx);
                }
                _ => continue,
            }
        }

        if possible_moves.len() == 0 {
            return State::GameOver(Player::None);
        }

        State::GameContinues { possible_moves }
    }

    pub fn minimax(board: &Board, field: Field, depth: i64) -> Result<Eval, GameError> {
        let state = board.state();
        return match state {
            State::GameOver(Player::CPU) => Ok(Eval {
                position: 0,
                score: 10 - depth,
            }),
            State::GameOver(Player::Human) => Ok(Eval {
                position: 0,
                score: depth - 10,
            }),
            State::GameOver(Player::None) => Ok(Eval {
                position: 0,
                score: 0,
            }),
            State::GameContinues { possible_moves } => {
                let evaluated_moves: Vec<Eval> = possible_moves
                    .par_iter()
                    .map(|possible_move| {
                        let cloned_board = board.set(*possible_move, field)?;
                        let score =
                            Board::minimax(&cloned_board, field.reverse(), depth + 1)?.score;
                        Ok(Eval::new(*possible_move, score)) as Result<Eval, GameError>
                    })
                    .filter(|e| e.is_ok())
                    .map(|e| e.unwrap() as Eval)
                    .collect::<Vec<Eval>>();

                let mut cloned_moves = evaluated_moves.clone();
                cloned_moves.sort();

                match field {
                    Field::Cross => {
                        let first = cloned_moves
                            .first()
                            .ok_or(GameError::new("No best move fox X".into()))?;
                        return Ok(*first);
                    }
                    Field::Nought => {
                        let last = cloned_moves
                            .last()
                            .ok_or(GameError::new("No best move for O".into()))?;
                        return Ok(*last);
                    }
                    _ => return Err(GameError::new("Should set either X or O".into())),
                };
            }
        };
    }

    pub fn cpu(&self) -> Result<Board, GameError> {
        match self.state() {
            State::GameOver(_) => Ok(Board::from_fields(&self.fields)),
            _ => {
                let eval = Board::minimax(self, Field::Nought, 1)?;
                self.set(eval.position, Field::Nought)
            }
        }
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} {} {}\n{} {} {}\n{} {} {}\n",
            self.fields[0],
            self.fields[1],
            self.fields[2],
            self.fields[3],
            self.fields[4],
            self.fields[5],
            self.fields[6],
            self.fields[7],
            self.fields[8]
        )
    }
}

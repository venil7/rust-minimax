use std::fmt;

#[derive(Copy,Clone)]
pub enum Field {
    Empty = 0,
    Nought = 1,
    Cross = 2,
}

impl fmt::Display for Field {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Field::Empty => write!(f, "[ ]"),
            &Field::Nought => write!(f, "[0]"),
            &Field::Cross => write!(f, "[X]"),
        }
    }
}
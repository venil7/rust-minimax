use std::fmt;
use field::Field;

const LENGTH:usize = 9;

pub struct Board {
    fields: [Field; LENGTH]
}

impl Board {
    pub fn new() -> Board {
        Board { fields: [Field::Empty; LENGTH] }
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {}\n{} {} {}\n{} {} {}\n",
            self.fields[0], self.fields[1], self.fields[2],
            self.fields[3], self.fields[4], self.fields[5],
            self.fields[6], self.fields[7], self.fields[8])
    }
}
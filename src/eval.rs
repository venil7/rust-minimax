use std::cmp::Ordering;

#[derive(Copy,Clone)]
pub struct Eval {
    pub position: usize,
    pub score: i64,
}

impl Eval {
    pub fn new(position: usize, score: i64) -> Eval {
        Eval {
            position: position,
            score: score,
        }
    }
}

impl Ord for Eval {
    fn cmp(&self, other: &Self) -> Ordering {
        self.score.cmp(&other.score)
    }
}

impl PartialOrd for Eval {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Eval {
    fn eq(&self, other: &Self) -> bool {
        (self.position, self.score) == (other.position, other.score)
    }
}

impl Eq for Eval {}
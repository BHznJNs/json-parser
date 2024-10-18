/// row, col
#[derive(Clone, Copy, Debug)]
pub struct Position(usize, usize);

impl Position {
    pub fn new(row: usize, col: usize) -> Self {
        Position(row, col)
    }

    pub fn incr_col(&mut self) {
        self.1 += 1;
    }
    pub fn incr_row(&mut self) {
        self.0 += 1;
        self.1  = 0;
    }
}

impl std::fmt::Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

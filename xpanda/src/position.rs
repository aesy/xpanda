#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Position {
    pub index: usize,
    pub line: usize,
    pub col: usize,
}

impl Position {
    pub const fn new(index: usize, line: usize, col: usize) -> Self {
        Self { index, line, col }
    }
}

impl Default for Position {
    fn default() -> Self {
        Self {
            index: 0,
            line: 1,
            col: 1,
        }
    }
}

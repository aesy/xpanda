#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Position {
    pub index: usize,
    pub line: usize,
    pub col: usize,
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

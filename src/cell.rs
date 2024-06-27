#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Cell {
    Alive,
    Dead,
}

impl Cell {
    pub fn is_alive(&self) -> bool {
        matches!(self, Cell::Alive)
    }
}



#[derive(Debug, Clone, Copy, PartialEq, Hash, Eq)]
pub enum Direction {
    RIGHT,
    LEFT,
    UP,
    DOWN,
    EndOfMap,
}

impl Direction {
    pub fn opposite(&self) -> Direction {
        match self {
            Direction::RIGHT => Direction::LEFT,
            Direction::LEFT => Direction::RIGHT,
            Direction::UP => Direction::DOWN,
            Direction::DOWN => Direction::UP,
            Direction::EndOfMap => Direction::EndOfMap,
        }
    }
}

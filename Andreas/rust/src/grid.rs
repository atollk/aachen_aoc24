use std::collections::HashMap;

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
pub struct Position {
    x: u32,
    y: u32,
}

impl Position {
    fn move_to(&self, direction: Direction) -> Position {
        self.move_by(direction, 1)
    }

    fn move_by(&self, direction: Direction, distance: u32) -> Position {
        match direction {
            Direction::Up => Position {
                x: self.x,
                y: self.y.wrapping_add(distance),
            },
            Direction::Down => Position {
                x: self.x,
                y: self.y.wrapping_sub(distance),
            },
            Direction::Left => Position {
                x: self.x.wrapping_sub(distance),
                y: self.y,
            },
            Direction::Right => Position {
                x: self.x.wrapping_add(distance),
                y: self.y,
            },
        }
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
pub struct Grid<T> {
    pub(crate) width: u32,
    pub(crate) height: u32,
    pub(crate) entities: HashMap<Position, Vec<T>>,
}

impl<T> Grid<T> {
    fn from_singleton_grid(entities: &[T]) -> Grid<T> {}

    fn is_in_range(&self, position: Position) -> bool {
        position.x < self.width && position.y < self.height
    }

    fn get(&self, position: Position) -> impl Iterator<Item=&T> {
        self.entities[&position].iter()
    }
}
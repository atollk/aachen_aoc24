use itertools::Itertools;

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
pub struct Position {
    pub(crate) x: u32,
    pub(crate) y: u32,
}

impl Position {
    pub fn move_to(&self, direction: Direction) -> Position {
        self.move_by(direction, 1)
    }

    pub fn move_by(&self, direction: Direction, distance: u32) -> Position {
        match direction {
            Direction::Up => Position {
                x: self.x,
                y: self.y.wrapping_sub(distance),
            },
            Direction::Down => Position {
                x: self.x,
                y: self.y.wrapping_add(distance),
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

#[derive(Debug, Clone)]
pub struct Grid<T> {
    pub width: u32,
    pub height: u32,
    pub entities: Vec<T>,
}

impl<T: PartialEq> Grid<T> {
    pub fn is_in_range(&self, position: Position) -> bool {
        position.x < self.width && position.y < self.height
    }

    pub fn get(&self, position: Position) -> Option<&T> {
        self.entities
            .get((position.x + position.y * self.width) as usize)
    }

    pub fn set(&mut self, position: Position, new: T) {
        self.entities[(position.x + position.y * self.width) as usize] = new;
    }

    pub fn find<'a>(&self, object: &'a T) -> impl Iterator<Item=Position> + use < 'a, '_, T > {
        self.entities
            .iter()
            .positions(|entity| *entity == *object)
            .map(|i| Position {
                x: i as u32 % self.width,
                y: i as u32 / self.width,
            })
    }

    pub fn pretty_print(&self, render: &dyn Fn(&T) -> char) -> String {
        let mut result = String::new();
        for y in 0..self.height {
            for x in 0..self.width {
                result.push(render(self.get(Position { x, y }).unwrap()))
            }
            result.push('\n');
        }
        result
    }
}

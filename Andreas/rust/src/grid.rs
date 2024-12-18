use itertools::Itertools;

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone, Ord, PartialOrd)]
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

impl Direction {
    pub fn all() -> [Direction; 4] {
        [
            Direction::Down,
            Direction::Up,
            Direction::Left,
            Direction::Right,
        ]
    }

    pub fn turn90(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        }
    }

    pub fn turn180(&self) -> Direction {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }

    pub fn turn270(&self) -> Direction {
        match self {
            Direction::Up => Direction::Left,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down,
            Direction::Right => Direction::Up,
        }
    }
}

impl From<char> for Direction {
    fn from(value: char) -> Self {
        match value {
            '^' => Direction::Up,
            '<' => Direction::Left,
            '>' => Direction::Right,
            'v' => Direction::Down,
            _ => unreachable!(),
        }
    }
}

impl From<Direction> for char {
    fn from(value: Direction) -> Self {
        match value {
            Direction::Up => '^',
            Direction::Down => 'v',
            Direction::Left => '<',
            Direction::Right => '>',
        }
    }
}

#[derive(Debug, Clone)]
pub struct Grid<T> {
    pub width: u32,
    pub height: u32,
    pub entities: Vec<T>,
}

impl<T: PartialEq> Grid<T> {
    pub fn parse_from_string<F>(string: &str, mapper: F) -> Result<Grid<T>, ()>
    where
        F: Fn(char) -> T,
    {
        let width = string.lines().next().ok_or(())?.chars().count() as u32;
        let height = string.lines().count() as u32;
        let entities: Vec<_> = string
            .lines()
            .flat_map(|line| line.chars().map(|c| mapper(c)))
            .collect();
        if width * height != entities.len() as u32 {
            Err(())
        } else {
            Ok(Grid {
                width,
                height,
                entities,
            })
        }
    }

    pub fn get(&self, position: Position) -> Option<&T> {
        if position.x > self.width || position.y > self.height {
            None
        } else {
            self.entities
                .get((position.x + position.y * self.width) as usize)
        }
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

    pub fn pretty_print<F>(&self, render: F) -> String
    where
        F: Fn(Position, &T) -> char,
    {
        let mut result = String::new();
        for y in 0..self.height {
            for x in 0..self.width {
                let pos = Position { x, y };
                result.push(render(pos, self.get(pos).unwrap()))
            }
            result.push('\n');
        }
        result
    }
}

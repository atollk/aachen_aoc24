use ::std::fs::File;
use std::collections::{BinaryHeap, HashSet};
use std::io::{self, prelude::*, BufReader};

use itertools::Itertools;

#[derive(Debug, Hash, Eq, PartialEq, Ord, PartialOrd, Copy, Clone)]
struct Position(usize, usize);

impl Position {
    fn up(&self) -> Self {
        Position(self.0 - 1, self.1)
    }

    fn down(&self) -> Self {
        Position(self.0 + 1, self.1)
    }

    fn left(&self) -> Self {
        Position(self.0, self.1 - 1)
    }

    fn right(&self) -> Self {
        Position(self.0, self.1 + 1)
    }
}

#[derive(Debug, Hash, Eq, PartialEq, Ord, PartialOrd, Copy, Clone)]
enum Orientation {
    North,
    East,
    South,
    West,
}

#[derive(Debug, Hash, Eq, PartialEq, Ord, PartialOrd, Copy, Clone)]
struct Pose {
    position: Position,
    orientation: Orientation,
}

use std::cmp::Ordering;

#[derive(Debug, PartialEq, Eq)]
struct HeapElem(u32, Pose);

impl PartialOrd for HeapElem {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.0.partial_cmp(&self.0)
    }
}

impl Ord for HeapElem {
    fn cmp(&self, other: &HeapElem) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}
impl Pose {
    fn new(position: Position, orientation: Orientation) -> Self {
        Self {
            position,
            orientation,
        }
    }

    fn rotate_ccw(&self) -> Self {
        match self.orientation {
            Orientation::North => Pose::new(self.position, Orientation::West),
            Orientation::East => Pose::new(self.position, Orientation::North),
            Orientation::South => Pose::new(self.position, Orientation::East),
            Orientation::West => Pose::new(self.position, Orientation::South),
        }
    }

    fn rotate_cw(&self) -> Self {
        match self.orientation {
            Orientation::North => Pose::new(self.position, Orientation::East),
            Orientation::East => Pose::new(self.position, Orientation::South),
            Orientation::South => Pose::new(self.position, Orientation::West),
            Orientation::West => Pose::new(self.position, Orientation::North),
        }
    }

    fn advance(&self) -> Self {
        match self.orientation {
            Orientation::North => Pose::new(self.position.up(), self.orientation),
            Orientation::East => Pose::new(self.position.right(), self.orientation),
            Orientation::South => Pose::new(self.position.down(), self.orientation),
            Orientation::West => Pose::new(self.position.left(), self.orientation),
        }
    }
}

fn find_start(grid: &Vec<Vec<char>>) -> Pose {
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == 'S' {
                return Pose::new(Position(i, j), Orientation::East);
            }
        }
    }
    panic!()
}

fn at(pose: &Pose, grid: &Vec<Vec<char>>) -> char {
    grid[pose.position.0][pose.position.1]
}

fn solve1(grid: &Vec<Vec<char>>) -> u32 {
    let start = find_start(&grid);

    let mut heap = BinaryHeap::new();
    heap.push(HeapElem(0, start));
    let mut visited = HashSet::new();

    let mut target = 0;
    while !heap.is_empty() {
        let HeapElem(weight, pose) = heap.pop().unwrap();
        if at(&pose, &grid) == 'E' {
            return weight;
        }

        if visited.contains(&pose) {
            continue;
        }

        visited.insert(pose);
        target = target.max(weight);

        let rotate_ccw_pose = pose.rotate_ccw().advance();
        if !visited.contains(&rotate_ccw_pose) && !(at(&rotate_ccw_pose, grid) == '#') {
            heap.push(HeapElem(weight + 1001, rotate_ccw_pose));
        }
        let rotate_cw_pose = pose.rotate_cw().advance();
        if !visited.contains(&rotate_cw_pose) && !(at(&rotate_cw_pose, grid) == '#') {
            heap.push(HeapElem(weight + 1001, rotate_cw_pose));
        }
        let advance_pose = pose.advance();
        if !visited.contains(&advance_pose) && !(at(&advance_pose, grid) == '#') {
            heap.push(HeapElem(weight + 1, advance_pose));
        }
    }
    target
}

fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let grid = reader
        .lines()
        .map(|line| line.unwrap().chars().collect_vec())
        .take_while(|row| !row.is_empty())
        .collect_vec();

    let res = solve1(&grid);

    println!("{:?}", res);

    Ok(())
}

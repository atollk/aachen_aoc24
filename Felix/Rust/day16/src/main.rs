use ::std::fs::File;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
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

fn find_start(src: char, grid: &Vec<Vec<char>>) -> Pose {
    let orientation = match src {
        'S' => Orientation::East,
        'E' => Orientation::West,
        _ => panic!(),
    };
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == src {
                return Pose::new(Position(i, j), orientation);
            }
        }
    }
    panic!()
}

fn at(pose: &Pose, grid: &Vec<Vec<char>>) -> char {
    grid[pose.position.0][pose.position.1]
}

fn solve(src: char, dst: char, grid: &Vec<Vec<char>>) -> (u32, HashSet<Position>) {
    let start = find_start(src, &grid);

    let mut heap = BinaryHeap::new();
    heap.push(HeapElem(0, start));
    let mut costs = HashMap::new();
    let mut prev: HashMap<Pose, Vec<Pose>> = HashMap::new();

    let mut res = u32::MAX;
    let mut end = None;
    while !heap.is_empty() {
        let HeapElem(weight, pose) = heap.pop().unwrap();
        if at(&pose, &grid) == dst {
            res = res.min(weight);
            if let None = end {
                end = Some(pose);
            }
        }

        if costs.contains_key(&pose) {
            continue;
        }

        costs.insert(pose, weight);

        let rotate_ccw_pose = pose.rotate_ccw().advance();
        if !costs.contains_key(&rotate_ccw_pose) && !(at(&rotate_ccw_pose, grid) == '#') {
            heap.push(HeapElem(weight + 1001, rotate_ccw_pose));
            prev.entry(rotate_ccw_pose).or_default().push(pose);
        }
        let rotate_cw_pose = pose.rotate_cw().advance();
        if !costs.contains_key(&rotate_cw_pose) && !(at(&rotate_cw_pose, grid) == '#') {
            heap.push(HeapElem(weight + 1001, rotate_cw_pose));
            prev.entry(rotate_cw_pose).or_default().push(pose);
        }
        let advance_pose = pose.advance();
        if !costs.contains_key(&advance_pose) && !(at(&advance_pose, grid) == '#') {
            heap.push(HeapElem(weight + 1, advance_pose));
            prev.entry(advance_pose).or_default().push(pose);
        }
    }

    let mut path = HashSet::new();
    let mut dq: VecDeque<Pose> = VecDeque::new();
    dq.push_back(end.unwrap());
    while !dq.is_empty() {
        for _ in 0..dq.len() {
            let node = dq.pop_front().unwrap();
            path.insert(node.position);
            if let Some(prevs) = prev.get(&node) {
                dq.extend(prevs);
            }
        }
    }

    (res, path)
}

fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let grid = reader
        .lines()
        .map(|line| line.unwrap().chars().collect_vec())
        .take_while(|row| !row.is_empty())
        .collect_vec();

    let (_, forward) = solve('S', 'E', &grid);
    let (res, backward) = solve('E', 'S', &grid);

    println!("{:?}", forward.len().min(backward.len()));

    println!("{:?}", res);

    Ok(())
}

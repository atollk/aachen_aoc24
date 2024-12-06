#![allow(dead_code)]

use itertools::Itertools;
use std::collections::HashSet;
use std::fs;

#[derive(Debug, Eq, Hash, PartialEq, Copy, Clone)]
struct Pos(i32, i32);

impl Pos {
    fn mov(self: Pos, to: &Direction) -> Pos {
        match to {
            Direction::Left => Pos(self.0 - 1, self.1),
            Direction::Right => Pos(self.0 + 1, self.1),
            Direction::Up => Pos(self.0, self.1 - 1),
            Direction::Down => Pos(self.0, self.1 + 1),
        }
    }
}

#[derive(Debug)]
struct MapBase {
    width: i32,
    height: i32,
    blockades: HashSet<Pos>,
}

impl MapBase {
    fn is_blocked(self: &MapBase, pos: Pos) -> bool {
        self.blockades.contains(&pos)
    }

    fn is_out_of_map(self: &MapBase, Pos(x, y): Pos) -> bool {
        x < 0 || x >= self.width || y < 0 || y >= self.height
    }
}

#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Direction {
    fn rotate(self: &Direction) -> Direction {
        match self {
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
        }
    }
}

#[derive(Debug, Clone)]
struct MapState {
    guard_position: Pos,
    guard_direction: Direction,
    visited: HashSet<(Pos, Direction)>,
}

impl MapState {
    fn step(self: &mut MapState, map_base: &MapBase) {
        self.visited
            .insert((self.guard_position, self.guard_direction));
        let mut new_pos = self.guard_position.mov(&self.guard_direction);
        while map_base.is_blocked(new_pos) {
            self.guard_direction = self.guard_direction.rotate();
            new_pos = self.guard_position.mov(&self.guard_direction);
        }
        self.guard_position = new_pos;
    }
}

fn parse_input(filename: &str) -> (MapBase, MapState) {
    let file_contents = fs::read_to_string(filename).unwrap();
    let mut base = MapBase {
        width: file_contents.lines().next().unwrap().chars().count() as i32,
        height: file_contents.lines().count() as i32,
        blockades: HashSet::new(),
    };
    let mut guard_position = None;
    let mut guard_direction = None;
    for (i, c) in file_contents.chars().filter(|c| *c != '\n').enumerate() {
        let pos = Pos(i as i32 % base.width, i as i32 / base.height);
        match c {
            '.' => {}
            '#' => {
                base.blockades.insert(pos);
            }
            '^' => {
                guard_position = Some(pos);
                guard_direction = Some(Direction::Up);
            }
            _ => {
                unreachable!();
            }
        }
    }
    (
        base,
        MapState {
            guard_position: guard_position.unwrap(),
            guard_direction: guard_direction.unwrap(),
            visited: HashSet::new(),
        },
    )
}

fn star1() {
    let (base, mut state) = parse_input("day6_input.txt");
    while !base.is_out_of_map(state.guard_position) {
        state.step(&base);
    }
    println!(
        "Star 1: {}",
        state
            .visited
            .into_iter()
            .map(|(p, _)| { p })
            .unique()
            .count()
    );
}

fn is_loop(base: &MapBase, mut state: MapState) -> bool {
    loop {
        state.step(&base);
        if base.is_out_of_map(state.guard_position) {
            return false;
        }
        if state
            .visited
            .contains(&(state.guard_position, state.guard_direction))
        {
            return true;
        }
    }
}

fn block_causes_loop(map_base: &mut MapBase, map_state: MapState, pos: Pos) -> bool {
    let is_new_block = !map_base.is_blocked(pos);
    if is_new_block {
        map_base.blockades.insert(pos);
    }
    let result = is_loop(&map_base, map_state);
    if is_new_block {
        map_base.blockades.remove(&pos);
    }
    result
}

fn star2() {
    // build in release mode, or this might take a while
    let (mut base, state) = parse_input("day6_input.txt");
    let mut loop_positions = 0;
    for x in 0..base.width {
        for y in 0..base.height {
            if block_causes_loop(&mut base, state.clone(), Pos(x, y)) {
                loop_positions += 1;
            }
        }
    }

    println!("Star 2: {}", loop_positions);
}

pub(crate) fn main() {
    star1();
    star2();
}

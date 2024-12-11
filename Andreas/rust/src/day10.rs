#![allow(dead_code)]

use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::fs;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Pos(usize, usize);

impl Pos {
    pub(crate) fn mov(&self, direction: Direction) -> Pos {
        match direction {
            Direction::Left => Pos(self.0.wrapping_sub(1), self.1),
            Direction::Right => Pos(self.0.wrapping_add(1), self.1),
            Direction::Up => Pos(self.0, self.1.wrapping_sub(1)),
            Direction::Down => Pos(self.0, self.1.wrapping_add(1)),
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Debug)]
struct Input {
    width: usize,
    height: usize,
    map: Vec<u8>,
}

impl Input {
    fn get(&self, pos: Pos) -> Option<u8> {
        if pos.0 < self.width && pos.1 < self.height {
            Some(self.map[pos.0 + pos.1 * self.height])
        } else {
            None
        }
    }
}

fn parse_input(filename: &str) -> Input {
    let file_contents = fs::read_to_string(filename).unwrap();
    let width = file_contents.lines().next().unwrap().chars().count();
    let height = file_contents.lines().count();
    let heights = file_contents
        .lines()
        .flat_map(|line| line.chars().map(|c| c.to_digit(10).unwrap() as u8))
        .collect();
    Input {
        width,
        height,
        map: heights,
    }
}

fn find_hikes(input: &Input) -> HashMap<Pos, HashSet<Vec<Pos>>> {
    let mut result = HashMap::new();

    fn visit(node: Pos, input: &Input, result: &mut HashMap<Pos, HashSet<Vec<Pos>>>) {
        if result.get(&node).is_some() {
            return;
        }

        let h = input.get(node).unwrap();
        let mut paths = HashSet::new();
        if h == 9 {
            paths.insert(vec![node]);
        } else {
            for dir in [
                Direction::Down,
                Direction::Left,
                Direction::Right,
                Direction::Up,
            ] {
                let next = node.mov(dir);
                if let Some(h_next) = input.get(next) {
                    if h + 1 == h_next {
                        visit(next, input, result);
                        let next_paths = result.get(&next).unwrap();
                        let new_paths = next_paths.iter().map(|path| {
                            let mut new_path = path.clone();
                            new_path.push(node);
                            new_path
                        });
                        paths.extend(new_paths);
                    }
                }
            }
        }

        result.insert(node, paths);
    }

    let zeroes: Vec<_> = (0..input.width)
        .cartesian_product(0..input.height)
        .map(|(x, y)| {
            let p = Pos(x, y);
            (p, input.get(p))
        })
        .filter_map(|(p, h)| if h == Some(0) { Some(p) } else { None })
        .collect();
    for zero_pos in zeroes {
        visit(zero_pos, input, &mut result);
    }

    result
}

fn star1(input: &Input, paths: &HashMap<Pos, HashSet<Vec<Pos>>>) {
    let goals: HashMap<_, _> = paths
        .iter()
        .map(|(&pos, foo)| {
            (
                pos,
                foo.iter().map(|paths| paths[0]).collect::<HashSet<_>>(),
            )
        })
        .collect();

    let zero_goals_sum: usize = goals
        .iter()
        .filter(|(pos, _)| input.get(**pos).unwrap() == 0)
        .map(|(_, goals)| goals.len())
        .sum();

    println!("Star 1: {}", zero_goals_sum);
}

fn star2(input: &Input, paths: &HashMap<Pos, HashSet<Vec<Pos>>>) {
    let zero_paths_sum: usize = paths
        .iter()
        .filter(|(pos, _)| input.get(**pos).unwrap() == 0)
        .map(|(_, paths)| paths.len())
        .sum();

    println!("Star 2: {}", zero_paths_sum);
}

pub(crate) fn main() {
    let input = parse_input("day10_input.txt");

    let paths = find_hikes(&input);

    star1(&input, &paths);
    star2(&input, &paths);
}

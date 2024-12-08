#![allow(dead_code)]

use itertools::Itertools;
use std::collections::hash_set::Iter;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::iter::{Filter, Map, Zip};

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
struct Pos(usize, usize);

#[derive(Debug)]
struct Input {
    width: usize,
    height: usize,
    antennas: HashMap<char, HashSet<Pos>>,
}

fn parse_input(filename: &str) -> Input {
    let file_contents = fs::read_to_string(filename).unwrap();
    let width = file_contents.lines().next().unwrap().len();
    let height = file_contents.lines().count();
    let chars: Vec<_> = file_contents
        .lines()
        .flat_map(|line| line.chars())
        .collect();
    let mut antennas = HashMap::new();
    for (i, &c) in chars.iter().enumerate() {
        let x = i % width;
        let y = i / width;
        if c != '.' {
            let positions = antennas.entry(c).or_insert(HashSet::new());
            positions.insert(Pos(x, y));
        }
    }
    Input {
        width,
        height,
        antennas,
    }
}

fn get_antinode(lhs: Pos, rhs: Pos) -> Pos {
    let x = (rhs.0 * 2).wrapping_sub(lhs.0);
    let y = (rhs.1 * 2).wrapping_sub(lhs.1);
    Pos(x, y)
}

fn antinodes<'a>(antennas: &'a HashSet<Pos>) -> impl Iterator<Item=Pos> + 'a {
    antennas
        .iter()
        .cartesian_product(antennas)
        .filter(|(lhs, rhs)| **lhs != **rhs)
        .map(|(&lhs, &rhs)| get_antinode(lhs, rhs))
}

fn star1(input: &Input) {
    let all_antinodes = input
        .antennas
        .values()
        .flat_map(antinodes)
        .unique()
        .filter(|pos| pos.0 < input.width && pos.1 < input.height);
    println!("Star 1: {}", all_antinodes.count());
}

pub(crate) fn main() {
    let input = parse_input("day8_input.txt");
    star1(&input);
}

#![allow(dead_code)]

use gcd::Gcd;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::fs;

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
struct Pos(i32, i32);

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
            positions.insert(Pos(x as i32, y as i32));
        }
    }
    Input {
        width,
        height,
        antennas,
    }
}

fn get_antinode1(lhs: Pos, rhs: Pos) -> Pos {
    let x = (rhs.0 * 2).wrapping_sub(lhs.0);
    let y = (rhs.1 * 2).wrapping_sub(lhs.1);
    Pos(x, y)
}

fn antinodes1<'a>(antennas: &'a HashSet<Pos>) -> impl Iterator<Item=Pos> + 'a {
    antennas
        .iter()
        .cartesian_product(antennas)
        .filter(|(lhs, rhs)| **lhs != **rhs)
        .map(|(&lhs, &rhs)| get_antinode1(lhs, rhs))
}

fn star1(input: &Input) {
    let all_antinodes = input
        .antennas
        .values()
        .flat_map(antinodes1)
        .unique()
        .filter(|pos| {
            0 <= pos.0 && pos.0 < input.width as i32 && 0 <= pos.1 && pos.1 < input.height as i32
        });
    println!("Star 1: {}", all_antinodes.count());
}

fn get_antinodes2(lhs: Pos, rhs: Pos, width: i32, height: i32) -> impl Iterator<Item=Pos> {
    let dx = lhs.0 - rhs.0;
    let dy = lhs.1 - rhs.1;
    let gcd = dx.unsigned_abs().gcd(dy.unsigned_abs());
    let dx = dx / gcd as i32;
    let dy = dy / gcd as i32;
    let is_on_map = move |pos: &Pos| 0 <= pos.0 && pos.0 < width && 0 <= pos.1 && pos.1 < height;
    let pos_line = (0..)
        .map(move |i| Pos(lhs.0 + dx * i, lhs.1 + dy * i))
        .take_while(is_on_map);
    let neg_line = (1..)
        .map(move |i| Pos(lhs.0 - dx * i, lhs.1 - dy * i))
        .take_while(is_on_map);
    pos_line.chain(neg_line)
}

fn antinodes2<'a>(
    antennas: &'a HashSet<Pos>,
    width: i32,
    height: i32,
) -> impl Iterator<Item=Pos> + 'a {
    antennas
        .iter()
        .cartesian_product(antennas)
        .filter(|(lhs, rhs)| **lhs != **rhs)
        .flat_map(move |(&lhs, &rhs)| get_antinodes2(lhs, rhs, width, height))
}

fn star2(input: &Input) {
    let all_antinodes = input
        .antennas
        .values()
        .flat_map(|antennas| antinodes2(antennas, input.width as i32, input.height as i32))
        .unique();
    println!("Star 2: {}", all_antinodes.count());
}

pub(crate) fn main() {
    let input = parse_input("day8_input.txt");
    star1(&input);
    star2(&input);
}

#![allow(dead_code)]

use std::collections::{HashMap, HashSet};
use std::fs;

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
    let chars: Vec<_> = file_contents.lines().flat_map(|line| {
        line.chars()
    }).collect();
    let mut antennas = HashMap::new();
    for (i, &c) in chars.iter().enumerate() {
        let x = i % width;
        let y = i / width;
        if c != '.' {
            let positions = antennas.entry(c).or_insert(HashSet::new());
            positions.insert(Pos(x, y));
        }
    }
    Input { width, height, antennas }
}

pub(crate) fn main() {
    let input = parse_input("day8_input.txt");
    println!("{:?}", input);
}
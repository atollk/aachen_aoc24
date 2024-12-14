#![allow(dead_code)]

use itertools::Itertools;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::fs;

const WIDTH: u32 = 101;
const HEIGHT: u32 = 103;

#[derive(Debug)]
struct Robot {
    start: (u32, u32),
    velocity: (i32, i32),
}

fn parse_input(filename: &str) -> Vec<Robot> {
    let file_contents = fs::read_to_string(filename).unwrap();
    let regex = Regex::new(r"p=(\d+),(\d+) v=(-?\d+),(-?\d+)").unwrap();
    file_contents
        .lines()
        .map(|line| {
            let captures = regex.captures(line).unwrap();
            Robot {
                start: (
                    captures.get(1).unwrap().as_str().parse().unwrap(),
                    captures.get(2).unwrap().as_str().parse().unwrap(),
                ),
                velocity: (
                    captures.get(3).unwrap().as_str().parse().unwrap(),
                    captures.get(4).unwrap().as_str().parse().unwrap(),
                ),
            }
        })
        .collect()
}

fn pretty_print(positions: &[(u32, u32)]) -> String {
    let mut result = String::new();
    let positions_map = {
        let mut m = HashMap::new();
        for c in positions {
            *m.entry(c).or_insert(0) += 1;
        }
        m
    };
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let count = *positions_map.get(&(x, y)).unwrap_or(&0);
            let c = if count == 0 {
                '.'
            } else {
                char::from_digit(count, 36).unwrap()
            };
            result.push(c);
        }
        result += "\n";
    }
    result
}

fn simulate_seconds(robots: &[Robot], seconds: u32) -> Vec<(u32, u32)> {
    robots
        .iter()
        .map(|robot| {
            (
                (robot.start.0 + robot.velocity.0 as u32 * seconds) % WIDTH,
                (robot.start.1 + robot.velocity.1 as u32 * seconds) % HEIGHT,
            )
        })
        .collect()
}

fn count_quadrants(positions: &[(u32, u32)]) -> [u32; 4] {
    let mut quadrants = [0; 4];
    for &(x, y) in positions {
        let mut i = 0;
        if x == WIDTH / 2 || y == HEIGHT / 2 {
            continue;
        }
        if x > WIDTH / 2 {
            i += 1;
        }
        if y > HEIGHT / 2 {
            i += 2;
        }
        quadrants[i] += 1;
    }
    quadrants
}

fn search_for_christmas_egg(robots: &[Robot]) {
    fn connectivity(positions: &[(u32, u32)]) -> u32 {
        let positions_set: HashSet<_> = positions.iter().collect();
        let mut result = 0;
        for x in 0..WIDTH {
            for y in 0..HEIGHT {
                let neighbours = [
                    (x + 1, y),
                    (x.wrapping_sub(1), y),
                    (x, y + 1),
                    (x, y.wrapping_sub(1)),
                ];
                result += neighbours
                    .iter()
                    .filter(|p| positions_set.contains(p))
                    .count()
                    .pow(2);
            }
        }
        result as u32
    }

    for i in 0..10000 {
        let pos = simulate_seconds(robots, i);
        let c = connectivity(&pos);
        if c > 2500 {
            println!("{} {}", i, c);
        }
    }
}

pub(crate) fn main() {
    let mut input = parse_input("day14_input.txt");
    for robot in input.iter_mut() {
        while robot.velocity.0 < 0 {
            robot.velocity.0 += WIDTH as i32;
        }
        while robot.velocity.1 < 0 {
            robot.velocity.1 += HEIGHT as i32;
        }
    }

    let positions_after_100_sec = simulate_seconds(&input, 100);
    println!(
        "Star 1: {}",
        count_quadrants(&positions_after_100_sec)
            .iter()
            .product::<u32>()
    );

    for i in [8179] {
        println!("{}", pretty_print(&simulate_seconds(&input, i)));
    }
}

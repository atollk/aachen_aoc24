use ::std::fs::File;
use std::{
    collections::HashMap,
    collections::HashSet,
    collections::VecDeque,
    io::{self, prelude::*, BufReader},
};

use itertools::Itertools;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Cell {
    Regular,
    Wall,
    Start,
    Finish,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn step(
    i: usize,
    j: usize,
    rows: usize,
    cols: usize,
    direction: &Direction,
) -> Option<(usize, usize)> {
    match direction {
        Direction::Up => {
            if i > 0 {
                Some((i - 1, j))
            } else {
                None
            }
        }
        Direction::Down => {
            if i < rows - 1 {
                Some((i + 1, j))
            } else {
                None
            }
        }
        Direction::Left => {
            if j > 0 {
                Some((i, j - 1))
            } else {
                None
            }
        }
        Direction::Right => {
            if j < cols - 1 {
                Some((i, j + 1))
            } else {
                None
            }
        }
    }
}

fn find_start(grid: &Vec<Vec<Cell>>) -> (usize, usize) {
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] == Cell::Start {
                return (i, j);
            }
        }
    }
    panic!()
}

fn bfs(grid: &Vec<Vec<Cell>>) -> Vec<(usize, usize)> {
    let start = find_start(grid);
    let (rows, cols) = (grid.len(), grid[0].len());
    let mut dq = VecDeque::new();
    let mut visited = HashSet::new();
    let mut path = Vec::new();
    dq.push_back(start);
    visited.insert(start);
    path.push(start);

    while !dq.is_empty() {
        for _ in 0..dq.len() {
            let (i, j) = dq.pop_front().unwrap();

            if grid[i][j] == Cell::Finish {
                return path;
            };

            for direction in vec![
                Direction::Up,
                Direction::Down,
                Direction::Left,
                Direction::Right,
            ] {
                if let Some(pos) = step(i, j, rows, cols, &direction) {
                    if grid[pos.0][pos.1] != Cell::Wall && !visited.contains(&pos) {
                        visited.insert(pos);
                        path.push(pos);
                        dq.push_back(pos);
                    }
                }
            }
        }
    }
    path
}

fn get_cheats(path: &Vec<(usize, usize)>, cheat_duration: usize) -> HashMap<usize, usize> {
    let mut cheats = HashMap::new();
    for i in 0..path.len() {
        let from = path[i];
        for j in i + 1..path.len() {
            let to = path[j];
            let dist = from.0.abs_diff(to.0) + from.1.abs_diff(to.1);
            let steps = j - i;
            if steps > dist && dist <= cheat_duration {
                let save = j - i - dist;
                *cheats.entry(save).or_default() += 1;
            }
        }
    }
    cheats
}

fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let grid = BufReader::new(file)
        .lines()
        .map(|line| {
            line.unwrap()
                .chars()
                .map(|cell| match cell {
                    '.' => Cell::Regular,
                    '#' => Cell::Wall,
                    'S' => Cell::Start,
                    'E' => Cell::Finish,
                    _ => panic!(),
                })
                .collect_vec()
        })
        .collect_vec();

    let path = bfs(&grid);
    let cheats = get_cheats(&path, 20);
    let res = cheats
        .iter()
        .fold(0, |acc, (k, v)| if *k >= 100 { acc + v } else { acc });
    println!("{:?}", res);

    Ok(())
}

use ::std::fs::File;
use std::collections::{HashSet, VecDeque};
use std::io::{self, prelude::*, BufReader};

use itertools::Itertools;

const SIZE: usize = 71;

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
enum Cell {
    Free,
    Blocked,
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn step(i: usize, j: usize, direction: &Direction) -> Option<(usize, usize)> {
    match direction {
        Direction::Up => {
            if i > 0 {
                Some((i - 1, j))
            } else {
                None
            }
        }
        Direction::Down => {
            if i < SIZE - 1 {
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
            if j < SIZE - 1 {
                Some((i, j + 1))
            } else {
                None
            }
        }
    }
}

fn bfs(start: (usize, usize), grid: &Vec<Vec<Cell>>) -> Option<u32> {
    let mut visited = HashSet::new();
    let mut dq = VecDeque::new();
    let mut steps = 0;
    visited.insert(start);
    dq.push_back(start);

    while !dq.is_empty() {
        for _ in 0..dq.len() {
            let (i, j) = dq.pop_front().unwrap();

            if i == SIZE - 1 && j == SIZE - 1 {
                return Some(steps);
            }

            for direction in vec![
                Direction::Up,
                Direction::Down,
                Direction::Left,
                Direction::Right,
            ] {
                if let Some((p, q)) = step(i, j, &direction) {
                    if !visited.contains(&(p, q)) && grid[p][q] != Cell::Blocked {
                        visited.insert((p, q));
                        dq.push_back((p, q));
                    }
                }
            }
        }
        steps += 1;
    }
    None
}

fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut grid = vec![vec![Cell::Free; SIZE]; SIZE];

    let blocked = reader.lines().fold(Vec::new(), |mut acc, line| {
        acc.push(
            line.unwrap()
                .split(",")
                .collect_tuple()
                .map(|(j, i)| (i.parse::<usize>().unwrap(), j.parse::<usize>().unwrap()))
                .unwrap(),
        );
        acc
    });

    blocked
        .iter()
        .take(1024)
        .for_each(|(i, j)| grid[*i][*j] = Cell::Blocked);
    let res1 = bfs((0, 0), &grid);
    println!("{:?}", res1);

    let mut it = blocked.iter().skip(1024);
    let mut res2 = (0, 0);
    while let Some(_) = bfs((0, 0), &grid) {
        if let Some((i, j)) = it.next() {
            res2 = (*i, *j);
            grid[*i][*j] = Cell::Blocked;
        }
    }

    println!("{:?}", res2);

    Ok(())
}

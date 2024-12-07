use std::collections::HashSet;
use std::fmt::write;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

use itertools::Itertools;

#[derive(Debug, PartialEq, Eq, Clone)]
enum Path {
    Up(usize, usize),
    Down(usize, usize),
    Left(usize, usize),
    Right(usize, usize),
    Cycle,
    OutOfBounds,
}

impl Path {
    fn hash(&self) -> (usize, usize, usize) {
        match self {
            Path::Up(i, j) => (0, *i, *j),
            Path::Down(i, j) => (1, *i, *j),
            Path::Left(i, j) => (2, *i, *j),
            Path::Right(i, j) => (3, *i, *j),
            Path::Cycle => panic!("cycle in hash"),
            Path::OutOfBounds => panic!("oob in hash"),
        }
    }
}

fn make_move(
    p: &Path,
    grid: &Vec<Vec<char>>,
    visited: &mut HashSet<(usize, usize, usize)>,
) -> Path {
    match p {
        Path::Up(mut i, mut j) => {
            while i > 0 && grid[i - 1][j] != '#' {
                i -= 1;
                if visited.contains(&(0, i, j)) {
                    return Path::Cycle;
                }
                visited.insert((0, i, j));
            }
            if i == 0 {
                Path::OutOfBounds
            } else {
                Path::Right(i, j)
            }
        }
        Path::Down(mut i, mut j) => {
            while i < grid.len() - 1 && grid[i + 1][j] != '#' {
                i += 1;
                if visited.contains(&(1, i, j)) {
                    return Path::Cycle;
                }
                visited.insert((1, i, j));
            }
            if i == grid.len() - 1 {
                Path::OutOfBounds
            } else {
                Path::Left(i, j)
            }
        }
        Path::Left(mut i, mut j) => {
            while j > 0 && grid[i][j - 1] != '#' {
                j -= 1;
                if visited.contains(&(2, i, j)) {
                    return Path::Cycle;
                }
                visited.insert((2, i, j));
            }
            if j == 0 {
                Path::OutOfBounds
            } else {
                Path::Up(i, j)
            }
        }
        Path::Right(mut i, mut j) => {
            while j < grid[0].len() - 1 && grid[i][j + 1] != '#' {
                j += 1;
                if visited.contains(&(3, i, j)) {
                    return Path::Cycle;
                }
                visited.insert((3, i, j));
            }
            if j == grid[0].len() - 1 {
                Path::OutOfBounds
            } else {
                Path::Down(i, j)
            }
        }
        Path::Cycle => panic!("cycle"),
        Path::OutOfBounds => panic!("oob"),
    }
}

fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut grid = reader
        .lines()
        .map(|line| line.unwrap().chars().collect_vec())
        .collect_vec();

    let mut start = Path::Up(0, 0);
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == '^' {
                start = Path::Up(i, j);
            }
        }
    }

    // Part two:
    // How do we detect a loop?
    // For each field on the original path, check if an obstacle at this position would lead us
    // back to an already visited position.
    // We explore into that turned direction until we hit a visited path of same direction or an
    // obstacle.

    println!("{:?}", start);
    println!("{:?}", grid);

    let mut visited = HashSet::new();
    let mut pos = start.clone();
    visited.insert(pos.hash());

    while pos != Path::OutOfBounds {
        pos = make_move(&pos, &grid, &mut visited);
    }

    let mut res = 0;
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            let mut new_visited = HashSet::new();
            let prev = grid[i][j];
            grid[i][j] = '#';
            pos = start.clone();
            new_visited.insert(pos.hash());
            while pos != Path::OutOfBounds && pos != Path::Cycle {
                pos = make_move(&pos, &grid, &mut new_visited);
            }
            if pos == Path::Cycle {
                res += 1;
            }
            grid[i][j] = prev;
        }
    }

    println!("{:?}", res);

    Ok(())
}

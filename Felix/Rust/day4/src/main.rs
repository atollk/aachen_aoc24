use std::env;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

use itertools::Itertools;

fn north(i: usize, j: usize, grid: &Vec<Vec<char>>) -> bool {
    if i < 3 {
        return false;
    }
    let lookup = vec!['X', 'M', 'A', 'S'];
    for k in 1..4 {
        if grid[i - k][j] != lookup[k] {
            return false;
        }
    }
    return true;
}

fn south(i: usize, j: usize, grid: &Vec<Vec<char>>) -> bool {
    if i > grid.len() - 4 {
        return false;
    }
    let lookup = vec!['X', 'M', 'A', 'S'];
    for k in 1..4 {
        if grid[i + k][j] != lookup[k] {
            return false;
        }
    }
    return true;
}

fn east(i: usize, j: usize, grid: &Vec<Vec<char>>) -> bool {
    if j > grid[0].len() - 4 {
        return false;
    }
    let lookup = vec!['X', 'M', 'A', 'S'];
    for k in 1..4 {
        if grid[i][j + k] != lookup[k] {
            return false;
        }
    }
    return true;
}

fn west(i: usize, j: usize, grid: &Vec<Vec<char>>) -> bool {
    if j < 3 {
        return false;
    }
    let lookup = vec!['X', 'M', 'A', 'S'];
    for k in 1..4 {
        if grid[i][j - k] != lookup[k] {
            return false;
        }
    }
    return true;
}

fn northeast(i: usize, j: usize, grid: &Vec<Vec<char>>) -> bool {
    if i < 3 || j > grid[0].len() - 4 {
        return false;
    }
    let lookup = vec!['X', 'M', 'A', 'S'];
    for k in 1..4 {
        if grid[i - k][j + k] != lookup[k] {
            return false;
        }
    }
    return true;
}

fn northwest(i: usize, j: usize, grid: &Vec<Vec<char>>) -> bool {
    if i < 3 || j < 3 {
        return false;
    }
    let lookup = vec!['X', 'M', 'A', 'S'];
    for k in 1..4 {
        if grid[i - k][j - k] != lookup[k] {
            return false;
        }
    }
    return true;
}

fn southeast(i: usize, j: usize, grid: &Vec<Vec<char>>) -> bool {
    if i > grid.len() - 4 || j > grid[0].len() - 4 {
        return false;
    }
    let lookup = vec!['X', 'M', 'A', 'S'];
    for k in 1..4 {
        if grid[i + k][j + k] != lookup[k] {
            return false;
        }
    }
    return true;
}

fn southwest(i: usize, j: usize, grid: &Vec<Vec<char>>) -> bool {
    if i > grid.len() - 4 || j < 3 {
        return false;
    }
    let lookup = vec!['X', 'M', 'A', 'S'];
    for k in 1..4 {
        if grid[i + k][j - k] != lookup[k] {
            return false;
        }
    }
    return true;
}

fn check(i: usize, j: usize, grid: &Vec<Vec<char>>) -> u32 {
    if i < 1 || j < 1 || i > grid.len() - 2 || j > grid[0].len() - 2 {
        return 0;
    }

    // NE, SW, NW, SE
    match (
        grid[i - 1][j + 1],
        grid[i + 1][j - 1],
        grid[i - 1][j - 1],
        grid[i + 1][j + 1],
    ) {
        ('M', 'S', 'M', 'S') => 1,
        ('M', 'S', 'S', 'M') => 1,
        ('S', 'M', 'M', 'S') => 1,
        ('S', 'M', 'S', 'M') => 1,
        _ => 0,
    }
}

fn main() -> io::Result<()> {
    env::set_var("RUST_BACKTRACE", "1");
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let grid = reader
        .lines()
        .map(|line| line.unwrap().chars().collect_vec())
        .collect_vec();

    let (m, n) = (grid.len(), grid.first().unwrap().len());

    let mut res = 0;

    for i in 0..m {
        for j in 0..n {
            if grid[i][j] != 'A' {
                continue;
            }
            res += check(i, j, &grid);
        }
    }

    println!("{}", res);

    Ok(())
}

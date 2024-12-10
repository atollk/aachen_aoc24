use std::fs::File;
use std::io::{self, prelude::*, BufReader};

use itertools::Itertools;

fn in_bounds(i: isize, j: isize, grid: &Vec<Vec<u32>>) -> bool {
    i >= 0 && j >= 0 && i < (grid.len() as isize) && j < (grid[0].len() as isize)
}

fn dfs(i: usize, j: usize, curr: u32, grid: &Vec<Vec<u32>>) -> u32 {
    if grid[i][j] == 9 {
        return 1;
    }

    let mut res = 0;
    for (di, dj) in vec![(0, 1), (0, -1), (1, 0), (-1, 0)] {
        let (p, q) = (i as isize + di, j as isize + dj);
        if in_bounds(p, q, &grid) && grid[p as usize][q as usize] == curr + 1 {
            res += dfs(p as usize, q as usize, curr + 1, &grid);
        }
    }
    res
}

fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let grid = reader
        .lines()
        .map(|line| {
            line.unwrap()
                .chars()
                .map(|ch| ch.to_digit(10).unwrap())
                .collect_vec()
        })
        .collect_vec();

    let mut res = 0;
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == 0 {
                res += dfs(i, j, 0, &grid);
            }
        }
    }

    println!("{:?}", res);

    Ok(())
}

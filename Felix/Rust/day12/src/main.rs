use std::collections::HashSet;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

use itertools::Itertools;

#[derive(Debug)]
struct Plot {
    size: u32,
    perimeter: u32,
}

impl Plot {
    fn default() -> Self {
        Plot::new(0, 0)
    }

    fn new(size: u32, perimeter: u32) -> Self {
        Self { size, perimeter }
    }

    fn grow(&mut self, other: Plot) {
        self.size += other.size;
        self.perimeter += other.perimeter;
    }
}

fn step(i: usize, j: usize, di: isize, dj: isize, grid: &Vec<Vec<char>>) -> Option<(usize, usize)> {
    if (i == 0 && di == -1)
        || (j == 0 && dj == -1)
        || (i == grid.len() - 1 && di == 1)
        || (j == grid[0].len() - 1 && dj == 1)
    {
        None
    } else {
        Some(((i as isize + di) as usize, (j as isize + dj) as usize))
    }
}

fn dfs(i: usize, j: usize, grid: &Vec<Vec<char>>, visited: &mut HashSet<(usize, usize)>) -> Plot {
    if visited.contains(&(i, j)) {
        return Plot::default();
    }

    visited.insert((i, j));

    let mut plot = Plot::new(1, 0);
    for (di, dj) in vec![(0, 1), (0, -1), (1, 0), (-1, 0)] {
        if let Some((p, q)) = step(i, j, di, dj, &grid) {
            if grid[i][j] != grid[p][q] {
                plot.grow(Plot::new(0, 1));
            } else {
                plot.grow(dfs(p, q, &grid, visited));
            }
        } else {
            plot.grow(Plot::new(0, 1));
        }
    }
    plot
}

fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let grid = reader
        .lines()
        .map(|line| line.unwrap().chars().collect_vec())
        .collect_vec();

    let mut visited = HashSet::new();
    let mut plots = Vec::new();
    for i in 0..grid.len() {
        for j in 0..grid.len() {
            plots.push(dfs(i, j, &grid, &mut visited));
        }
    }

    let res = plots
        .iter()
        .fold(0, |acc, plot| acc + plot.size * plot.perimeter);

    println!("{:?}", res);

    Ok(())
}

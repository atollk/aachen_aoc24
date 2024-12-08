use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

use itertools::Itertools;

fn in_bounds(pos: (isize, isize), m: usize, n: usize) -> bool {
    pos.0 >= 0 && pos.1 >= 0 && (pos.0 as usize) < m && (pos.1 as usize) < n
}

fn calculate_antinodes(
    lhs: &(usize, usize),
    rhs: &(usize, usize),
    m: usize,
    n: usize,
) -> HashSet<(usize, usize)> {
    // . . o <- lhs
    // . . .
    // o . . <- rhs
    // rhs = (2, 0), lhs = (0, 2)
    // first = (-2, 4), second = (4, -2)
    // slope = (2, -2)
    // first = lhs - slope, second = rhs + slope
    let mut antinodes = HashSet::new();
    let mut lhs = (lhs.0 as isize, lhs.1 as isize);
    let mut rhs = (rhs.0 as isize, rhs.1 as isize);
    let slope = (rhs.0 - lhs.0, rhs.1 - lhs.1);
    while in_bounds(rhs, m, n) {
        antinodes.insert((rhs.0 as usize, rhs.1 as usize));
        rhs = (rhs.0 + slope.0, rhs.1 + slope.1);
    }
    while in_bounds(lhs, m, n) {
        antinodes.insert((lhs.0 as usize, lhs.1 as usize));
        lhs = (lhs.0 - slope.0, lhs.1 - slope.1);
    }
    antinodes
}

fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let grid = reader
        .lines()
        .map(|line| line.unwrap().chars().collect_vec())
        .collect_vec();
    let (m, n) = (grid.len(), grid[0].len());

    let antennas = grid.into_iter().enumerate().fold(
        HashMap::<char, Vec<(usize, usize)>>::new(),
        |acc, (i, row)| {
            row.into_iter().enumerate().fold(acc, |mut a, (j, cell)| {
                if cell != '.' {
                    a.entry(cell).or_default().push((i, j));
                }
                a
            })
        },
    );

    let res = antennas
        .into_values()
        .fold(HashSet::<(usize, usize)>::new(), |acc, ant_list| {
            ant_list.iter().combinations(2).fold(acc, |mut a, pair| {
                a.extend(&calculate_antinodes(pair[0], pair[1], m, n));
                a
            })
        })
        .len();

    println!("{:?}", res);

    Ok(())
}

use std::collections::HashSet;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

use itertools::Itertools;

#[derive(Debug)]
struct Plot {
    size: u32,
    perimeter: u32,
    corners: u32,
}

impl Plot {
    fn default() -> Self {
        Plot::new(0, 0, 0)
    }

    fn new(size: u32, perimeter: u32, corners: u32) -> Self {
        Self {
            size,
            perimeter,
            corners,
        }
    }

    fn grow(&mut self, other: Plot) {
        self.size += other.size;
        self.perimeter += other.perimeter;
        self.corners += other.corners;
    }
}

enum Step {
    Up,
    Left,
    Down,
    Right,
}

struct Cell<'a> {
    i: usize,
    j: usize,
    grid: &'a Vec<Vec<char>>,
}

impl<'a> Cell<'a> {
    fn new(i: usize, j: usize, grid: &'a Vec<Vec<char>>) -> Self {
        Self { i, j, grid }
    }

    fn eval(&self) -> char {
        self.grid[self.i][self.j]
    }

    fn same(&self, other: &Cell) -> bool {
        self.eval() == other.eval()
    }

    fn pos(&self) -> (usize, usize) {
        (self.i, self.j)
    }

    fn step(&self, s: Step) -> Option<Self> {
        match s {
            Step::Up => self.up(),
            Step::Down => self.down(),
            Step::Left => self.left(),
            Step::Right => self.right(),
        }
    }

    fn up(&self) -> Option<Self> {
        if self.i > 0 && self.eval() == self.grid[self.i - 1][self.j] {
            Some(Self {
                i: self.i - 1,
                ..*self
            })
        } else {
            None
        }
    }
    fn down(&self) -> Option<Self> {
        if self.i < self.grid.len() - 1 && self.eval() == self.grid[self.i + 1][self.j] {
            Some(Self {
                i: self.i + 1,
                ..*self
            })
        } else {
            None
        }
    }
    fn left(&self) -> Option<Self> {
        if self.j > 0 && self.eval() == self.grid[self.i][self.j - 1] {
            Some(Self {
                j: self.j - 1,
                ..*self
            })
        } else {
            None
        }
    }
    fn right(&self) -> Option<Self> {
        if self.j < self.grid[0].len() - 1 && self.eval() == self.grid[self.i][self.j + 1] {
            Some(Self {
                j: self.j + 1,
                ..*self
            })
        } else {
            None
        }
    }
    fn up_left(&self) -> Option<Self> {
        self.up().map_or(None, |c| c.left())
    }
    fn up_right(&self) -> Option<Self> {
        self.up().map_or(None, |c| c.right())
    }
    fn down_left(&self) -> Option<Self> {
        self.down().map_or(None, |c| c.left())
    }
    fn down_right(&self) -> Option<Self> {
        self.down().map_or(None, |c| c.right())
    }
}

fn count_corners(cell: &Cell) -> u32 {
    /*     _
     *   _| |_
     *  |_   _|
     *    |_|
     */
    // Case 1: left and up out || right and up out || left and down out || right and down out
    // Case 2: left and up in, leftup out || right and up in, rightup out || left and down in ,
    // leftdown out || right and down in, rightdown out
    // out means not same plot or out of bounds
    let mut res = 0;
    res += match (cell.up(), cell.left(), cell.up_left()) {
        (None, None, _) => 1,
        (Some(_), Some(_), None) => 1,
        _ => 0,
    };
    res += match (cell.up(), cell.right(), cell.up_right()) {
        (None, None, _) => 1,
        (Some(_), Some(_), None) => 1,
        _ => 0,
    };
    res += match (cell.down(), cell.left(), cell.down_left()) {
        (None, None, _) => 1,
        (Some(_), Some(_), None) => 1,
        _ => 0,
    };
    res += match (cell.down(), cell.right(), cell.down_right()) {
        (None, None, _) => 1,
        (Some(_), Some(_), None) => 1,
        _ => 0,
    };
    res
}

fn dfs(cell: &Cell, visited: &mut HashSet<(usize, usize)>) -> Plot {
    if visited.contains(&cell.pos()) {
        return Plot::default();
    }

    visited.insert(cell.pos());

    let mut plot = Plot::new(1, 0, 0);
    plot.grow(Plot::new(0, 0, count_corners(&cell)));
    for s in vec![Step::Up, Step::Down, Step::Left, Step::Right] {
        if let Some(other) = cell.step(s) {
            if !cell.same(&other) {
                plot.grow(Plot::new(0, 1, 0));
            } else {
                plot.grow(dfs(&other, visited));
            }
        } else {
            plot.grow(Plot::new(0, 1, 0));
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
            if !visited.contains(&(i, j)) {
                let cell = Cell::new(i, j, &grid);
                plots.push(dfs(&cell, &mut visited));
            }
        }
    }

    let res = plots
        .iter()
        .fold(0, |acc, plot| acc + plot.size * plot.corners);

    println!("{:?}", res);

    Ok(())
}

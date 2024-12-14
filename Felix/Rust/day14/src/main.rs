use ::std::fs::File;
use std::collections::HashSet;
use std::io::{self, prelude::*, BufReader};

use itertools::Itertools;

#[derive(Debug)]
struct Vec2 {
    x: i32,
    y: i32,
}

impl Vec2 {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

#[derive(Debug)]
struct Robot {
    p: Vec2,
    v: Vec2,
}

enum Quadrant {
    First,
    Second,
    Third,
    Fourth,
    None,
}

impl Robot {
    const M: i32 = 101;
    const N: i32 = 103;

    fn new(p: Vec2, v: Vec2) -> Self {
        Self { p, v }
    }

    fn step(&mut self) {
        self.p.x += self.v.x;
        self.p.y += self.v.y;
        if self.p.x < 0 {
            self.p.x += Self::M;
        }
        if self.p.x >= Self::M {
            self.p.x -= Self::M;
        }
        if self.p.y < 0 {
            self.p.y += Self::N;
        }
        if self.p.y >= Self::N {
            self.p.y -= Self::N;
        }
    }

    fn quadrant(&self) -> Quadrant {
        if self.p.x < Self::M / 2 && self.p.y < Self::N / 2 {
            Quadrant::First
        } else if self.p.x < Self::M / 2 && self.p.y > Self::N / 2 {
            Quadrant::Second
        } else if self.p.x > Self::M / 2 && self.p.y < Self::N / 2 {
            Quadrant::Third
        } else if self.p.x > Self::M / 2 && self.p.y > Self::N / 2 {
            Quadrant::Fourth
        } else {
            Quadrant::None
        }
    }
}

fn dfs(i: usize, j: usize, grid: &Vec<Vec<char>>, visited: &mut HashSet<(usize, usize)>) -> u32 {
    if grid[i][j] == '.' {
        return 0;
    }

    if visited.contains(&(i, j)) {
        return 0;
    }

    visited.insert((i, j));

    let mut res = 1;
    if i > 0 {
        res += dfs(i - 1, j, grid, visited);
    }
    if j > 0 {
        res += dfs(i, j - 1, grid, visited);
    }
    if i < grid.len() - 1 {
        res += dfs(i + 1, j, grid, visited);
    }
    if j < grid[0].len() - 1 {
        res += dfs(i, j + 1, grid, visited);
    }
    res
}

fn build_grid(robots: &Vec<Robot>) -> Vec<Vec<char>> {
    let mut grid = vec![vec!['.'; Robot::M as usize]; Robot::N as usize];
    for robot in robots {
        grid[robot.p.y as usize][robot.p.x as usize] = 'x';
    }
    grid
}

fn max_island(robots: &Vec<Robot>) -> u32 {
    let grid = build_grid(&robots);

    let mut res = 0;
    let mut visited = HashSet::new();
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if !visited.contains(&(i, j)) {
                res = res.max(dfs(i, j, &grid, &mut visited));
            }
        }
    }

    res
}

fn print_grid(robots: &Vec<Robot>) {
    let grid = build_grid(&robots);

    let s = grid
        .iter()
        .map(|line| {
            line.iter().fold("".to_string(), |mut acc, ch| {
                acc.push(*ch);
                acc
            })
        })
        .fold("".to_string(), |acc, s| format!("{}\n{}", acc, s));
    println!("{}", s);
}

fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut robots = reader
        .lines()
        .map(|line| {
            let data = line
                .unwrap()
                .split(['=', ',', ' '])
                .map(|substr| substr.parse::<i32>())
                .flatten()
                .collect_vec();
            Robot::new(Vec2::new(data[0], data[1]), Vec2::new(data[2], data[3]))
        })
        .collect_vec();

    // let mut res = (0, 0);
    for i in 0..7138 {
        for robot in &mut robots {
            robot.step();
        }
        // let island = max_island(&robots);
        // if island > res.1 {
        //     res = (i, island);
        // }
    }
    // println!("{:?}", res);

    print_grid(&robots);

    let (first, second, third, fourth) = robots.iter().fold(
        (0, 0, 0, 0),
        |(first, second, third, fourth), robot| match robot.quadrant() {
            Quadrant::First => (first + 1, second, third, fourth),
            Quadrant::Second => (first, second + 1, third, fourth),
            Quadrant::Third => (first, second, third + 1, fourth),
            Quadrant::Fourth => (first, second, third, fourth + 1),
            Quadrant::None => (first, second, third, fourth),
        },
    );

    let res = first * second * third * fourth;

    println!("{:?}", res);

    Ok(())
}

use ::std::fs::File;
use std::collections::{HashMap, VecDeque};
use std::io::{self, prelude::*, BufReader};

use itertools::Itertools;

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn find_start(grid: &Vec<Vec<char>>) -> (usize, usize) {
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == '@' {
                return (i, j);
            }
        }
    }
    panic!()
}

fn step1(pos: (usize, usize), direction: &Direction, grid: &mut Vec<Vec<char>>) -> (usize, usize) {
    match direction {
        // Find first non obstacle.
        // If wall:
        //   pass
        // Else:
        //   swap(next, found)
        //   swap(curr, next)
        Direction::Up => {
            let mut i = 1;
            while grid[pos.0 - i][pos.1] == 'O' {
                i += 1;
            }
            if grid[pos.0 - i][pos.1] == '#' {
                pos
            } else {
                grid[pos.0 - i][pos.1] = grid[pos.0 - 1][pos.1];
                grid[pos.0 - 1][pos.1] = '@';
                grid[pos.0][pos.1] = '.';
                (pos.0 - 1, pos.1)
            }
        }
        Direction::Down => {
            let mut i = 1;
            while grid[pos.0 + i][pos.1] == 'O' {
                i += 1;
            }
            if grid[pos.0 + i][pos.1] == '#' {
                pos
            } else {
                grid[pos.0 + i][pos.1] = grid[pos.0 + 1][pos.1];
                grid[pos.0 + 1][pos.1] = '@';
                grid[pos.0][pos.1] = '.';
                (pos.0 + 1, pos.1)
            }
        }
        Direction::Left => {
            let mut j = 1;
            while grid[pos.0][pos.1 - j] == 'O' {
                j += 1;
            }
            if grid[pos.0][pos.1 - j] == '#' {
                pos
            } else {
                grid[pos.0][pos.1 - j] = grid[pos.0][pos.1 - 1];
                grid[pos.0][pos.1 - 1] = '@';
                grid[pos.0][pos.1] = '.';
                (pos.0, pos.1 - 1)
            }
        }
        Direction::Right => {
            let mut j = 1;
            while grid[pos.0][pos.1 + j] == 'O' {
                j += 1;
            }
            if grid[pos.0][pos.1 + j] == '#' {
                pos
            } else {
                grid[pos.0][pos.1 + j] = grid[pos.0][pos.1 + 1];
                grid[pos.0][pos.1 + 1] = '@';
                grid[pos.0][pos.1] = '.';
                (pos.0, pos.1 + 1)
            }
        }
    }
}

fn solve1(directions: &Vec<Direction>, mut grid: Vec<Vec<char>>) -> usize {
    let mut pos = find_start(&grid);
    for direction in directions {
        pos = step1(pos, direction, &mut grid);
    }

    let mut res = 0;
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == 'O' {
                res += 100 * i + j;
            }
        }
    }
    res
}

fn enlarge_grid(grid: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    grid.iter()
        .map(|row| {
            row.iter().fold(Vec::new(), |mut acc, cell| match cell {
                '#' => {
                    acc.push('#');
                    acc.push('#');
                    acc
                }
                'O' => {
                    acc.push('[');
                    acc.push(']');
                    acc
                }
                '.' => {
                    acc.push('.');
                    acc.push('.');
                    acc
                }
                '@' => {
                    acc.push('@');
                    acc.push('.');
                    acc
                }
                _ => panic!(),
            })
        })
        .collect_vec()
}

fn eval_vert(
    pos: &(usize, usize),
    grid: &Vec<Vec<char>>,
    f: &dyn Fn(usize) -> usize,
) -> Option<Vec<(usize, usize)>> {
    match (grid[pos.0][pos.1], grid[f(pos.0)][pos.1]) {
        (_, '#') => None,
        (_, '.') => Some(Vec::new()),
        ('@', '[') => Some(vec![(f(pos.0), pos.1), (f(pos.0), pos.1 + 1)]),
        ('@', ']') => Some(vec![(f(pos.0), pos.1), (f(pos.0), pos.1 - 1)]),
        ('[', '[') => Some(vec![(f(pos.0), pos.1)]),
        ('[', ']') => Some(vec![(f(pos.0), pos.1), (f(pos.0), pos.1 - 1)]),
        (']', '[') => Some(vec![(f(pos.0), pos.1), (f(pos.0), pos.1 + 1)]),
        (']', ']') => Some(vec![(f(pos.0), pos.1)]),
        _ => panic!(),
    }
}

fn find_box_cluster(
    pos: &(usize, usize),
    grid: &Vec<Vec<char>>,
    f: &dyn Fn(usize) -> usize,
) -> Option<HashMap<(usize, usize), char>> {
    let mut res = HashMap::new();

    let start = eval_vert(&pos, &grid, f)?;
    let mut dq = VecDeque::from(start);
    while !dq.is_empty() {
        for _ in 0..dq.len() {
            let (i, j) = dq.pop_front().unwrap();
            res.insert((i, j), grid[i][j]);
            let next = eval_vert(&(i, j), &grid, f)?;
            dq.extend(next.iter());
        }
    }

    Some(res)
}

fn step2(pos: (usize, usize), direction: &Direction, grid: &mut Vec<Vec<char>>) -> (usize, usize) {
    match direction {
        Direction::Up => {
            if grid[pos.0 - 1][pos.1] == '[' || grid[pos.0 - 1][pos.1] == ']' {
                if let Some(cluster) = find_box_cluster(&pos, &grid, &|i| i - 1) {
                    for ((i, j), _) in &cluster {
                        grid[*i][*j] = '.';
                    }
                    for ((i, j), v) in &cluster {
                        grid[*i - 1][*j] = *v;
                    }
                    grid[pos.0][pos.1] = '.';
                    grid[pos.0 - 1][pos.1] = '@';
                    (pos.0 - 1, pos.1)
                } else {
                    pos
                }
            } else if grid[pos.0 - 1][pos.1] == '#' {
                pos
            } else {
                grid[pos.0][pos.1] = '.';
                grid[pos.0 - 1][pos.1] = '@';
                (pos.0 - 1, pos.1)
            }
        }
        Direction::Down => {
            if grid[pos.0 + 1][pos.1] == '[' || grid[pos.0 + 1][pos.1] == ']' {
                if let Some(cluster) = find_box_cluster(&pos, &grid, &|i| i + 1) {
                    for ((i, j), _) in &cluster {
                        grid[*i][*j] = '.';
                    }
                    for ((i, j), v) in &cluster {
                        grid[*i + 1][*j] = *v;
                    }
                    grid[pos.0][pos.1] = '.';
                    grid[pos.0 + 1][pos.1] = '@';
                    (pos.0 + 1, pos.1)
                } else {
                    pos
                }
            } else if grid[pos.0 + 1][pos.1] == '#' {
                pos
            } else {
                grid[pos.0][pos.1] = '.';
                grid[pos.0 + 1][pos.1] = '@';
                (pos.0 + 1, pos.1)
            }
        }
        Direction::Left => {
            let mut j = 1;
            while grid[pos.0][pos.1 - j] == '[' || grid[pos.0][pos.1 - j] == ']' {
                j += 1;
            }
            if grid[pos.0][pos.1 - j] == '#' {
                pos
            } else {
                for k in (1..j + 1).rev() {
                    grid[pos.0][pos.1 - k] = grid[pos.0][pos.1 - k + 1];
                }
                grid[pos.0][pos.1 - 1] = '@';
                grid[pos.0][pos.1] = '.';
                (pos.0, pos.1 - 1)
            }
        }
        Direction::Right => {
            let mut j = 1;
            while grid[pos.0][pos.1 + j] == '[' || grid[pos.0][pos.1 + j] == ']' {
                j += 1;
            }
            if grid[pos.0][pos.1 + j] == '#' {
                pos
            } else {
                for k in (1..j + 1).rev() {
                    grid[pos.0][pos.1 + k] = grid[pos.0][pos.1 + k - 1];
                }
                grid[pos.0][pos.1 + 1] = '@';
                grid[pos.0][pos.1] = '.';
                (pos.0, pos.1 + 1)
            }
        }
    }
}

fn print_grid(grid: &Vec<Vec<char>>) {
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

fn solve2(directions: &Vec<Direction>, grid: &Vec<Vec<char>>) -> usize {
    let mut grid = enlarge_grid(&grid);

    let mut pos = find_start(&grid);
    for direction in directions {
        pos = step2(pos, direction, &mut grid);
        // print_grid(&grid);
    }

    let mut res = 0;
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == '[' {
                res += 100 * i + j;
            }
        }
    }
    res
}

fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let grid = reader
        .lines()
        .map(|line| line.unwrap().chars().collect_vec())
        .take_while(|row| !row.is_empty())
        .collect_vec();

    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let directions = reader
        .lines()
        .map(|line| line.unwrap())
        .skip_while(|line| !line.starts_with(['<', '^', 'v', '>']))
        .fold(Vec::new(), |mut acc, line| {
            let next_ds = line
                .chars()
                .filter(|d| d != &' ')
                .map(|d| match d {
                    '^' => Direction::Up,
                    'v' => Direction::Down,
                    '<' => Direction::Left,
                    '>' => Direction::Right,
                    _ => panic!(),
                })
                .collect_vec();
            acc.extend(next_ds);
            acc
        });

    println!("{:?}", solve1(&directions, grid.clone()));

    println!("{:?}", solve2(&directions, &grid));

    Ok(())
}

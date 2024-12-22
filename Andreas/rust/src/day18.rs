#![allow(dead_code)]

use crate::grid::{Direction, Grid, Position};
use itertools::Itertools;
use std::collections::{HashSet, VecDeque};
use std::fs;

const GRID_WIDTH: u32 = 71;
const GRID_HEIGHT: u32 = 71;

fn parse_input(filename: &str) -> Vec<Position> {
    let file_contents = fs::read_to_string(filename).unwrap();
    file_contents
        .lines()
        .map(|line| {
            let (x, y) = line.split(",").collect_tuple().unwrap();
            Position {
                x: x.parse().unwrap(),
                y: y.parse().unwrap(),
            }
        })
        .collect()
}

fn star1(corrupted_bytes: &[Position]) {
    let grid_corrupted = Grid {
        width: GRID_WIDTH,
        height: GRID_HEIGHT,
        entities: (0..GRID_WIDTH)
            .flat_map(|x| {
                (0..GRID_HEIGHT).map(move |y| corrupted_bytes.contains(&Position { x, y }))
            })
            .collect(),
    };
    let mut grid_min_distance = Grid {
        width: GRID_WIDTH,
        height: GRID_HEIGHT,
        entities: (0..GRID_WIDTH)
            .flat_map(|x| {
                (0..GRID_HEIGHT).map(move |y| if x == 0 && y == 0 { 0 } else { u32::MAX })
            })
            .collect(),
    };
    let mut grid_predecessor = Grid {
        width: GRID_WIDTH,
        height: GRID_HEIGHT,
        entities: (0..GRID_WIDTH)
            .flat_map(|_| (0..GRID_HEIGHT).map(|_| Direction::Up))
            .collect(),
    };

    let starting_pos = Position { x: 0, y: 0 };
    let goal_pos = Position {
        x: GRID_WIDTH - 1,
        y: GRID_HEIGHT - 1,
    };
    let mut nodes_to_visit = VecDeque::from([starting_pos]);
    let mut visited_nodes = HashSet::new();

    while let Some(node) = nodes_to_visit.pop_back() {
        if node == goal_pos {
            break;
        }
        if visited_nodes.get(&node).is_some() {
            continue;
        }
        visited_nodes.insert(node);
        for dir in Direction::all() {
            let next_node = node.move_to(dir);
            if let Some(next_node_distance) = grid_min_distance.get(next_node) {
                nodes_to_visit.push_front(next_node);
                if *next_node_distance > grid_min_distance.get(node).unwrap() + 1 {
                    grid_min_distance.set(next_node, grid_min_distance.get(node).unwrap() + 1);
                    grid_predecessor.set(next_node, dir);
                }
            }
        }
    }

    let shortest_path: HashSet<_> = {
        let mut path = vec![goal_pos];
        while *path.last().unwrap() != starting_pos {
            println!("{:?}", path);
            let pos = *path.last().unwrap();
            path.push(pos.move_to(*grid_predecessor.get(pos).expect(&format!("{:?}", pos))));
        }
        path
    }
        .into_iter()
        .collect();
    println!(
        "{}",
        grid_corrupted.pretty_print(|position, corrupted| {
            if *corrupted {
                '#'
            } else {
                if shortest_path.contains(&position) {
                    char::from(*grid_predecessor.get(position).unwrap())
                } else {
                    '.'
                }
            }
        })
    );
    //println!("Star 1: {}", path_length);
}

pub(crate) fn main() {
    let input = parse_input("day18_input.txt");

    star1(&input[..1024]);
}

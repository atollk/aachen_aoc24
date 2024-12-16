#![allow(dead_code)]

use std::cmp::min;
use crate::grid::{Direction, Grid, Position};
use itertools::Itertools;
use std::collections::HashMap;
use std::fs;
use std::fs::canonicalize;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
enum Tile {
    Empty,
    Wall,
    Start,
    End,
}

fn parse_input(filename: &str) -> Grid<Tile> {
    let file_contents = fs::read_to_string(filename).unwrap();
    Grid::parse_from_string(&file_contents, |c| match c {
        '.' => Tile::Empty,
        '#' => Tile::Wall,
        'S' => Tile::Start,
        'E' => Tile::End,
        _ => unreachable!(),
    })
        .unwrap()
}

fn print_path_from_costs(grid: &Grid<Tile>, costs: &HashMap<(Position, Direction), Option<u32>>) {
    for y in 0..grid.height {
        for x in 0..grid.width {
            let pos = Position { x, y };
            let tile = *grid.get(pos).unwrap();
            let c = if tile == Tile::Wall {
                '#'
            } else {
                let best_direction = Direction::all()
                    .iter()
                    .map(|&d| {
                        (
                            d,
                            costs
                                .get(&(pos.move_to(d), d))
                                .unwrap_or(&None)
                                .unwrap_or(u32::MAX),
                        )
                    })
                    .min_by_key(|(_, cost)| *cost)
                    .unwrap();
                if best_direction.1 == u32::MAX {
                    '.'
                } else {
                    char::from(best_direction.0)
                }
            };
            print!("{}", c);
        }
        println!();
    }
}

fn find_shortest_path(grid: &Grid<Tile>) -> u32 {
    let start = grid.find(&Tile::Start).exactly_one().ok().unwrap();
    let end = grid.find(&Tile::End).exactly_one().ok().unwrap();

    let mut costs = HashMap::new();
    let mut prevs = HashMap::new();
    let mut nodes_to_visit = vec![(start, Direction::Right)];
    costs.insert((start, Direction::Right), 0);

    while !nodes_to_visit.is_empty() {
        let node = nodes_to_visit.remove(
            nodes_to_visit
                .iter()
                .position_min_by_key(|x| costs.get(*x).unwrap_or(&u32::MAX))
                .unwrap(),
        );
        let forward_node = (node.0.move_to(node.1), node.1);
        let clockwise_node = (node.0, node.1.turn90());
        let counter_clockwise_node = (node.0, node.1.turn270());
        let neighbours = if *grid.get(forward_node.0).unwrap_or(&Tile::Wall) == Tile::Wall {
            vec![(clockwise_node, 1000), (counter_clockwise_node, 1000)]
        } else {
            vec![
                (forward_node, 1),
                (clockwise_node, 1000),
                (counter_clockwise_node, 1000),
            ]
        };
        for (next_node, added_cost) in neighbours {
            if !costs.contains_key(&next_node) {
                nodes_to_visit.push(next_node);
            }
            let next_potential = costs[&node].saturating_add(added_cost);
            let current_best = costs.entry(next_node).or_insert(u32::MAX);
            if next_potential <= *current_best {
                prevs.entry(next_node).or_insert(Vec::new()).push(node);
                *current_best = next_potential;
            }
        }
    }

    let min_cost = Direction::all()
        .map(|dir| *costs.get(&(end, dir)).unwrap_or(&u32::MAX))
        .into_iter()
        .min()
        .unwrap();
    min_cost
}

pub(crate) fn main() {
    let input = parse_input("day16_input.txt");
    let shortest_path = find_shortest_path(&input);
    println!("{:?}", shortest_path);
}

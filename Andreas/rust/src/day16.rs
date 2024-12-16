#![allow(dead_code)]

use crate::grid::{Direction, Grid, Position};
use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::fs;

type PathNode = (Position, Direction);

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
enum Tile {
    Empty,
    Wall,
    Start,
    End,
}

impl From<Tile> for char {
    fn from(value: Tile) -> Self {
        match value {
            Tile::Empty => '.',
            Tile::Wall => '#',
            Tile::Start => 'S',
            Tile::End => 'E',
        }
    }
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

fn find_shortest_path(grid: &Grid<Tile>) -> (HashMap<PathNode, Vec<PathNode>>, u32) {
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
    (prevs, min_cost)
}

fn path_cost(path: &[PathNode]) -> u32 {
    path.windows(2)
        .map(|window| {
            let (&a, &b) = window.iter().collect_tuple().unwrap();
            if a.0 == b.0 {
                1000
            } else {
                1
            }
        })
        .sum()
}

fn collect_best_paths(
    grid: &Grid<Tile>,
    prevs: &HashMap<PathNode, Vec<PathNode>>,
    min_cost: u32,
) -> HashSet<Vec<PathNode>> {
    fn collect_from(
        node: PathNode,
        prevs: &HashMap<PathNode, Vec<PathNode>>,
    ) -> Vec<Vec<PathNode>> {
        if let Some(previous_nodes) = prevs.get(&node) {
            previous_nodes
                .iter()
                .flat_map(|prev_node| {
                    collect_from(*prev_node, prevs).into_iter().map(|mut path| {
                        path.push(node);
                        path
                    })
                })
                .collect()
        } else {
            vec![vec![node]]
        }
    }

    let end = grid.find(&Tile::End).exactly_one().ok().unwrap();
    let mut result = HashSet::new();
    for dir in Direction::all() {
        result.extend(collect_from((end, dir), prevs));
    }
    result
        .into_iter()
        .filter(|path| path_cost(path) == min_cost)
        .collect()
}

pub(crate) fn main() {
    let input = parse_input("day16_input.txt");
    let (prevs, shortest_path) = find_shortest_path(&input);
    println!("Star 1: {:?}", shortest_path);

    let best_paths = collect_best_paths(&input, &prevs, shortest_path);
    let on_a_best_path: HashSet<_> = best_paths
        .iter()
        .flat_map(|p| p)
        .map(|node| node.0)
        .collect();
    println!("Star 2: {}", on_a_best_path.len());
}

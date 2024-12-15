use crate::grid::{Direction, Grid};
use itertools::Itertools;
use std::collections::HashMap;
use std::fs;

#[derive(Debug)]
struct Input {
    start_grid: Grid<()>,
    move_plan: Vec<Direction>,
}

fn parse_input(filename: &str) -> Input {
    let file_contents = fs::read_to_string(filename).unwrap();
    let (grid_input, plan_input) = file_contents.split("\n\n").collect_tuple().unwrap();

    let start_grid = Grid {
        width: grid_input.lines().next().unwrap().chars().count() as u32,
        height: grid_input.lines().count() as u32,
        entities: HashMap::new(),
    };

    let move_plan = Vec::new();

    Input {
        start_grid,
        move_plan,
    }
}

pub(crate) fn main() {
    let input = parse_input("day15_input.txt");
    println!("{:?}", input);
}

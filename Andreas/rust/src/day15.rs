use crate::grid::{Direction, Grid};
use itertools::Itertools;
use std::cmp::PartialEq;
use std::fs;

#[derive(Debug, Eq, PartialEq, Clone)]
enum GridTile {
    Empty,
    Box,
    Wall,
    Robot,
}

#[derive(Debug)]
struct Input {
    start_grid: Grid<GridTile>,
    move_plan: Vec<Direction>,
}

fn parse_input(filename: &str) -> Input {
    let file_contents = fs::read_to_string(filename).unwrap();
    let (grid_input, plan_input) = file_contents.split("\n\n").collect_tuple().unwrap();

    let start_grid = Grid {
        width: grid_input.lines().next().unwrap().chars().count() as u32,
        height: grid_input.lines().count() as u32,
        entities: grid_input
            .chars()
            .filter(|c| !c.is_whitespace())
            .map(|c| match c {
                '.' => GridTile::Empty,
                '#' => GridTile::Wall,
                'O' => GridTile::Box,
                '@' => GridTile::Robot,
                _ => unreachable!(),
            })
            .collect(),
    };

    let move_plan = plan_input
        .chars()
        .filter(|c| !c.is_whitespace())
        .map(|c| match c {
            '<' => Direction::Left,
            '>' => Direction::Right,
            '^' => Direction::Up,
            'v' => Direction::Down,
            _ => unreachable!(),
        })
        .collect();

    Input {
        start_grid,
        move_plan,
    }
}

fn pp_tile(grid_tile: &GridTile) -> char {
    match grid_tile {
        GridTile::Empty => '.',
        GridTile::Box => 'O',
        GridTile::Wall => '#',
        GridTile::Robot => '@',
    }
}

fn apply_move(grid: &mut Grid<GridTile>, direction: Direction) {
    let robot_pos = grid.find(&GridTile::Robot).exactly_one().ok().unwrap();
    let mut steps = 0;
    while [GridTile::Robot, GridTile::Box]
        .contains(grid.get(robot_pos.move_by(direction, steps)).unwrap())
    {
        steps += 1;
    }
    if *grid.get(robot_pos.move_by(direction, steps)).unwrap() == GridTile::Wall {
        // Blocked by wall
        return;
    }
    for i in 2..=steps {
        grid.set(robot_pos.move_by(direction, i), GridTile::Box);
    }
    grid.set(robot_pos.move_by(direction, 1), GridTile::Robot);
    grid.set(robot_pos.move_by(direction, 0), GridTile::Empty);
}

fn gps_coordinate_sum(grid: &Grid<GridTile>) -> u32 {
    grid.find(&GridTile::Box)
        .map(|pos| pos.x + pos.y * 100)
        .sum()
}

pub(crate) fn main() {
    let input = parse_input("day15_input.txt");
    let mut grid = input.start_grid.clone();
    println!("{}", grid.pretty_print(&pp_tile));
    for direction in input.move_plan {
        apply_move(&mut grid, direction);
    }
    println!("{}", grid.pretty_print(&pp_tile));
    println!("Star 1: {}", gps_coordinate_sum(&grid));
}

use crate::grid::{Direction, Grid, Position};
use itertools::Itertools;
use std::cmp::{min, PartialEq};
use std::collections::HashSet;
use std::{fs, iter};

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
enum GridTile {
    Empty,
    Box,
    BigBoxLeft,
    BigBoxRight,
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
                'O' => GridTile::Box,
                '[' => GridTile::BigBoxLeft,
                ']' => GridTile::BigBoxRight,
                '#' => GridTile::Wall,
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
        GridTile::BigBoxLeft => '[',
        GridTile::BigBoxRight => ']',
        GridTile::Wall => '#',
        GridTile::Robot => '@',
    }
}

fn apply_move(grid: &mut Grid<GridTile>, direction: Direction) {
    fn affected_by_moving_entity(
        grid: &Grid<GridTile>,
        direction: Direction,
        position: Position,
        step_sideways: bool,
    ) -> Option<Vec<Position>> {
        let entity = *grid.get(position).unwrap();
        let mut result = match entity {
            GridTile::Empty => Some(Vec::new()),
            GridTile::Box | GridTile::BigBoxLeft | GridTile::BigBoxRight | GridTile::Robot => {
                let mut rec =
                    affected_by_moving_entity(grid, direction, position.move_to(direction), true);
                if let Some(v) = &mut rec {
                    v.push(position);
                }
                rec
            }
            GridTile::Wall => None,
        };
        if result.is_some()
            && step_sideways
            && (direction == Direction::Up || direction == Direction::Down)
            && (entity == GridTile::BigBoxLeft || entity == GridTile::BigBoxRight)
        {
            let neighbour_pos = if entity == GridTile::BigBoxLeft {
                position.move_to(Direction::Right)
            } else {
                position.move_to(Direction::Left)
            };
            let neighbour_move = affected_by_moving_entity(grid, direction, neighbour_pos, false);
            if neighbour_move.is_none() {
                None
            } else {
                let mut x = result.unwrap();
                x.extend(neighbour_move.unwrap());
                Some(x)
            }
        } else {
            result
        }
    }

    let robot_pos = grid.find(&GridTile::Robot).exactly_one().ok().unwrap();
    let affected_positions = affected_by_moving_entity(grid, direction, robot_pos, true);
    if let Some(affected_positions) = affected_positions {
        let old_grid = grid.clone(); // slow as hell
        for &pos in affected_positions.iter() {
            grid.set(pos, GridTile::Empty);
        }
        for &pos in affected_positions.iter() {
            grid.set(pos.move_to(direction), *old_grid.get(pos).unwrap());
        }
    }
}

fn gps_coordinate_sum(grid: &Grid<GridTile>, grid_width: u32) -> u32 {
    let simple_boxes: u32 = grid
        .find(&GridTile::Box)
        .map(|pos| pos.x + pos.y * 100)
        .sum();
    let wide_boxes: u32 = grid
        .find(&GridTile::BigBoxLeft)
        .map(|pos| pos.x + pos.y * 100)
        .sum();
    simple_boxes + wide_boxes
}

fn widen_grid(grid: Grid<GridTile>) -> Grid<GridTile> {
    let entities = grid
        .entities
        .iter()
        .flat_map(|entity| match entity {
            GridTile::Empty => [GridTile::Empty, GridTile::Empty],
            GridTile::Box => [GridTile::BigBoxLeft, GridTile::BigBoxRight],
            GridTile::BigBoxLeft | GridTile::BigBoxRight => unreachable!(),
            GridTile::Wall => [GridTile::Wall, GridTile::Wall],
            GridTile::Robot => [GridTile::Robot, GridTile::Empty],
        })
        .collect();
    Grid {
        width: grid.width * 2,
        height: grid.height,
        entities,
    }
}

fn star1(input: &Input) {
    let mut grid = input.start_grid.clone();
    println!("{}", grid.pretty_print(&pp_tile));
    for &direction in input.move_plan.iter() {
        apply_move(&mut grid, direction);
    }
    println!("{}", grid.pretty_print(&pp_tile));
    println!("Star 1: {}", gps_coordinate_sum(&grid, grid.width));
}

fn star2(input: &Input) {
    let mut grid = widen_grid(input.start_grid.clone());
    println!("{}", grid.pretty_print(&pp_tile));
    for &direction in input.move_plan.iter() {
        apply_move(&mut grid, direction);
    }
    println!("{}", grid.pretty_print(&pp_tile));
    println!("Star 2: {}", gps_coordinate_sum(&grid, grid.width));
}

pub(crate) fn main() {
    let input = parse_input("day15_input.txt");
    star1(&input);
    star2(&input);
}

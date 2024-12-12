#![allow(dead_code)]

use std::collections::HashSet;
use std::fs;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Pos(usize, usize);

impl Pos {
    fn neighbours(&self) -> [Pos; 4] {
        [
            self.mov(Direction::Down),
            self.mov(Direction::Up),
            self.mov(Direction::Left),
            self.mov(Direction::Right),
        ]
    }

    fn mov(&self, direction: Direction) -> Pos {
        match direction {
            Direction::Up => Pos(self.0.wrapping_sub(1), self.1),
            Direction::Down => Pos(self.0.wrapping_add(1), self.1),
            Direction::Left => Pos(self.0, self.1.wrapping_sub(1)),
            Direction::Right => Pos(self.0, self.1.wrapping_add(1)),
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

static ALL_DIRECTIONS: [Direction; 4] = [
    Direction::Up,
    Direction::Down,
    Direction::Left,
    Direction::Right,
];

impl Direction {
    fn all() -> &'static [Direction; 4] {
        &ALL_DIRECTIONS
    }
}

#[derive(Debug)]
struct GardenPlot {
    width: usize,
    height: usize,
    plant_map: Vec<char>,
}

impl GardenPlot {
    fn get(&self, pos: Pos) -> Option<char> {
        if pos.0 >= self.width || pos.1 >= self.height {
            None
        } else {
            Some(self.plant_map[pos.0 + pos.1 * self.width])
        }
    }
}

fn parse_input(filename: &str) -> GardenPlot {
    let file_contents = fs::read_to_string(filename).unwrap();
    let width = file_contents.lines().next().unwrap().chars().count();
    let height = file_contents.lines().count();
    let plant_map = file_contents
        .lines()
        .flat_map(|line| line.chars())
        .collect();
    GardenPlot {
        width,
        height,
        plant_map,
    }
}

fn find_regions(garden_plot: &GardenPlot) -> Vec<HashSet<Pos>> {
    let mut regions: Vec<HashSet<Pos>> = Vec::new();

    fn dfs_region(garden_plot: &GardenPlot, pos: Pos, region: &mut HashSet<Pos>) {
        if region.contains(&pos) {
            return;
        }
        region.insert(pos);
        let c = garden_plot.get(pos).unwrap();
        for neighbour in pos.neighbours() {
            if garden_plot.get(neighbour) == Some(c) {
                dfs_region(garden_plot, neighbour, region);
            }
        }
    }

    for x in 0..garden_plot.width {
        for y in 0..garden_plot.height {
            let pos = Pos(x, y);
            if regions.iter_mut().find(|r| r.contains(&pos)).is_none() {
                regions.push(HashSet::new());
                dfs_region(garden_plot, pos, regions.last_mut().unwrap());
            };
        }
    }

    regions
}

fn region_perimeter(region: &HashSet<Pos>) -> HashSet<(Pos, Direction)> {
    region
        .iter()
        .flat_map(|&pos| {
            Direction::all().iter().filter_map(move |&direction| {
                if region.contains(&pos.mov(direction)) {
                    None
                } else {
                    Some((pos, direction))
                }
            })
        })
        .collect()
}

fn region_sides(region: &HashSet<Pos>) -> HashSet<(Pos, Direction)> {
    let perimeter = region_perimeter(region);
    let mut sides = HashSet::new();

    for &(pos, fence_direction) in perimeter.iter() {
        let orthogonal_direction =
            if fence_direction == Direction::Up || fence_direction == Direction::Down {
                Direction::Left
            } else {
                Direction::Up
            };
        let mut sliding_pos = pos;
        loop {
            let next_sliding_pos = sliding_pos.mov(orthogonal_direction);
            if !region.contains(&next_sliding_pos)
                || !perimeter.contains(&(next_sliding_pos, fence_direction))
            {
                break;
            }
            sliding_pos = next_sliding_pos;
        }
        sides.insert((sliding_pos, fence_direction));
    }

    sides
}

pub(crate) fn main() {
    let garden_plot = parse_input("day12_input.txt");
    let regions = find_regions(&garden_plot);

    println!(
        "Star 1: {}",
        regions
            .iter()
            .map(|region| region.len() * region_perimeter(region).len())
            .sum::<usize>()
    );

    println!(
        "Star 2: {}",
        regions
            .iter()
            .map(|region| region.len() * region_sides(region).len())
            .sum::<usize>()
    );
}

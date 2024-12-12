#![allow(dead_code)]

use std::collections::HashSet;
use std::fs;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Pos(usize, usize);

impl Pos {
    fn neighbours(&self) -> [Pos; 4] {
        [
            Pos(self.0.wrapping_sub(1), self.1),
            Pos(self.0.wrapping_add(1), self.1),
            Pos(self.0, self.1.wrapping_sub(1)),
            Pos(self.0, self.1.wrapping_add(1)),
        ]
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

fn region_area(garden_plot: &GardenPlot, region: &HashSet<Pos>) -> usize {
    region.len()
}

fn region_perimeter(garden_plot: &GardenPlot, region: &HashSet<Pos>) -> usize {
    region
        .iter()
        .map(|pos| {
            pos.neighbours()
                .iter()
                .filter(|n| !region.contains(n))
                .count()
        })
        .sum()
}

pub(crate) fn main() {
    let garden_plot = parse_input("day12_input.txt");
    let regions = find_regions(&garden_plot);

    let fence_costs: Vec<_> = regions
        .iter()
        .map(|region| region_area(&garden_plot, region) * region_perimeter(&garden_plot, region))
        .collect();

    println!("{:?}", fence_costs);
    println!("{}", fence_costs.iter().sum::<usize>());
}

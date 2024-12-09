#![allow(dead_code)]

use itertools::Itertools;
use leptos::prelude::*;
use std::collections::HashSet;
use std::iter::Iterator;
use std::thread;
use std::time::Duration;
use leptos::task::spawn_local;

#[derive(Debug, Eq, Hash, PartialEq, Copy, Clone)]
struct Pos(i32, i32);

impl Pos {
    fn mov(self: Pos, to: &Direction) -> Pos {
        match to {
            Direction::Left => Pos(self.0 - 1, self.1),
            Direction::Right => Pos(self.0 + 1, self.1),
            Direction::Up => Pos(self.0, self.1 - 1),
            Direction::Down => Pos(self.0, self.1 + 1),
        }
    }
}

#[derive(Debug)]
struct MapBase {
    width: i32,
    height: i32,
    blockades: HashSet<Pos>,
}

impl MapBase {
    fn is_blocked(self: &MapBase, pos: Pos) -> bool {
        self.blockades.contains(&pos)
    }

    fn is_out_of_map(self: &MapBase, Pos(x, y): Pos) -> bool {
        x < 0 || x >= self.width || y < 0 || y >= self.height
    }
}

#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Direction {
    fn rotate(self: &Direction) -> Direction {
        match self {
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
        }
    }
}

#[derive(Debug, Clone)]
struct MapState {
    guard_position: Pos,
    guard_direction: Direction,
    visited: HashSet<(Pos, Direction)>,
}

impl MapState {
    fn step(self: &mut MapState, map_base: &MapBase) {
        self.visited
            .insert((self.guard_position, self.guard_direction));
        let mut new_pos = self.guard_position.mov(&self.guard_direction);
        while map_base.is_blocked(new_pos) {
            self.guard_direction = self.guard_direction.rotate();
            new_pos = self.guard_position.mov(&self.guard_direction);
        }
        self.guard_position = new_pos;
    }
}

fn parse_input() -> (MapBase, MapState) {
    let mut base = MapBase {
        width: INPUT.lines().next().unwrap().chars().count() as i32,
        height: INPUT.lines().count() as i32,
        blockades: HashSet::new(),
    };
    let mut guard_position = None;
    let mut guard_direction = None;
    for (i, c) in INPUT.chars().filter(|c| *c != '\n').enumerate() {
        let pos = Pos(i as i32 % base.width, i as i32 / base.height);
        match c {
            '.' => {}
            '#' => {
                base.blockades.insert(pos);
            }
            '^' => {
                guard_position = Some(pos);
                guard_direction = Some(Direction::Up);
            }
            _ => {
                unreachable!();
            }
        }
    }
    (
        base,
        MapState {
            guard_position: guard_position.unwrap(),
            guard_direction: guard_direction.unwrap(),
            visited: HashSet::new(),
        },
    )
}

pub(crate) fn main() {
    let (base, mut state) = parse_input();
    while !base.is_out_of_map(state.guard_position) {
        state.step(&base);
    }
    println!(
        "Star 1: {}",
        state
            .visited
            .into_iter()
            .map(|(p, _)| { p })
            .unique()
            .count()
    );
}

#[component]
pub(crate) fn App() -> impl IntoView {
    let (value, set_value) = signal(0);

    let x = spawn_local(async {
        for i in 1..10 {
            leptos::logging::log!("hi number {i} from the spawned thread!");
        }
    });

    // thanks to https://tailwindcomponents.com/component/blue-buttons-example for the showcase layout
    view! {
        <h1>Leptos + Tailwindcss</h1>
        <main>
            <div class="bg-gradient-to-tl from-blue-800 to-blue-500 text-white font-mono flex flex-col min-h-screen">
                <div class="flex flex-row-reverse flex-wrap m-auto">
                    <button on:click=move |_| set_value.update(|value| *value += 1) class="rounded px-3 py-2 m-1 border-b-4 border-l-2 shadow-lg bg-blue-700 border-blue-800 text-white">
                        "+"
                    </button>
                    <button class="rounded px-3 py-2 m-1 border-b-4 border-l-2 shadow-lg bg-blue-800 border-blue-900 text-white">
                        {value}
                    </button>
                    <button
                        on:click=move |_| set_value.update(|value| *value -= 1)
                        class="rounded px-3 py-2 m-1 border-b-4 border-l-2 shadow-lg bg-blue-700 border-blue-800 text-white"
                        class:invisible=move || {value.get() < 1}
                    >
                        "-"
                    </button>
                </div>
            </div>
        </main>
    }
}

const INPUT: &str = include_str!("../day6_input.txt");
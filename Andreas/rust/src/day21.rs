#![allow(dead_code)]

use crate::grid::Position;
use itertools::Itertools;
use std::fs;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum KeypadTile {
    A,
    Blank,
    Num0,
    Num1,
    Num2,
    Num3,
    Num4,
    Num5,
    Num6,
    Num7,
    Num8,
    Num9,
}

impl KeypadTile {
    fn position(&self) -> Position {
        match self {
            KeypadTile::A => Position { x: 2, y: 3 },
            KeypadTile::Blank => Position { x: 0, y: 3 },
            KeypadTile::Num0 => Position { x: 1, y: 3 },
            KeypadTile::Num1 => Position { x: 0, y: 2 },
            KeypadTile::Num2 => Position { x: 1, y: 2 },
            KeypadTile::Num3 => Position { x: 2, y: 2 },
            KeypadTile::Num4 => Position { x: 0, y: 1 },
            KeypadTile::Num5 => Position { x: 1, y: 1 },
            KeypadTile::Num6 => Position { x: 2, y: 1 },
            KeypadTile::Num7 => Position { x: 0, y: 0 },
            KeypadTile::Num8 => Position { x: 1, y: 0 },
            KeypadTile::Num9 => Position { x: 2, y: 0 },
        }
    }
}

impl From<char> for KeypadTile {
    fn from(value: char) -> Self {
        match value {
            'A' => KeypadTile::A,
            ' ' => KeypadTile::Blank,
            '0' => KeypadTile::Num0,
            '1' => KeypadTile::Num1,
            '2' => KeypadTile::Num2,
            '3' => KeypadTile::Num3,
            '4' => KeypadTile::Num4,
            '5' => KeypadTile::Num5,
            '6' => KeypadTile::Num6,
            '7' => KeypadTile::Num7,
            '8' => KeypadTile::Num8,
            '9' => KeypadTile::Num9,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
enum DpadTile {
    A,
    Blank,
    Left,
    Right,
    Up,
    Down,
}

fn keypad_sequence_to_directions(start: KeypadTile, sequence: &[KeypadTile]) -> Vec<DpadTile> {
    let mut result = Vec::new();
    let mut current_pos = start.position();
    for target_tile in sequence {
        let target_pos = target_tile.position();
        while current_pos != target_pos {
            let next_dpad = if current_pos.x != target_pos.x {
                match (current_pos.x, target_pos.x) {
                    (0, 1) => DpadTile::Right,
                    (0, 2) => DpadTile::Left,
                    (1, 0) => DpadTile::Left,
                    (1, 2) => DpadTile::Right,
                    (2, 0) => DpadTile::Right,
                    (2, 1) => DpadTile::Left,
                    _ => unreachable!(),
                }
            } else {
                match (current_pos.y, target_pos.y) {
                    (0, 1) => DpadTile::Up,
                    (0, 2) => DpadTile::Up,
                    (0, 3) => DpadTile::Up,
                    (1, 0) => DpadTile::Up,
                    (1, 2) => DpadTile::Up,
                    (1, 3) => DpadTile::Up,
                    (2, 0) => DpadTile::Up,
                    (2, 1) => DpadTile::Up,
                    (2, 3) => DpadTile::Up,
                    (3, 0) => DpadTile::Up,
                    (3, 1) => DpadTile::Up,
                    (3, 2) => DpadTile::Up,
                    _ => unreachable!(),
                }
            };
            result.push(next_dpad);
            match next_dpad {
                DpadTile::Left => current_pos.x = (current_pos.x + 2) % 3,
                DpadTile::Right => current_pos.x = (current_pos.x + 1) % 3,
                DpadTile::Up => current_pos.y = (current_pos.y + 3) % 4,
                DpadTile::Down => current_pos.y = (current_pos.y + 1) % 4,
                _ => unreachable!(),
            };
        }
        result.push(DpadTile::A);
    }
    result
}

fn parse_input(filename: &str) -> Vec<Vec<KeypadTile>> {
    let file_contents = fs::read_to_string(filename).unwrap();
    file_contents
        .lines()
        .map(|line| line.chars().map(|c| KeypadTile::from(c)).collect_vec())
        .collect_vec()
}

pub(crate) fn main() {
    let input = parse_input("day21_input.txt");
    println!("{:?}", input);

    println!("{:?}", keypad_sequence_to_directions(KeypadTile::A, &input[0]));
}

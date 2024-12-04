#![allow(dead_code)]

use std::fs;

#[derive(Debug)]
struct WordMatrix {
    letters: Vec<char>,
    width: i32,
    height: i32,
}

impl WordMatrix {
    fn new(filename: &str) -> WordMatrix {
        let input = fs::read_to_string(filename).unwrap();
        let lines: Vec<_> = input.lines().collect();
        let width = lines[0].len() as i32;
        let height = lines.len() as i32;
        let letters = lines
            .into_iter()
            .map(|line| line.chars())
            .flatten()
            .collect();
        WordMatrix {
            width,
            height,
            letters,
        }
    }

    fn get(self: &WordMatrix, x: i32, y: i32) -> Option<char> {
        if x < 0 || y < 0 || x >= self.width || y >= self.height {
            None
        } else {
            let i = (x + y * self.width) as usize;
            Some(self.letters[i])
        }
    }
}

fn count_xmas_from(matrix: &WordMatrix, (x, y): (i32, i32), word: &str) -> i32 {
    let deltas = [
        (0, 1),
        (1, 0),
        (0, -1),
        (-1, 0),
        (1, 1),
        (-1, 1),
        (1, -1),
        (-1, -1),
    ];
    let mut count = 0;
    'outer: for delta in deltas {
        let mut x = x;
        let mut y = y;
        for c in word.chars() {
            if matrix.get(x, y) != Some(c) {
                continue 'outer;
            }
            x += delta.0;
            y += delta.1;
        }
        count += 1;
    }
    count
}

pub(crate) fn main() {
    let matrix = WordMatrix::new("day4_input.txt");
    let mut xmas_count = 0;
    for x in 0..matrix.width {
        for y in 0..matrix.height {
            xmas_count += count_xmas_from(&matrix, (x, y), "XMAS");
        }
    }

    println!("{}", xmas_count);
}

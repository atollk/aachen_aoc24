#![allow(dead_code)]

use std::fs;
use std::sync::LazyLock;

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


static SLIDE_DELTAS_ALL: LazyLock<[(i32, i32); 8]> = LazyLock::new(|| {
    [
        (0, 1),
        (1, 0),
        (0, -1),
        (-1, 0),
        (1, 1),
        (-1, 1),
        (1, -1),
        (-1, -1),
    ]
});

static SLIDE_DELTAS_CROSS: LazyLock<[(i32, i32); 4]> = LazyLock::new(|| {
    [
        (1, 1),
        (-1, 1),
        (1, -1),
        (-1, -1),
    ]
});

fn find_words_from(matrix: &WordMatrix, (sx, sy): (i32, i32), word: &str, slide_deltas: &[(i32, i32)]) -> Vec<((i32, i32), (i32, i32))> {
    let mut findings = Vec::new();
    'outer: for delta in slide_deltas.iter() {
        let mut x = sx;
        let mut y = sy;
        for c in word.chars() {
            if matrix.get(x, y) != Some(c) {
                continue 'outer;
            }
            x += delta.0;
            y += delta.1;
        }
        findings.push(((sx, sy), (x - delta.0, y - delta.1)));
    }
    findings
}

fn find_all_words(matrix: &WordMatrix, word: &str, slide_deltas: &[(i32, i32)]) -> Vec<((i32, i32), (i32, i32))> {
    let mut words = Vec::new();
    for x in 0..matrix.width {
        for y in 0..matrix.height {
            words.append(&mut find_words_from(&matrix, (x, y), word, slide_deltas));
        }
    }
    words
}

fn star2(matrix: &WordMatrix) {
    let mas_words = find_all_words(matrix, "MAS", SLIDE_DELTAS_CROSS.as_ref());
    let mut x_count = 0;
    for i in 0..mas_words.len() {
        for j in (i+1)..mas_words.len() {
            let (astart, aend) = mas_words[i];
            let amid = ((astart.0 + aend.0) / 2, (astart.1 + aend.1) / 2);
            let (bstart, bend) = mas_words[j];
            let bmid = ((bstart.0 + bend.0) / 2, (bstart.1 + bend.1) / 2);
            if amid == bmid {
                x_count += 1;
            }
        }
    }

    println!("Star 2: {}", x_count);
}

pub(crate) fn main() {
    let matrix = WordMatrix::new("day4_input.txt");
    println!("Star 1: {}", find_all_words(&matrix, "XMAS", SLIDE_DELTAS_ALL.as_ref()).len());
    star2(&matrix);
}

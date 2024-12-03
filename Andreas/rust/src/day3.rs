#![allow(dead_code)]

use regex::Regex;
use std::fs;
use std::sync::LazyLock;

static EVAL_MUL_RE: LazyLock<Regex> = LazyLock::new(|| {Regex::new(r"^mul\((\d+),(\d+)\)$").unwrap()});

fn eval_mul(s: &str) -> u32 {
    let capture = EVAL_MUL_RE.captures(s).unwrap();
    let a: u32 = capture.get(1).unwrap().as_str().parse().unwrap();
    let b: u32 = capture.get(2).unwrap().as_str().parse().unwrap();
    a * b
}

pub(crate) fn main() {
    let input = fs::read_to_string("day3_input.txt").unwrap();

    {
        // star 1
        let re = Regex::new(r"mul\(\d+,\d+\)").unwrap();
        let sum: u32 = re
            .captures_iter(&input)
            .map(|c| eval_mul(c.get(0).unwrap().as_str()))
            .sum();
        println!("Star 1: {}", sum);
    }

    {
        // star 2
        let re = Regex::new(r"(mul\(\d+,\d+\))|(do\(\))|(don't\(\))").unwrap();
        let mut sum = 0;
        let mut enabled = true;
        for capture in re.captures_iter(&input) {
            if let Some(m) = capture.get(1) {
                if enabled {
                    sum += eval_mul(m.as_str());
                }
            } else if let Some(_) = capture.get(2) {
                enabled = true;
            } else if let Some(_) = capture.get(3) {
                enabled = false;
            } else {
                unreachable!();
            }
        }
        println!("Star 2: {}", sum);
    }
}

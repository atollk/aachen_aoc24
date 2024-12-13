#![allow(dead_code)]

use itertools::Itertools;
use regex::{Captures, Regex};
use std::fs;

#[derive(Debug)]
struct ClawMachine {
    a_delta: (i64, i64),
    b_delta: (i64, i64),
    prize: (i64, i64),
}

fn parse_input(filename: &str) -> Vec<ClawMachine> {
    let file_contents = fs::read_to_string(filename).unwrap();
    let machine_blocks = file_contents.split("\n\n");
    let button_regex = Regex::new(r"Button \w: X\+(\d+), Y\+(\d+)").unwrap();
    let prize_regex = Regex::new(r"Prize: X=(\d+), Y=(\d+)").unwrap();
    let unwrap_capture = |captures: Captures| -> (i64, i64) {
        (
            captures.get(1).unwrap().as_str().parse().unwrap(),
            captures.get(2).unwrap().as_str().parse().unwrap(),
        )
    };
    machine_blocks
        .map(|block| {
            let (a_line, b_line, prize_line) = block.lines().collect_tuple().unwrap();
            ClawMachine {
                a_delta: { unwrap_capture(button_regex.captures(a_line).expect(a_line)) },
                b_delta: { unwrap_capture(button_regex.captures(b_line).expect(b_line)) },
                prize: { unwrap_capture(prize_regex.captures(prize_line).expect(prize_line)) },
            }
        })
        .collect()
}

fn get_button_presses(machine: &ClawMachine) -> Option<(i64, i64)> {
    let a = (machine.prize.1 * machine.b_delta.0 - machine.prize.0 * machine.b_delta.1)
        / (machine.a_delta.1 * machine.b_delta.0 - machine.a_delta.0 * machine.b_delta.1);
    let b = (machine.prize.0 - a * machine.a_delta.0) / machine.b_delta.0;
    if a * machine.a_delta.0 + b * machine.b_delta.0 == machine.prize.0
        && a * machine.a_delta.1 + b * machine.b_delta.1 == machine.prize.1
    {
        Some((a, b))
    } else {
        None
    }
}

fn compute_cost_sum(machines: &[ClawMachine]) -> u64 {
    let costs: Vec<_> = machines
        .iter()
        .filter_map(|m| get_button_presses(&m))
        .map(|(a_press, b_press)| 3 * a_press + b_press)
        .collect();
    //println!("{:?}", costs);
    costs.iter().map(|c| *c as u64).sum()
}

pub(crate) fn main() {
    let mut machines = parse_input("day13_input.txt");
    println!("Star 1: {}", compute_cost_sum(&machines));
    for machine in machines.iter_mut() {
        machine.prize.0 += 10000000000000;
        machine.prize.1 += 10000000000000;
    }
    println!("Star 2: {}", compute_cost_sum(&machines));
}

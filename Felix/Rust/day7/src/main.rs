use std::fs::File;
use std::io::{self, prelude::*, BufReader};

use itertools::Itertools;

#[derive(Debug)]
struct Computation(u64, Vec<u64>);

fn concatenate(lhs: u64, rhs: u64) -> u64 {
    let mut cc = lhs.to_string();
    cc.push_str(&rhs.to_string());
    cc.parse::<u64>().unwrap()
}

fn dfs(seq: &Vec<u64>, target: u64, i: usize, total: u64) -> bool {
    if i == seq.len() {
        return target == total;
    }
    dfs(seq, target, i + 1, total * seq[i])
        || dfs(seq, target, i + 1, total + seq[i])
        || dfs(seq, target, i + 1, concatenate(total, seq[i]))
}

fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let lines = reader.lines().filter_map(|line| line.ok()).collect_vec();
    let input = lines
        .iter()
        .map(|line| {
            let split_line = line.split(": ").collect_vec();
            let first = split_line.first().unwrap().parse::<u64>().unwrap();
            let second = split_line
                .last()
                .unwrap()
                .split(" ")
                .map(|e| e.parse::<u64>().unwrap())
                .collect_vec();
            Computation(first, second)
        })
        .collect_vec();

    let mut res = 0;
    for Computation(target, seq) in input {
        if dfs(&seq, target, 0, 0) {
            res += target;
        }
    }

    println!("{:?}", res);

    Ok(())
}

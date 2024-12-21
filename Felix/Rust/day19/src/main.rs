use ::std::fs::File;
use std::io::{self, prelude::*, BufReader};

use itertools::Itertools;

fn is_possible(design: &str, patterns: &Vec<&str>) -> bool {
    let mut dp = vec![false; design.len() + 1];
    dp[design.len()] = true;

    for i in (0..design.len()).rev() {
        for pattern in patterns {
            if i + pattern.len() <= design.len() {
                dp[i] = dp[i] || dp[i + pattern.len()] && design[i..].starts_with(pattern);
            }
        }
    }

    dp[0]
}

fn get_combinations(design: &str, patterns: &Vec<&str>) -> u64 {
    let mut dp = vec![0; design.len() + 1];
    dp[design.len()] = 1;

    for i in (0..design.len()).rev() {
        for pattern in patterns {
            if i + pattern.len() <= design.len() {
                if design[i..].starts_with(pattern) {
                    dp[i] += dp[i + pattern.len()]
                }
            }
        }
    }

    dp[0]
}

fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let lines = BufReader::new(file)
        .lines()
        .map(|line| line.unwrap())
        .collect_vec();

    let patterns = lines.first().unwrap().split(", ").collect_vec();
    let designs = lines.iter().skip(2).collect_vec();

    let res1 = designs
        .iter()
        .filter(|design| is_possible(design, &patterns)).count();
    println!("{:?}", res1);

    let res2 = designs
        .iter()
        .fold(0, |acc, design| acc + get_combinations(design, &patterns));
    println!("{:?}", res2);

    Ok(())
}

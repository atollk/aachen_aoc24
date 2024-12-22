use ::std::fs::File;
use std::{
    collections::{HashMap, HashSet},
    io::{self, prelude::*, BufReader},
    ops::BitXor,
};

fn mix(secret_number: i64, value: i64) -> i64 {
    secret_number.bitxor(value)
}

fn prune(secret_number: i64) -> i64 {
    secret_number % 16777216
}

fn evolve(secret_number: i64) -> i64 {
    let mixer = 64 * secret_number;
    let secret_number = mix(secret_number, mixer);
    let secret_number = prune(secret_number);
    let mixer = secret_number / 32;
    let secret_number = mix(secret_number, mixer);
    let secret_number = prune(secret_number);
    let mixer = 2048 * secret_number;
    let secret_number = mix(secret_number, mixer);
    let secret_number = prune(secret_number);
    secret_number
}

fn translate(secret_number: i64) -> i64 {
    secret_number % 10
}

use itertools::Itertools;

fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let secret_numbers = BufReader::new(file)
        .lines()
        .map(|line| line.unwrap().parse::<i64>().unwrap())
        .collect_vec();
    let res1 = secret_numbers.iter().fold(0, |acc, secret_number| {
        let mut secret_number = *secret_number;
        for _ in 0..2000 {
            secret_number = evolve(secret_number);
        }
        acc + secret_number
    });
    println!("{:?}", res1);

    let res2 = secret_numbers
        .iter()
        .map(|secret_number| {
            let mut secret_number = *secret_number;
            let mut seq = Vec::new();
            let mut prev = translate(secret_number);
            for _ in 0..2000 {
                secret_number = evolve(secret_number);
                let digit = translate(secret_number);
                seq.push((digit, digit - prev));
                prev = digit;
            }
            seq
        })
        .fold(
            HashMap::<(i64, i64, i64, i64), i64>::new(),
            |mut acc, seq| {
                let mut visited = HashSet::new();
                seq.iter()
                    .tuple_windows()
                    .for_each(|(first, second, third, fourth)| {
                        let key = (first.1, second.1, third.1, fourth.1);
                        if !visited.contains(&key) {
                            visited.insert(key);
                            *acc.entry(key).or_default() += fourth.0;
                        }
                    });
                acc
            },
        )
        .iter()
        .fold(0, |acc, (_, v)| acc.max(*v));
    println!("{:?}", res2);

    Ok(())
}

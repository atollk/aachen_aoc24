#![allow(dead_code)]

use gat_lending_iterator::ToLendingIterator;
use itertools::Itertools;
use std::cmp::max;
use std::collections::HashMap;
use std::fs;

const SECRET_NUMBER_PRUNE: u64 = 2u64.pow(24) - 1;
const STEPS: usize = 2000;

fn parse_input(filename: &str) -> Vec<u64> {
    let file_contents = fs::read_to_string(filename).unwrap();
    file_contents
        .lines()
        .map(|line| line.parse().unwrap())
        .collect()
}

fn evolve_secret_number(n: u64) -> u64 {
    let a = ((n << 6) ^ n) & SECRET_NUMBER_PRUNE;
    let b = (a >> 5) ^ a;
    ((b << 11) ^ b) & SECRET_NUMBER_PRUNE
}

pub(crate) fn main() {
    let input = parse_input("day22_input.txt");

    // println!("{:?}", input);
    // for n in input {
    //     println!("{}: {}", n, evolve_secret_number_multi(n, 2000));
    // }

    let buyer_secret_numbers: Vec<[u64; STEPS]> = input
        .iter()
        .map(|n| {
            (0..STEPS)
                .scan(*n, |state, _| {
                    let res = *state;
                    *state = evolve_secret_number(*state);
                    Some(res)
                })
                .collect_vec()
                .try_into()
                .unwrap()
        })
        .collect_vec();
    let buyer_prices: Vec<[u64; STEPS]> = buyer_secret_numbers
        .iter()
        .map(|secrets| secrets.map(|i| i % 10))
        .collect_vec();
    let buyer_change_sequences: Vec<[i8; STEPS - 1]> = buyer_prices
        .iter()
        .map(|prices| {
            prices
                .iter()
                .scan(0i64, |state, i| {
                    let prev = *state;
                    *state = *i as i64;
                    Some((*state - prev) as i8)
                })
                .skip(1)
                .collect_vec()
                .try_into()
                .unwrap()
        })
        .collect_vec();

    println!(
        "Star 1: {}",
        buyer_secret_numbers
            .iter()
            .map(|secrets| secrets.last().unwrap())
            .sum::<u64>()
    );

    let buyer_prices_for_sequence = buyer_change_sequences
        .iter()
        .enumerate()
        .map(|(n, change_seq)| {
            let mut price_map = HashMap::new();
            for i in 0..change_seq.len() - 3 {
                let window = [0, 1, 2, 3].map(|j| change_seq[i + j]);
                price_map.entry(window).or_insert(buyer_prices[n][i + 4]);
            }
            price_map
        })
        .collect_vec();

    let all_possible_change_sequences = buyer_prices_for_sequence
        .iter()
        .flat_map(|map| map.keys().map(|k| *k))
        .unique()
        .collect_vec();

    let mut best_price_sum = 0;
    for change_sequence in tqdm::tqdm(all_possible_change_sequences) {
        let mut price_sum = 0;
        for bpfs in buyer_prices_for_sequence.iter() {
            price_sum += bpfs.get(&change_sequence).unwrap_or(&0);
        }
        best_price_sum = max(price_sum, best_price_sum);
    }
    println!("Star 2: {}", best_price_sum);
}

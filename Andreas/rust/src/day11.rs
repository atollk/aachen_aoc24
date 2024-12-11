#![allow(dead_code)]

use std::fs;

const MAX_STEPS: usize = 100;

fn parse_input(filename: &str) -> Vec<u64> {
    let file_contents = fs::read_to_string(filename).unwrap();
    file_contents
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect()
}

type PrecomputeArray = [[usize; MAX_STEPS + 1]; 10];

fn precompute_base_stones() -> PrecomputeArray {
    let mut result: PrecomputeArray = [[usize::MAX; MAX_STEPS + 1]; 10];
    for i in 0..=9 {
        result[i][0] = 1;
    }

    fn compute(n: u64, steps: usize, result: &mut PrecomputeArray) -> usize {
        if steps == 0 {
            1
        } else if n >= 10 {
            let n_digits = n.ilog10() + 1;
            if n_digits % 2 == 0 {
                let digits_pow = 10u64.pow(n_digits / 2);
                let n_lhs = n / digits_pow;
                let n_rhs = n % digits_pow;
                compute(n_lhs, steps - 1, result) + compute(n_rhs, steps - 1, result)
            } else {
                compute(n * 2024, steps - 1, result)
            }
        } else {
            if result[n as usize][steps] == usize::MAX {
                let m = if n == 0 { 1 } else { n * 2024 };
                result[n as usize][steps] = compute(m, steps - 1, result);
            }
            result[n as usize][steps]
        }
    }

    for n in 0..=9 {
        for steps in 1..=MAX_STEPS {
            compute(n, steps, &mut result);
        }
    }

    result
}

fn count_stones(n: u64, steps: usize, precomputed_base: &PrecomputeArray) -> usize {
    if steps == 0 {
        1
    } else if n < 10 {
        precomputed_base[n as usize][steps]
    } else {
        let n_digits = n.ilog10() + 1;
        if n_digits % 2 == 0 {
            let digits_pow = 10u64.pow(n_digits / 2);
            let n_lhs = n / digits_pow;
            let n_rhs = n % digits_pow;
            count_stones(n_lhs, steps - 1, precomputed_base)
                + count_stones(n_rhs, steps - 1, precomputed_base)
        } else {
            count_stones(n * 2024, steps - 1, precomputed_base)
        }
    }
}

pub(crate) fn main() {
    let input = parse_input("day11_input.txt");
    let base_stones = precompute_base_stones();

    println!(
        "Star 1: {}",
        input
            .iter()
            .map(|x| count_stones(*x, 25, &base_stones))
            .sum::<usize>()
    );

    println!(
        "Star 2: {}",
        input
            .iter()
            .map(|x| count_stones(*x, 75, &base_stones))
            .sum::<usize>()
    );
}

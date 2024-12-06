#![allow(dead_code)]

use std::collections::{HashMap, HashSet};
use std::fs;

#[derive(Debug)]
struct Input {
    rules: HashSet<(usize, usize)>,
    pages: Vec<Vec<usize>>,
}

fn parse_input(filename: &str) -> Input {
    let file_contents = fs::read_to_string(filename).unwrap();
    let file_parts: Vec<_> = file_contents.split("\n\n").collect();
    assert_eq!(file_parts.len(), 2);
    let rules_raw = file_parts[0];
    let pages_raw = file_parts[1];
    let rules = rules_raw
        .lines()
        .map(|line| {
            let split_l: [usize; 2] = line
                .split("|")
                .map(|x| x.parse().unwrap())
                .collect::<Vec<_>>()
                .try_into()
                .unwrap();
            split_l.try_into().unwrap()
        })
        .collect();
    let pages = pages_raw
        .lines()
        .map(|line| line.split(",").map(|x| x.parse().unwrap()).collect())
        .collect();
    Input { rules, pages }
}

fn find_broken_rule_in_page_split(
    lhs: &HashMap<usize, i32>,
    rhs: &HashMap<usize, i32>,
    rules: &HashSet<(usize, usize)>
) -> Option<(usize, usize)> {
    for rule in rules {
        if *rhs.get(&rule.0).unwrap_or(&0) > 0 && *lhs.get(&rule.1).unwrap_or(&0) > 0 {
            return Some(*rule);
        }
    }
    None
}

fn find_broken_rule_in_page(page: &[usize], rules: &HashSet<(usize, usize)>) -> Option<(usize, usize)> {
    let mut lhs = HashMap::new();
    let mut rhs = {
        let mut rhs = HashMap::new();
        for k in page {
            rhs.entry(*k).and_modify(|v| *v += 1).or_insert(1);
        }
        rhs
    };

    for k in page {
        let rhs_v = rhs.get_mut(k).unwrap();
        *rhs_v -= 1;
        if *rhs_v == 0 {
            rhs.remove(k);
        }
        lhs.entry(*k).and_modify(|v| *v += 1).or_insert(1);

        if let Some(x) = find_broken_rule_in_page_split(&lhs, &rhs, rules) {
            return Some(x);
        }
    }

    None
}

fn star1() {
    let input = parse_input("day5_input.txt");

    let mut midsum = 0;
    for page in input.pages.iter() {
        if find_broken_rule_in_page(page, &input.rules).is_none() {
            midsum += page[page.len() / 2];
        }
    }

    println!("Star 1: {}", midsum);
}

fn fix_broken_page(
    page: &mut [usize],
    rules: &HashSet<(usize, usize)>,
) {
    // i hate this
    while let Some(rule) = find_broken_rule_in_page(page, rules) {
        let i = page.iter().position(|x| *x == rule.0).unwrap();
        let j = page.iter().position(|x| *x == rule.1).unwrap();
        page.swap(i, j);
    }
}

fn star2() {
    let mut input = parse_input("day5_input.txt");

    let mut midsum = 0;
    for page in input.pages.iter_mut() {
        if find_broken_rule_in_page(page, &input.rules).is_none() {
            continue;
        }
        fix_broken_page(page, &input.rules);
        midsum += page[page.len() / 2];
    }

    println!("Star 2: {}", midsum);
}

pub(crate) fn main() {
    star1();
    star2();
}

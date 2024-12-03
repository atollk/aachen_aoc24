#![allow(dead_code)]

use std::collections::HashMap;
use std::fs;
use std::iter::zip;

fn parse_input(filename: &str) -> Vec<(i32, i32)> {
    let file_contents = fs::read_to_string(filename).unwrap();
    file_contents
        .lines()
        .map(|line| {
            let mut values = line.split("   ").map(|x| x.parse().unwrap());
            (values.next().unwrap(), values.next().unwrap())
        })
        .collect()
}

fn star1() {
    let input = parse_input("day1_input.txt");
    let lists: (Vec<_>, Vec<_>) = input.into_iter().unzip();
    let list1_sorted = {
        let mut x = lists.0;
        x.sort();
        x
    };
    let list2_sorted = {
        let mut x = lists.1;
        x.sort();
        x
    };
    let deltas: i32 = zip(list1_sorted.into_iter(), list2_sorted.into_iter()).map(|(x, y)| (x - y).abs()).sum();
    println!("star 1: {}", deltas);
}

fn star2() {
    let input = parse_input("day1_input.txt");
    let lists: (Vec<_>, Vec<_>) = input.into_iter().unzip();
    let list1_counter = {
        let mut counter = HashMap::new();
        for x in lists.0 {
            counter.entry(x).and_modify(|y| *y += 1).or_insert(1);
        }
        counter
    };
    let mut score = 0;
    for x in lists.1 {
        score += list1_counter.get(&x).unwrap_or(&0) * x;
    }
    println!("{}", score);
}

pub(crate) fn main() {
    star1();
    star2();
}

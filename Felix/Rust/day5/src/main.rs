use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::env;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

use itertools::Itertools;

fn fix_top_order(seq: &Vec<u32>, edges: &Vec<Vec<u32>>) -> u32 {
    let adj = edges
        .iter()
        .fold(HashMap::<u32, Vec<u32>>::new(), |mut acc, edge| {
            if seq.contains(&edge[0]) && seq.contains(&edge[1]) {
                acc.entry(edge[1]).or_default().push(edge[0]);
                if !acc.contains_key(&edge[0]) {
                    acc.insert(edge[0], Vec::new());
                }
                acc
            } else {
                acc
            }
        });
    println!("{:?}", seq);
    println!("{:?}", adj);
    let top_ordered = seq.iter().tuples().fold(true, |acc, (lhs, rhs)| {
        acc && 1 + adj.get(&lhs).unwrap().len() == adj.get(&rhs).unwrap().len()
    });
    if !top_ordered {
        let mut fixed = adj.iter().map(|(k, v)| (*k, v.len() as u32)).collect_vec();
        fixed.sort_by(|lhs, rhs| lhs.1.cmp(&rhs.1));
        fixed[fixed.len() / 2].0
    } else {
        0
    }
}

fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    env::set_var("RUST_BACKTRACE", "1");

    let edges = reader
        .lines()
        .filter_map(|line| line.ok())
        .map_while(|line| match line.is_empty() {
            true => None,
            false => Some(
                line.split("|")
                    .map(|substr| substr.parse::<u32>().unwrap())
                    .take(2)
                    .collect_vec(),
            ),
        })
        .collect_vec();

    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let seqs = reader
        .lines()
        .filter_map(|line| line.ok())
        .skip_while(|line| line.is_empty() || line.contains('|'))
        .map(|line| {
            line.split(",")
                .map(|ch| ch.parse::<u32>().unwrap())
                .collect_vec()
        })
        .collect_vec();

    let res = seqs
        .iter()
        .fold(0, |acc, seq| acc + fix_top_order(&seq, &edges));

    println!("{}", res);

    Ok(())
}

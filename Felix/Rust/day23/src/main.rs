use ::std::fs::File;
use std::{
    collections::HashMap,
    collections::HashSet,
    io::{self, prelude::*, BufReader},
};

use itertools::Itertools;

fn build_maximal_clique(
    src: &(char, char),
    adj: &HashMap<(char, char), HashSet<(char, char)>>,
) -> HashSet<(char, char)> {
    let mut clique = HashSet::new();
    clique.insert(*src);
    for dst in adj.get(src).unwrap().iter() {
        for member in clique.iter() {
            if !adj.get(&member).unwrap().contains(dst) {
                return clique;
            }
        }
        clique.insert(*dst);
    }
    clique
}

fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let adj = BufReader::new(file)
        .lines()
        .map(|line| line.unwrap().to_string())
        .map(|line| {
            line.split("-")
                .map(|subl| subl.to_owned())
                .collect_tuple()
                .map(|(src, dst)| {
                    (
                        src.chars().collect_tuple().unwrap(),
                        dst.chars().collect_tuple().unwrap(),
                    )
                })
        })
        .map(|connection| connection.unwrap())
        .fold(
            HashMap::<(char, char), HashSet<(char, char)>>::new(),
            |mut acc, (src, dst)| {
                acc.entry(src).or_default().insert(dst);
                acc.entry(dst).or_default().insert(src);
                acc
            },
        );

    let max_clique = adj.keys().fold(HashSet::new(), |acc, src| {
        let curr_max_clique = build_maximal_clique(src, &adj);
        if curr_max_clique.len() >= acc.len() {
            curr_max_clique
        } else {
            acc
        }
    });
    let mut max_clique = max_clique.iter().collect_vec();
    max_clique.sort();
    let res = max_clique
        .iter()
        .fold(String::new(), |mut acc, (src, dst)| {
            acc.push(*src);
            acc.push(*dst);
            acc.push(',');
            acc
        });

    println!("{:?}", res);

    Ok(())
}

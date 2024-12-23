#![allow(dead_code)]

use itertools::Itertools;
use std::cmp::{max, min};
use std::collections::{HashMap, HashSet};
use std::fs;

#[derive(Debug)]
struct NetworkMap {
    nodes: Vec<String>,
    edges: HashSet<(usize, usize)>,
}

impl NetworkMap {
    fn has_edge(&self, i: usize, j: usize) -> bool {
        self.edges.contains(&(min(i, j), max(i, j)))
    }
}

fn parse_input(filename: &str) -> NetworkMap {
    let file_contents = fs::read_to_string(filename).unwrap();
    let connections: Vec<(&str, &str)> = file_contents
        .lines()
        .map(|line| line.split("-").collect_tuple().unwrap())
        .collect_vec();
    let nodes = connections
        .iter()
        .flat_map(|(l, r)| [(*l).to_owned(), (*r).to_owned()])
        .unique()
        .collect_vec();
    let edges = connections
        .into_iter()
        .map(|(l, r)| {
            let li = nodes.iter().position(|x| x == l).unwrap();
            let ri = nodes.iter().position(|x| x == r).unwrap();
            (min(li, ri), max(li, ri))
        })
        .collect();
    NetworkMap { nodes, edges }
}

pub(crate) fn main() {
    let network_map = parse_input("day23_input.txt");
    println!("{:?}", network_map);

    let neighbours = {
        let mut m = HashMap::new();
        for edge in network_map.edges.iter() {
            m.entry(edge.0).or_insert(HashSet::new()).insert(edge.1);
            m.entry(edge.1).or_insert(HashSet::new()).insert(edge.0);
        }
        m
    };

    println!("{:?}", neighbours.get(&1).unwrap());

    let mut cliques = HashSet::new();
    for i in 0..network_map.nodes.len() {
        for j in 0..network_map.nodes.len() {
            if network_map.has_edge(i, j) {
                let ni = neighbours.get(&i).unwrap();
                let nj = neighbours.get(&j).unwrap();
                for &k in ni.intersection(nj) {
                    let mut clique = [i, j, k];
                    clique.sort();
                    cliques.insert(clique);
                }
            }
        }
    }

    let t_cliques = cliques
        .iter()
        .filter(|nodes| {
            nodes
                .iter()
                .any(|&n| network_map.nodes[n].chars().next().unwrap() == 't')
        })
        .map(|nodes| nodes.map(|i| &network_map.nodes[i]))
        .collect_vec();
    println!("{:?}", t_cliques.len());
}

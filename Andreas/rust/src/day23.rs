#![allow(dead_code)]

use itertools::Itertools;
use std::cmp::{max, min};
use std::collections::{HashMap, HashSet};
use std::fs;
use std::hash::Hash;

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

fn is_clique(network_map: &NetworkMap, nodes: &[usize]) -> bool {
    for i in 0..nodes.len() {
        for j in (i + 1)..nodes.len() {
            if !network_map.has_edge(nodes[i], nodes[j]) {
                return false;
            }
        }
    }
    true
}

fn cliques_of_size(
    network_map: &NetworkMap,
    neighbours: &[HashSet<usize>],
    size: usize,
) -> impl Iterator<Item=Vec<usize>> {
    fn rec(
        max_size: usize,
        clique: &mut Vec<usize>,
        neighbours: &[HashSet<usize>],
        f: &mut dyn FnMut(&Vec<usize>) -> (),
    ) {
        if clique.len() == max_size {
            f(clique);
        } else {
            let i = *clique.last().unwrap();
            for &j in neighbours[i].iter().filter(|&&j| j > i) {
                clique.push(j);
                rec(max_size, clique, neighbours, f);
                clique.pop();
            }
        }
    }

    let mut result = Vec::new();
    let mut clique = Vec::with_capacity(size);
    for i in 0..network_map.nodes.len() {
        clique.push(i);
        rec(size, &mut clique, &neighbours, &mut |clique| {
            if is_clique(network_map, clique) {
                result.push(clique.clone());
            }
        });
        clique.pop();
    }
    result.into_iter()
}

fn get_neighbours(network_map: &NetworkMap) -> Vec<HashSet<usize>> {
    let mut m = vec![HashSet::new(); network_map.nodes.len()];
    for edge in network_map.edges.iter() {
        m[edge.0].insert(edge.1);
        m[edge.1].insert(edge.0);
    }
    m
}

fn star1(network_map: &NetworkMap) {
    let cliques: HashSet<_> =
        cliques_of_size(&network_map, &get_neighbours(&network_map), 3).collect();
    let t_cliques = cliques
        .iter()
        .filter(|nodes| {
            nodes
                .iter()
                .any(|&n| network_map.nodes[n].chars().next().unwrap() == 't')
        })
        .map(|nodes| nodes.into_iter().map(|i| &network_map.nodes[*i]))
        .collect_vec();
    println!("Star 1: {}", t_cliques.len());
}

fn star2(network_map: &NetworkMap) {
    let neighbours = get_neighbours(&network_map);

    let mut right = 1;
    while right < network_map.nodes.len() + 1 {
        println!("{}", right);
        let has_clique = cliques_of_size(&network_map, &neighbours, right)
            .next()
            .is_some();
        if has_clique {
            right *= 2;
        } else {
            break;
        }
    }

    let mut left = 0;
    while left + 1 < right {
        println!("{} {}", left, right);
        let mid = (left + right) / 2;
        let has_clique = cliques_of_size(&network_map, &neighbours, mid)
            .next()
            .is_some();
        if has_clique {
            left = mid;
        } else {
            right = mid;
        }
    }
    let largest_clique = cliques_of_size(&network_map, &neighbours, left)
        .next()
        .unwrap();
    let largest_clique_names = largest_clique
        .into_iter()
        .map(|i| &network_map.nodes[i])
        .sorted()
        .collect_vec();
    println!("Star 2: {}", largest_clique_names.into_iter().join(","));
}

pub(crate) fn main() {
    let network_map = parse_input("day23_input.txt");
    star1(&network_map);
    star2(&network_map);
}

use ::std::fs::File;
use std::{
    collections::HashMap,
    collections::VecDeque,
    io::{self, prelude::*, BufReader},
};

#[derive(Debug)]
enum Operation {
    And(String, String, String),
    Or(String, String, String),
    Xor(String, String, String),
}

impl Operation {
    fn parse(line: &str) -> Self {
        line.split(" ")
            .collect_tuple()
            .map(|(lhs, op, rhs, _, target)| match op {
                "AND" => Operation::And(lhs.to_string(), rhs.to_string(), target.to_string()),
                "OR" => Operation::Or(lhs.to_string(), rhs.to_string(), target.to_string()),
                "XOR" => Operation::Xor(lhs.to_string(), rhs.to_string(), target.to_string()),
                _ => unreachable!(),
            })
            .unwrap()
    }

    fn eval_and_update(&self, results: &mut HashMap<String, u32>) {
        match self {
            Operation::And(lhs, rhs, target) => {
                let res = *results.get(lhs).unwrap() == 1 && *results.get(rhs).unwrap() == 1;
                results.insert(target.clone(), res as u32);
            }
            Operation::Or(lhs, rhs, target) => {
                let res = *results.get(lhs).unwrap() == 1 || *results.get(rhs).unwrap() == 1;
                results.insert(target.clone(), res as u32);
            }
            Operation::Xor(lhs, rhs, target) => {
                let res = *results.get(lhs).unwrap() != *results.get(rhs).unwrap();
                results.insert(target.clone(), res as u32);
            }
        };
    }
}

use itertools::Itertools;

fn topological_sort(operations: &Vec<Operation>) -> Vec<String> {
    let mut res = Vec::new();

    let mut indegree = HashMap::new();
    let mut adj = HashMap::new();
    for operation in operations {
        let (lhs, rhs, target) = match operation {
            Operation::And(lhs, rhs, target) => (lhs, rhs, target),
            Operation::Or(lhs, rhs, target) => (lhs, rhs, target),
            Operation::Xor(lhs, rhs, target) => (lhs, rhs, target),
        };
        if !indegree.contains_key(lhs) {
            indegree.insert(lhs, 0);
        }
        if !indegree.contains_key(rhs) {
            indegree.insert(rhs, 0);
        }
        indegree.insert(target, 0);
        if !lhs.starts_with('x') && !lhs.starts_with('y') {
            *indegree.entry(target).or_insert(0) += 1;
        }
        if !rhs.starts_with('x') && !rhs.starts_with('y') {
            *indegree.entry(target).or_insert(0) += 1;
        }
        adj.entry(lhs).or_insert(Vec::new()).push(target);
        adj.entry(rhs).or_insert(Vec::new()).push(target);
        if !adj.contains_key(target) {
            adj.insert(target, Vec::new());
        }
    }
    let mut dq = indegree
        .iter()
        .fold(VecDeque::new(), |mut acc, (wire, count)| {
            if *count == 0 {
                acc.push_back(*wire);
            }
            acc
        });

    while !dq.is_empty() {
        for _ in 0..dq.len() {
            let wire = dq.pop_front().unwrap();
            res.push(wire.clone());
            for reachable in adj.get(wire).unwrap() {
                *indegree.entry(reachable).or_default() -= 1;
                if indegree[reachable] == 0 {
                    dq.push_back(reachable);
                }
            }
        }
    }

    res
}

fn into_map(operations: Vec<Operation>) -> HashMap<String, Operation> {
    operations
        .into_iter()
        .fold(HashMap::new(), |mut acc, operation| {
            match operation {
                Operation::And(lhs, rhs, target) => {
                    acc.insert(target.clone(), Operation::And(lhs, rhs, target))
                }
                Operation::Or(lhs, rhs, target) => {
                    acc.insert(target.clone(), Operation::Or(lhs, rhs, target))
                }
                Operation::Xor(lhs, rhs, target) => {
                    acc.insert(target.clone(), Operation::Xor(lhs, rhs, target))
                }
            };
            acc
        })
}

fn evaluate(
    top_order: &Vec<String>,
    operations: &HashMap<String, Operation>,
    results: &mut HashMap<String, u32>,
) {
    for wire in top_order {
        if results.contains_key(wire) {
            continue;
        }
        let operation = operations.get(wire).unwrap();
        operation.eval_and_update(results);
    }
}

fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let input = BufReader::new(file)
        .lines()
        .map(|line| line.unwrap())
        .collect_vec();

    let mut eval =
        input
            .iter()
            .take_while(|line| !line.is_empty())
            .fold(HashMap::new(), |mut acc, line| {
                line.split(": ")
                    .collect_tuple()
                    .map(|(wire, value)| {
                        acc.insert(wire.to_string(), value.parse::<u32>().unwrap());
                        acc
                    })
                    .unwrap()
            });

    let operations = input
        .iter()
        .skip_while(|line| !line.contains("->"))
        .map(|line| Operation::parse(line))
        .collect_vec();

    let topological_order = topological_sort(&operations);
    let operations = into_map(operations);
    evaluate(&topological_order, &operations, &mut eval);

    let eval = eval
        .iter()
        .filter(|(wire, _)| wire.starts_with('z'))
        .sorted()
        .rev()
        .fold(String::new(), |acc, (_, value)| format!("{acc}{value}"));
    let res = u64::from_str_radix(&eval[..], 2).unwrap();

    println!("{}", res);

    Ok(())
}

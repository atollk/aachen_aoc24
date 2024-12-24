#![allow(dead_code)]

use itertools::Itertools;
use std::collections::HashMap;
use std::fs;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum LogicOperator {
    And,
    Or,
    Xor,
}

impl LogicOperator {
    fn apply(&self, lhs: bool, rhs: bool) -> bool {
        match self {
            LogicOperator::And => lhs && rhs,
            LogicOperator::Or => lhs || rhs,
            LogicOperator::Xor => lhs != rhs,
        }
    }
}

#[derive(Debug)]
struct Input {
    initial_values: HashMap<String, bool>,
    gates: Vec<(String, LogicOperator, String, String)>,
}

fn parse_input(filename: &str) -> Input {
    let file_contents = fs::read_to_string(filename).unwrap();
    let (part1, part2) = file_contents.split("\n\n").collect_tuple().unwrap();
    let initial_values = part1
        .lines()
        .map(|line| {
            let (var, val) = line.split(": ").collect_tuple().unwrap();
            (var.to_owned(), val == "1")
        })
        .collect();
    let gates = part2
        .lines()
        .map(|line| {
            let (lhs, op, rhs, _, res) = line.split(" ").collect_tuple().unwrap();
            let op = match op {
                "AND" => LogicOperator::And,
                "OR" => LogicOperator::Or,
                "XOR" => LogicOperator::Xor,
                _ => unreachable!(),
            };
            (lhs.to_owned(), op, rhs.to_owned(), res.to_owned())
        })
        .collect();
    Input {
        initial_values,
        gates,
    }
}

#[derive(Debug)]
struct DeviceLogic {
    variable_names: Vec<String>,
    initial_values: HashMap<usize, bool>,
    gates: Vec<((usize, usize), (LogicOperator, usize))>,
}

impl From<&Input> for DeviceLogic {
    fn from(value: &Input) -> Self {
        let variable_names = value
            .initial_values
            .keys()
            .chain(value.gates.iter().map(|g| &g.3))
            .unique()
            .map(|s| s.to_owned())
            .collect_vec();
        let initial_values = value
            .initial_values
            .iter()
            .map(|(var, val)| {
                (
                    variable_names.iter().position(|x| *x == *var).unwrap(),
                    *val,
                )
            })
            .collect();
        let gates = value
            .gates
            .iter()
            .map(|(lhs, op, rhs, res)| {
                (
                    (
                        variable_names.iter().position(|x| *x == *lhs).unwrap(),
                        variable_names.iter().position(|x| *x == *rhs).unwrap(),
                    ),
                    (*op, variable_names.iter().position(|x| *x == *res).unwrap()),
                )
            })
            .collect();
        DeviceLogic {
            variable_names,
            initial_values,
            gates,
        }
    }
}

impl DeviceLogic {
    fn initial_assignment(&self) -> Vec<Option<bool>> {
        let mut assignment = vec![None; self.variable_names.len()];
        for (var, val) in self.initial_values.iter() {
            assignment[*var] = Some(*val);
        }
        assignment
    }

    fn number_from_assignment(&self, assignment: &[Option<bool>]) -> u64 {
        let z_variable_indices = self
            .variable_names
            .iter()
            .enumerate()
            .filter(|(_, var)| var.starts_with("z"))
            .sorted_by_key(|(_, var)| var[1..].parse::<u8>().unwrap())
            .rev()
            .map(|(i, _)| i)
            .collect_vec();
        let binary = z_variable_indices
            .iter()
            .map(|i| if assignment[*i].unwrap() { '1' } else { '0' })
            .join("");
        u64::from_str_radix(&binary, 2).unwrap()
    }
}

pub(crate) fn main() {
    let input = parse_input("day24_input.txt");
    let device_logic = DeviceLogic::from(&input);
    println!("{:?}", device_logic);

    let mut open_gates = device_logic.gates.clone();
    let mut assignment = device_logic.initial_assignment();
    while !open_gates.is_empty() {
        open_gates.retain(|&((lhs, rhs), (op, res))| {
            if let Some(lhs_val) = assignment[lhs] {
                if let Some(rhs_val) = assignment[rhs] {
                    assignment[res] = Some(op.apply(lhs_val, rhs_val));
                    return false;
                }
            }
            true
        });
    }
    println!("{:?}", assignment);

    println!("{}", device_logic.number_from_assignment(&assignment));
}

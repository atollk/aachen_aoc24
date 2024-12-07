#![allow(dead_code)]

use gat_lending_iterator::LendingIterator;
use std::fs;

#[derive(Debug)]
struct Equation {
    sum: i64,
    operands: Vec<i64>,
}

impl Equation {
    fn eval(self: &Equation, operations: &[Operation]) -> i64 {
        let mut result = self.operands[0];
        for (operand, operation) in self.operands.iter().skip(1).zip(operations) {
            match *operation {
                Operation::Addition => {
                    result += operand;
                }
                Operation::Multiplication => {
                    result *= operand;
                }
                Operation::Concatenation => {
                    result = result * 10i64.pow(operand.ilog10() + 1) + operand;
                }
            }
        }
        result
    }
}

fn parse_input(filename: &str) -> Vec<Equation> {
    let file_contents = fs::read_to_string(filename).unwrap();
    file_contents
        .lines()
        .map(|line| {
            let equation_sides: [&str; 2] =
                line.split(": ").collect::<Vec<_>>().try_into().unwrap();
            let sum = equation_sides[0].parse().unwrap();
            let operands = equation_sides[1]
                .split(" ")
                .map(|op| op.parse().unwrap())
                .collect();
            Equation { sum, operands }
        })
        .collect()
}

#[derive(Debug, Clone, Copy)]
enum Operation {
    Addition,
    Multiplication,
    Concatenation,
}

struct OperationsIter {
    ops: Option<Vec<Operation>>,
    len: usize,
    include_concat: bool,
}

impl OperationsIter {
    fn new(len: usize, include_concat: bool) -> OperationsIter {
        OperationsIter {
            ops: None,
            len,
            include_concat,
        }
    }
}

impl LendingIterator for OperationsIter {
    type Item<'a>
        = &'a [Operation]
    where
        Self: 'a;

    fn next(&mut self) -> Option<Self::Item<'_>> {
        if let Some(ops) = self.ops.as_mut() {
            let mut exhausted = true;
            for op in ops.iter_mut() {
                match *op {
                    Operation::Addition => {
                        *op = Operation::Multiplication;
                        exhausted = false;
                        break;
                    }
                    Operation::Multiplication => {
                        if self.include_concat {
                            *op = Operation::Concatenation;
                            exhausted = false;
                            break;
                        } else {
                            Operation::Addition
                        };
                    }
                    Operation::Concatenation => {
                        *op = Operation::Addition;
                    }
                }
            }
            if exhausted {
                None
            } else {
                Some(self.ops.as_ref().unwrap())
            }
        } else {
            self.ops = Some(vec![Operation::Addition; self.len]);
            Some(self.ops.as_ref().unwrap())
        }
    }
}

fn has_valid_operands(equation: &Equation, mut operations_iter: OperationsIter) -> bool {
    while let Some(ops) = operations_iter.next() {
        let result = equation.eval(ops);
        if result == equation.sum {
            return true;
        }
    }
    false
}

fn sum_valid_equations(input: &[Equation], make_iter: &dyn Fn(&Equation) -> OperationsIter) -> i64 {
    input
        .iter()
        .filter(|eq| has_valid_operands(eq, make_iter(eq)))
        .map(|eq| eq.sum)
        .sum()
}

pub(crate) fn main() {
    let input = parse_input("day7_input.txt");
    println!(
        "Star 1: {}",
        sum_valid_equations(&input, &|eq| {
            OperationsIter::new(eq.operands.len() - 1, false)
        })
    );
    println!(
        "Star 2: {}",
        sum_valid_equations(&input, &|eq| {
            OperationsIter::new(eq.operands.len() - 1, true)
        })
    );
}

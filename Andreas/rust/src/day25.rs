#![allow(dead_code)]

use itertools::Itertools;
use std::cmp::PartialEq;
use std::fs;

#[derive(Debug, PartialEq, Eq)]
enum SchemaType {
    Lock,
    Key,
}

#[derive(Debug)]
struct Schema {
    typ: SchemaType,
    columns: Vec<usize>,
    height: usize,
}

fn parse_input(filename: &str) -> Vec<Schema> {
    let file_contents = fs::read_to_string(filename).unwrap();
    file_contents
        .split("\n\n")
        .map(|schema_string| {
            let width = schema_string.lines().next().unwrap().len();
            let height = schema_string.lines().count();
            let typ = if schema_string.lines().next().unwrap().contains('.') {
                SchemaType::Key
            } else {
                SchemaType::Lock
            };
            let mut columns = vec![0; width];
            for (y, line) in schema_string.lines().enumerate() {
                for (x, c) in line.chars().enumerate() {
                    if typ == SchemaType::Lock && c == '.' && columns[x] == 0 {
                        columns[x] = y;
                    } else if typ == SchemaType::Key && c == '#' && columns[x] == 0 {
                        columns[x] = height - y;
                    }
                }
            }
            Schema {
                typ,
                columns,
                height,
            }
        })
        .collect()
}

pub(crate) fn main() {
    let input = parse_input("day25_input.txt");

    let mut sum = 0;
    for (key, lock) in input.iter().cartesian_product(input.iter()) {
        if key.typ != SchemaType::Key || lock.typ != SchemaType::Lock {
            continue;
        }
        if key
            .columns
            .iter()
            .zip(lock.columns.iter())
            .all(|(key_height, lock_height)| key_height + lock_height <= key.height)
        {
            sum += 1;
        }
    }
    println!("{}", sum);
}

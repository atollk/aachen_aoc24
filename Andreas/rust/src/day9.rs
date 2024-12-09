#![allow(dead_code)]

use crate::day9::Block::Empty;
use itertools::{repeat_n, Itertools};
use std::fs;

fn parse_input(filename: &str) -> Vec<u8> {
    let file_contents = fs::read_to_string(filename).unwrap();
    file_contents
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u8)
        .collect()
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Block {
    Empty,
    File(usize),
}

fn pretty_print_blocks(blocks: &[Block]) -> String {
    let mut result = String::new();
    for b in blocks {
        match b {
            Block::Empty => {
                result += ".";
            }
            Block::File(i) => {
                result += &i.to_string();
            }
        }
    }
    result
}

fn expand_fs(block_sizes: &[u8]) -> Vec<Block> {
    #![allow(unstable_name_collisions)]
    let block_ids = (0..).map(|i| Block::File(i)).intersperse(Block::Empty);
    block_ids
        .zip(block_sizes)
        .flat_map(|(block, &size)| repeat_n(block, size as usize))
        .collect()
}

fn defragment(blocks: &mut [Block]) {
    let mut first_empty = blocks.iter().position(|b| *b == Block::Empty).unwrap();
    let mut last_file = blocks.len() - 1;
    loop {
        while first_empty < blocks.len() && blocks[first_empty] != Block::Empty {
            first_empty += 1;
        }
        while last_file > 0 && blocks[last_file] == Block::Empty {
            last_file -= 1;
        }
        if last_file == 0 || last_file < first_empty {
            break;
        }
        blocks.swap(first_empty, last_file);
        //println!("{} {} {}", pretty_print_blocks(blocks), first_empty, last_file);
    }
}

fn checksum(blocks: &[Block]) -> usize {
    blocks
        .iter()
        .filter_map(|b| match b {
            Empty => None,
            Block::File(i) => Some(*i),
        })
        .enumerate()
        .map(|(i, j)| i * j)
        .sum()
}

pub(crate) fn main() {
    let input = parse_input("day9_input.txt");

    let mut blocks = expand_fs(&input);
    defragment(&mut blocks);

    println!("{}", checksum(&blocks));
}

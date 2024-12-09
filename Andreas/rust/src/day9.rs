#![allow(dead_code)]

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

fn defragment_by_block(blocks: &mut [Block]) {
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

fn defragment_by_file(blocks: &mut [Block]) {
    //println!("{}", pretty_print_blocks(blocks),);
    let mut last_file_start = blocks.len();
    loop {
        // Shift to the next file to attempt to move.
        let mut last_file_end = last_file_start - 1;
        while last_file_end > 0 && blocks[last_file_end] == Block::Empty {
            last_file_end -= 1;
        }
        if last_file_end == 0 {
            break;
        }
        last_file_start = last_file_end - 1;
        while last_file_start > 0 && blocks[last_file_start] == blocks[last_file_end] {
            last_file_start -= 1;
        }
        if last_file_start == 0 {
            break;
        }
        last_file_end += 1;
        last_file_start += 1;

        // Find the first free block to move to.
        let mut first_empty_start = 1;
        let mut first_empty_end = 1;
        while first_empty_end < blocks.len() {
            if first_empty_end - first_empty_start >= last_file_end - last_file_start
            {
                break;
            }
            first_empty_end += 1;
            if let Some(Block::File(_)) = blocks.get(first_empty_end - 1) {
                first_empty_start = first_empty_end;
            }
        }

        if first_empty_end - first_empty_start >= last_file_end - last_file_start && first_empty_start < last_file_start {
            for i in 0..(last_file_end - last_file_start) {
                blocks.swap(first_empty_start + i, last_file_start + i);
            }
        }

        /*println!(
            "{} {}-{} {}-{}",
            pretty_print_blocks(blocks),
            first_empty_start,
            first_empty_end,
            last_file_start,
            last_file_end,
        );*/
    }
}

fn checksum(blocks: &[Block]) -> usize {
    blocks
        .iter()
        .enumerate()
        .filter_map(|(i, b)| match b {
            Block::Empty => None,
            Block::File(id) => Some(id * i),
        })
        .sum()
}

fn star1(input: &[u8]) {
    let mut blocks = expand_fs(&input);
    defragment_by_block(&mut blocks);
    println!("Star 1: {}", checksum(&blocks));
}

fn star2(input: &[u8]) {
    let mut blocks = expand_fs(&input);
    defragment_by_file(&mut blocks);
    println!("Star 2: {}", checksum(&blocks));
}

pub(crate) fn main() {
    let input = parse_input("day9_input.txt");
    star1(&input);
    star2(&input);
}

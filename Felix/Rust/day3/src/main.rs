use std::fs::File;
use std::io::{self, prelude::*, BufReader};

use regex::Regex;

use itertools::Itertools;

fn main() -> io::Result<()> {
    let re = Regex::new(r"(do\(\))()|(don't\(\))()|mul\(([0-9]{1,3}),([0-9]{1,3})\)").unwrap();
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let input = reader
        .lines()
        .fold_ok("".to_string(), |val, acc| (acc + &val).to_string())
        .unwrap();
    let res = re
        .captures_iter(&input)
        .fold((0, true), |(acc, enabled), cap| match cap.extract() {
            (_, [fst, snd]) => match fst {
                "do()" => (acc, true),
                "don't()" => (acc, false),
                _ => {
                    if enabled {
                        return (
                            acc + fst.parse::<u32>().unwrap() * snd.parse::<u32>().unwrap(),
                            enabled,
                        );
                    } else {
                        return (acc, enabled);
                    }
                }
            },
        });
    println!("{:?}", res);

    Ok(())
}

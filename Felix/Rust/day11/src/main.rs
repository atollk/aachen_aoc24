use itertools::Itertools;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn blink(val: u64, blinks: u64, memo: &mut HashMap<(u64, u64), u64>) -> u64 {
    if blinks == 75 {
        return 1;
    }

    if memo.contains_key(&(val, blinks)) {
        return memo[&(val, blinks)];
    }

    let digits = val.checked_ilog10().unwrap_or(0) + 1;
    let mut res = 0;
    if val == 0 {
        res += blink(1, blinks + 1, memo);
    } else if digits % 2 == 0 {
        let div = 10_u64.pow(digits / 2);
        res += blink(val / div, blinks + 1, memo);
        res += blink(val % div, blinks + 1, memo);
    } else {
        res += blink(2024 * val, blinks + 1, memo);
    }

    memo.insert((val, blinks), res);

    res
}

fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let mut reader = BufReader::new(file);

    let mut buf = String::new();
    let _ = reader.read_line(&mut buf);
    buf.pop();
    let input = buf
        .split(" ")
        .map(|s| s.parse::<u64>().unwrap())
        .collect_vec();

    let mut memo: HashMap<(u64, u64), u64> = HashMap::new();
    let res = input
        .iter()
        .fold(0, |acc, &val| acc + blink(val, 0, &mut memo));
    println!("{:?}", res);

    Ok(())
}

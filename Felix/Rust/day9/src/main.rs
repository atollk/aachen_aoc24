use std::fs::File;
use std::io::{self, prelude::*, BufReader};

use itertools::Itertools;

fn compress1(layout: &mut Vec<i32>) {
    let (mut l, mut r) = (0, layout.len() - 1);
    while l <= r - 2 {
        while layout[l] != -1 {
            l += 1;
        }
        layout.swap(l, r);
        r -= 1;
    }
}

fn compress2(pages: &Vec<(usize, usize)>, freespace: &mut Vec<(usize, usize)>) -> usize {
    let mut res = 0;
    let calc = |lb, ub| (ub - lb + 1) * (lb + ub) / 2;
    for (id, (start, size)) in pages.iter().enumerate().rev() {
        if let Some((fstart, fsize)) = freespace
            .iter_mut()
            .take_while(|(fstart, _)| fstart <= start)
            .find(|(_, fsize)| fsize >= size)
        {
            res += id * calc(*fstart, *fstart + size - 1);
            *fstart += size;
            *fsize -= size;
        } else {
            res += id * calc(*start, start + size - 1);
        }
    }
    res
}

fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let mut reader = BufReader::new(file);

    let mut buf = String::new();
    let _ = reader.read_line(&mut buf);
    buf.pop();
    buf.push('0');

    let layout = buf
        .chars()
        .tuples()
        .map(|(size, space)| {
            (
                size.to_digit(10).unwrap() as usize,
                space.to_digit(10).unwrap() as usize,
            )
        })
        .collect_vec();

    let mut freespace = layout
        .iter()
        .fold(
            (Vec::<(usize, usize)>::new(), 0),
            |(mut acc, prev), (size, space)| {
                if *space > 0 {
                    acc.push((prev + size, *space));
                }
                (acc, prev + size + space)
            },
        )
        .0;

    let pages = layout
        .iter()
        .fold(
            (Vec::<(usize, usize)>::new(), 0),
            |(mut acc, prev), (size, space)| {
                acc.push((prev, *size));
                (acc, prev + size + space)
            },
        )
        .0;

    println!("{:?}", compress2(&pages, &mut freespace));

    Ok(())
}

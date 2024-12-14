use std::fs::File;
use std::io::{self, prelude::*, BufReader};

use itertools::Itertools;

#[derive(Debug)]
struct Coord(i64, i64);

#[derive(Debug)]
struct Game {
    a: Coord,
    b: Coord,
    goal: Coord,
}

impl Game {
    fn new(a: Coord, b: Coord, goal: Coord) -> Self {
        Self { a, b, goal }
    }

    fn solve(&self) -> i64 {
        let det = self.a.0 * self.b.1 - self.a.1 * self.b.0;
        let m = (self.b.1 * self.goal.0 - self.b.0 * self.goal.1) / det;
        let n = (-self.a.1 * self.goal.0 + self.a.0 * self.goal.1) / det;
        if m * self.a.0 + n * self.b.0 == self.goal.0 && m * self.a.1 + n * self.b.1 == self.goal.1
        {
            m * 3 + n * 1
        } else {
            0
        }
    }
}

fn parse_line(s: &String) -> (i64, i64) {
    let res = s
        .split([',', '+', '='])
        .map(|substr| substr.parse::<i64>())
        .flatten()
        .collect_vec();
    (res[0], res[1])
}

fn parse(data: &Vec<String>) -> Vec<Game> {
    data.iter()
        .tuples()
        .map(|(a, b, goal, _)| {
            let (dx1, dy1) = parse_line(&a);
            let (dx2, dy2) = parse_line(&b);
            let (x, y) = parse_line(&goal);
            Game::new(
                Coord(dx1, dy1),
                Coord(dx2, dy2),
                Coord(x + 10000000000000, y + 10000000000000),
            )
        })
        .collect_vec()
}

fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let input = reader.lines().map(|line| line.unwrap()).collect_vec();
    let games = parse(&input);
    let res = games.iter().fold(0, |acc, game| acc + game.solve());

    println!("{:?}", res);

    Ok(())
}

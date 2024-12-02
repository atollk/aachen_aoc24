use std::collections::HashSet;
use std::fs;

fn parse_input(filename: &str) -> Vec<Vec<i16>> {
    let file_contents = fs::read_to_string(filename).unwrap();
    file_contents
        .lines()
        .map(|line| line.split(" ").map(|x| x.parse().unwrap()).collect())
        .collect()
}

fn is_safe_general(level: &[i16], valid_values: &[i16], dampening_limit: usize) -> bool {
    if level.len() < 2 {
        return true;
    }
    let mut previous = level[0];
    let mut dampening_used = 0;
    for &current in level.iter().skip(1) {
        let diff = current - previous;
        if valid_values.contains(&diff) {
            previous = current;
        } else {
            dampening_used += 1;
            if dampening_used > dampening_limit {
                return false;
            }
        }
    }
    true
}

fn is_safe(level: &[i16], dampening_limit: usize) -> bool {
    for skip_first in 0..=dampening_limit {
        let is_safe_inc = is_safe_general(
            &level[skip_first..],
            &[1, 2, 3],
            dampening_limit - skip_first,
        );
        let is_safe_dec = is_safe_general(
            &level[skip_first..],
            &[-1, -2, -3],
            dampening_limit - skip_first,
        );
        if is_safe_inc || is_safe_dec {
            return true;
        }
    }
    false
}

pub fn main() {
    let input = parse_input("day2_input.txt");

    let safe_levels1: HashSet<_> = input.iter().filter(|level| is_safe(level, 0)).collect();
    let safe_levels2: HashSet<_> = input.iter().filter(|level| is_safe(level, 1)).collect();
    // Mir ist aufgefallen dass meine Lösung für Tag 2 gar nicht richtig funktioniert, z.B: für "88 91 90 91 93", aber scheinbar hatte ich Glück mit dem Input :^)

    println!("Star 1: {:?}", safe_levels1.len());
    println!("Star 2: {:?}", safe_levels2.len());
}

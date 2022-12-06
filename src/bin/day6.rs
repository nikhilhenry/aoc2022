use itertools::Itertools;
use std::collections::HashSet;

fn main() {
    let input = include_str!("../../data/day6.input").trim();
    let input = input.chars().collect_vec();
    for idx in 0..input.len() {
        if idx + 14 == input.len() - 1 {
            break;
        }
        let marker: HashSet<char> = input[idx..idx + 14].iter().map(|c| *c).collect();
        if marker.len() == 14 {
            println!("Part 1: {}", idx + 14);
            break;
        }
    }
}

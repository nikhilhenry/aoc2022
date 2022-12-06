use itertools::Itertools;
use std::collections::HashSet;

fn find_marker(s: &Vec<char>, size: usize) -> Option<usize> {
    for idx in 0..s.len() {
        if idx + size == s.len() - 1 {
            break;
        }
        let marker: HashSet<char> = s[idx..idx + size].iter().map(|c| *c).collect();
        if marker.len() == size {
            return Some(idx + size);
        }
    }
    return None;
}
fn main() {
    let input = include_str!("../../data/day6.input").trim();
    let input = input.chars().collect_vec();
    if let Some(ans) = find_marker(&input, 4) {
        println!("Part 1: {}", ans);
    }
    if let Some(ans) = find_marker(&input, 14) {
        println!("Part 1: {}", ans);
    }
}

use anyhow::Result;
use aoc;
use itertools::Itertools;
use std::collections::HashSet;

trait Priority {
    fn compute_priority(&self) -> u32;
}

impl Priority for char {
    fn compute_priority(&self) -> u32 {
        if self.is_uppercase() {
            return *self as u32 - 64 + 26;
        } else {
            return *self as u32 - 96;
        }
    }
}

fn total_priority() -> Result<u32> {
    Ok(aoc::read_one_per_line::<String>("./data/day3.input")?
        .iter()
        .map(|line| line.split_at(line.len() / 2))
        .flat_map(|(split_1, split_2)| {
            split_1
                .chars()
                .collect::<HashSet<char>>()
                .into_iter()
                .merge(split_2.chars().collect::<HashSet<char>>())
                .duplicates()
                .nth(0)
        })
        .map(|ch| ch.compute_priority())
        .sum())
}

fn main() -> Result<()> {
    println!("Part 1:{}", total_priority()?);
    Ok(())
}

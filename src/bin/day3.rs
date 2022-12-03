use anyhow::Result;
use aoc;
use itertools::Itertools;
use std::collections::HashSet;

fn main() -> Result<()> {
    let mut total_score = 0;
    let lines = aoc::read_one_per_line::<String>("./data/day3.input")?;
    for line in lines {
        let mut set_1: HashSet<char> = HashSet::new();
        let split_1 = &line[0..line.len() / 2];
        for ch in split_1.chars() {
            set_1.insert(ch);
        }
        let mut set_2: HashSet<char> = HashSet::new();
        let split_2 = &line[line.len() / 2..line.len()];
        for ch in split_2.chars() {
            set_2.insert(ch);
        }
        let mut sack: Vec<char> = Vec::new();
        sack.extend(set_1);
        sack.extend(set_2);
        let item = sack.into_iter().duplicates().collect_vec();
        if item == vec![] {
            continue;
        }
        let item = item[0];
        //println!("{:?}", item);
        let score: u32;
        if item.is_uppercase() {
            score = item as u32 - 64 + 26;
        } else {
            score = item as u32 - 96;
        }
        total_score += score;
    }
    println!("{}", total_score);
    Ok(())
}

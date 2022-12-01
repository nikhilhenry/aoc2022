use anyhow::Result;
use aoc;
use itertools::Itertools;
fn max_calories(top: usize) -> Result<u32> {
    Ok(aoc::read_one_per_line::<String>("./data/day1.input")?
        .into_iter()
        .group_by(|elt| elt.trim() != "")
        .into_iter()
        .filter(|(ge0, _group)| *ge0)
        .map(|(_ge0, group)| group.map(|el| el.parse::<u32>().unwrap()).collect_vec())
        .map(|g| g.iter().sum::<u32>())
        .sorted()
        .rev()
        .take(top)
        .sum())
}
fn main() -> Result<()> {
    println!("Part 1: {}", max_calories(1)?);
    println!("Part 2: {}", max_calories(3)?);
    Ok(())
}

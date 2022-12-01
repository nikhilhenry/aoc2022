use anyhow::Result;
use aoc;
use itertools::Itertools;
fn max_calories() -> Result<u32> {
    Ok(aoc::read_one_per_line::<String>("./data/day1.input")?
        .into_iter()
        .group_by(|elt| elt.trim() != "")
        .into_iter()
        .filter(|(ge0, _group)| *ge0)
        .map(|(_ge0, group)| group.map(|el| el.parse::<u32>().unwrap()).collect_vec())
        .map(|g| g.iter().sum::<u32>())
        .sorted()
        .last()
        .unwrap())
}
fn main() -> Result<()> {
    println!("Part 1: {}", max_calories()?);
    Ok(())
}

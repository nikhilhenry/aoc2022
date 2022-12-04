use anyhow::Result;
use aoc;
use std::ops::RangeInclusive;

trait TupleInt {
    fn parse_range(self: &Self) -> RangeInclusive<u32>;
}

impl TupleInt for (&str, &str) {
    fn parse_range(self: &Self) -> RangeInclusive<u32> {
        self.0.trim().parse::<u32>().unwrap()..=self.1.trim().parse::<u32>().unwrap()
    }
}

trait RangeComparable {
    fn contains_within(self: &Self, other: &RangeInclusive<u32>) -> bool;
    fn overlaps(self: &Self, other: &RangeInclusive<u32>) -> bool;
}

impl RangeComparable for RangeInclusive<u32> {
    fn contains_within(self: &Self, other: &RangeInclusive<u32>) -> bool {
        self.contains(&other.start()) && self.contains(&other.end())
    }
    fn overlaps(self: &Self, other: &RangeInclusive<u32>) -> bool {
        self.start() <= other.end() && other.start() <= self.end()
    }
}
fn count_ids(filter: &dyn Fn((RangeInclusive<u32>, RangeInclusive<u32>)) -> bool) -> Result<usize> {
    Ok(aoc::read_one_per_line::<String>("./data/day4.input")?
        .iter()
        .flat_map(|block| block.split_once(","))
        .map(|(r1, r2)| {
            (
                r1.split_once("-").unwrap().parse_range(),
                r2.split_once("-").unwrap().parse_range(),
            )
        })
        .filter(|val| filter(val.to_owned()))
        .count())
}

fn main() -> Result<()> {
    println!(
        "Part 1: {}",
        count_ids(&|(r1, r2)| { r1.contains_within(&r2) || r2.contains_within(&r1) })?
    );
    println!(
        "Part 2: {}",
        count_ids(&|(r1, r2)| { r1.overlaps(&r2) || r2.overlaps(&r1) })?
    );
    Ok(())
}

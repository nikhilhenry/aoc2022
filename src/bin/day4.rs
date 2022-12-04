use anyhow::Result;
use aoc;
use itertools::Itertools;
use std::ops::RangeInclusive;

trait TupleInt {
    fn parse_int_tuple(self: &Self) -> (u32, u32);
}

impl TupleInt for (&str, &str) {
    fn parse_int_tuple(self: &Self) -> (u32, u32) {
        (
            self.0.trim().parse::<u32>().unwrap(),
            self.1.trim().parse::<u32>().unwrap(),
        )
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
fn count_overlaps() -> Result<usize> {
    Ok(aoc::read_one_per_line::<String>("./data/day4.input")?
        .iter()
        .flat_map(|block| block.split_once(","))
        .filter(|(r1, r2)| {
            let r1 = r1.split_once("-").unwrap().parse_int_tuple();
            let r2 = r2.split_once("-").unwrap().parse_int_tuple();
            let r1 = [r1.0..=r1.1];
            let r2 = [r2.0..=r2.1];
            r1[0].overlaps(&r2[0]) || r2[0].overlaps(&r1[0])
        })
        .count())
}
fn count_contains() -> Result<usize> {
    Ok(aoc::read_one_per_line::<String>("./data/day4.input")?
        .iter()
        .flat_map(|block| block.split_once(","))
        //.map(|(l1, l2)| (l1.split_once(" ").unwrap().1, l2.split_once(" ").unwrap().1))
        .filter(|(r1, r2)| {
            let r1 = r1.split_once("-").unwrap().parse_int_tuple();
            let r2 = r2.split_once("-").unwrap().parse_int_tuple();
            let r1 = [r1.0..=r1.1];
            let r2 = [r2.0..=r2.1];
            r1[0].contains_within(&r2[0]) || r2[0].contains_within(&r1[0])
        })
        .count())
}

fn main() -> Result<()> {
    println!("Part 1: {}", count_contains()?);
    println!("Part 2: {}", count_overlaps()?);
    Ok(())
}

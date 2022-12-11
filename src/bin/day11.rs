use anyhow::anyhow;
use anyhow::Result;
use aoc;
use std::str::FromStr;

#[derive(Debug, Clone)]
enum Operation {
    Add(u64),
    Multiply(u64),
    Square,
}

impl Operation {
    fn execute(&self, val: u64) -> u64 {
        match self {
            Operation::Add(mag) => val + mag,
            Operation::Multiply(mag) => val * mag,
            Operation::Square => val * val,
        }
    }
}
#[derive(Debug, Clone)]
struct Monkey {
    items: Vec<u64>,
    operation: Operation,
    test: u64,
    success: usize,
    failure: usize,
    inspected: u64,
}

impl FromStr for Operation {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut input = s.split("=");
        input.next();
        let mut input = input
            .next()
            .ok_or(anyhow!("Unable to parse operation"))?
            .split_whitespace();
        input.next();
        match input.next() {
            Some("+") => Ok(Operation::Add(input.next().unwrap().parse()?)),
            Some("*") => match input.next() {
                Some("old") => Ok(Operation::Square),
                Some(val) => Ok(Operation::Multiply(val.parse()?)),
                _ => panic!("how did we get here?"),
            },
            _ => panic!("how did we get here?"),
        }
    }
}

impl FromStr for Monkey {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self> {
        let mut input = s.split('\n');
        input.next();
        let items = input
            .next()
            .ok_or(anyhow!("Unable to parse Monkey"))?
            .split(":")
            .skip(1)
            .next()
            .unwrap()
            .split(",")
            .map(|num| num.trim().parse::<u64>().expect("items must be numbers"))
            .collect::<Vec<_>>();
        let operation = input
            .next()
            .ok_or(anyhow!("unable to parse test"))?
            .parse::<Operation>()
            .expect("should parse operation");
        let test = input
            .next()
            .ok_or(anyhow!("unable to parse test"))?
            .trim()
            .split_whitespace()
            .last()
            .ok_or(anyhow!("unable to parse test"))?
            .parse::<u64>()
            .expect("should be a number");
        let success = input
            .next()
            .ok_or(anyhow!("unable to parse test"))?
            .trim()
            .split_whitespace()
            .last()
            .ok_or(anyhow!("unable to parse test"))?
            .parse::<usize>()?;
        let failure = input
            .next()
            .ok_or(anyhow!("unable to parse test"))?
            .trim()
            .split_whitespace()
            .last()
            .ok_or(anyhow!("unable to parse test"))?
            .parse::<usize>()?;
        Ok(Monkey {
            items,
            operation,
            test,
            success,
            failure,
            inspected: 0,
        })
    }
}

fn main() -> Result<()> {
    let mut monkeys = aoc::read_one_per_block::<Monkey>("data/day11.input")?;
    let lcm: u64 = monkeys.iter().map(|m| m.test).product();
    for _ in 0..10000 {
        for idx in 0..monkeys.len() {
            let mut monkey_copy = monkeys[idx].clone();
            for (jdx, item) in monkey_copy.items.iter_mut().enumerate() {
                monkeys[idx].inspected += 1;
                let item = monkey_copy.operation.execute(*item) % lcm;
                if item % monkey_copy.test == 0 {
                    let success = monkey_copy.success;
                    // transfering to the other monkey
                    monkeys[success].items.push(item);
                    monkeys[idx].items.remove(0);
                } else {
                    let failure = monkey_copy.failure;
                    // transfering to the other monkey
                    monkeys[failure].items.push(item);
                    monkeys[idx].items.remove(0);
                }
            }
        }
    }
    let mut monkey_scores: Vec<_> = monkeys.into_iter().map(|m| m.inspected).collect();
    dbg!(&monkey_scores);
    monkey_scores.sort();
    let monkey_business: u64 = monkey_scores.iter().rev().take(2).product();
    println!("{}", monkey_business);
    Ok(())
}

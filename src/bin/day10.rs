use anyhow::anyhow;
use anyhow::Result;
use aoc;
use itertools::Itertools;
use std::str::FromStr;

#[derive(Debug)]
enum Instruction {
    Noop,
    AddX(i32),
}

impl FromStr for Instruction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut cmd = s.split_whitespace();
        match cmd.next() {
            Some("noop") => Ok(Instruction::Noop),
            Some("addx") => Ok(Instruction::AddX(
                cmd.next().expect("needs to be present").parse()?,
            )),
            _ => Err(anyhow!("how did we get here")),
        }
    }
}

fn value_during_cycle(instructions: &Vec<Instruction>, at: usize) -> i32 {
    let mut cycle_count = 1;
    let mut total = 1;
    for instruction in instructions {
        if cycle_count >= at - 1 {
            break;
        }
        match instruction {
            Instruction::Noop => {
                cycle_count += 1;
            }
            Instruction::AddX(mag) => {
                cycle_count += 2;
                total += mag;
            }
        }
    }
    return total;
}
fn main() -> Result<()> {
    let input = aoc::read_one_per_line::<Instruction>("data/day10.input")?;
    let val_20 = value_during_cycle(&input, 20) * 20;
    let val_60 = value_during_cycle(&input, 60) * 60;
    let val_100 = value_during_cycle(&input, 100) * 100;
    let val_140 = value_during_cycle(&input, 140) * 140;
    let val_180 = value_during_cycle(&input, 180) * 180;
    let val_220 = value_during_cycle(&input, 220) * 220;
    dbg!(value_during_cycle(&input, 220));
    dbg!(val_20);
    dbg!(val_60);
    dbg!(val_100);
    dbg!(val_140);
    dbg!(val_180);
    dbg!(val_220);
    println!(
        "{}",
        val_20 + val_60 + val_100 + val_140 + val_180 + val_220
    );
    Ok(())
}

use anyhow::anyhow;
use anyhow::Context;
use anyhow::Result;
use aoc;
use std::str::FromStr;

struct Coord {
    x: u32,
    y: u32,
}

#[derive(Debug)]
enum Instruction {
    R(u32),
    L(u32),
    D(u32),
    U(u32),
}

impl FromStr for Instruction {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Instruction> {
        let (dir, mag) = s.split_once(" ").context("Need dir and mag")?;
        match dir {
            "R" => Ok(Instruction::R(mag.parse()?)),
            "L" => Ok(Instruction::L(mag.parse()?)),
            "D" => Ok(Instruction::D(mag.parse()?)),
            "U" => Ok(Instruction::U(mag.parse()?)),
            _ => Err(anyhow!("Invalid direction")),
        }
    }
}
fn main() -> Result<()> {
    let mut head = Coord { x: 0, y: 0 };
    let mut tail = Coord { x: 0, y: 0 };

    // moving the head according to the input
    let instructions = aoc::read_one_per_line::<Instruction>("./data/day9.sample")?;
    for instruction in instructions {
        dbg!(instruction);
    }
    Ok(())
}

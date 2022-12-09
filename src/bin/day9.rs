use anyhow::anyhow;
use anyhow::Result;
use aoc;
use itertools::Itertools;
use std::str::FromStr;

#[derive(Debug)]
enum Direction {
    Up(u32),
    Down(u32),
    Left(u32),
    Right(u32),
}

impl FromStr for Direction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        if let Some((dir, mag)) = s.split_once(" ") {
            let mag: u32 = mag.parse().expect("nums?");
            match dir {
                "U" => Ok(Direction::Up(mag)),
                "D" => Ok(Direction::Down(mag)),
                "L" => Ok(Direction::Left(mag)),
                "R" => Ok(Direction::Right(mag)),
                _ => Err(anyhow!("this is hard")),
            }
        } else {
            Err(anyhow!("this is hard"))
        }
    }
}

#[derive(Debug)]
struct Coord {
    x: u32,
    y: u32,
}

impl Coord {
    fn new() -> Self {
        Coord { x: 0, y: 0 }
    }
    fn move_me(&mut self, dir: Direction) {
        match dir {
            Direction::Up(v) => self.y += v,
            Direction::Down(v) => self.y -= v,
            Direction::Left(v) => self.x -= v,
            Direction::Right(v) => self.x += v,
        }
    }
}

fn main() -> Result<()> {
    let input = aoc::read_one_per_line::<Direction>("data/day9.sample")?;
    let mut head = Coord::new();
    let tail = Coord::new();
    for dir in input {
        println!("{:?}", dir);
        head.move_me(dir);
        println!("{:?}", head);
    }
    Ok(())
}

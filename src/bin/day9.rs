use anyhow::anyhow;
use anyhow::Result;
use aoc;
use itertools::Itertools;
use std::collections::HashSet;
use std::str::FromStr;

#[derive(Debug)]
enum Direction {
    Up(i32),
    Down(i32),
    Left(i32),
    Right(i32),
    Diag((i32, i32)),
}

impl FromStr for Direction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        if let Some((dir, mag)) = s.split_once(" ") {
            let mag: i32 = mag.parse().expect("nums?");
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

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
struct Coord {
    x: i32,
    y: i32,
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
            Direction::Diag((x, y)) => {
                self.x += x;
                self.y += y;
            }
        }
    }
    fn should_i_move(&self, head: &Coord) -> Option<Direction> {
        let x_diff = head.x - self.x;
        let y_diff = head.y - self.y;
        // we have to move diagonally
        if x_diff.abs() == 1 && y_diff.abs() > 1 {
            // we should change the horizontal thing we are working on
            if y_diff > 1 {
                return Some(Direction::Diag((x_diff as i32, 1)));
            }
            return Some(Direction::Diag((x_diff as i32, -1)));
        }
        if x_diff.abs() > 1 && y_diff.abs() == 1 {
            // we should change the vertical thing we are working on
            if x_diff > 1 {
                return Some(Direction::Diag((1, y_diff as i32)));
            }
            return Some(Direction::Diag((-1, y_diff as i32)));
        }
        if x_diff > 1 {
            return Some(Direction::Right(1));
        }
        if x_diff < -1 {
            return Some(Direction::Left(1));
        }
        if y_diff > 1 {
            return Some(Direction::Up(1));
        }
        if y_diff < -1 {
            return Some(Direction::Down(1));
        }
        return None;
    }
    fn follow(&mut self, head: &Coord) -> Vec<Coord> {
        let mut coords = Vec::new();
        while let Some(dir) = self.should_i_move(head) {
            self.move_me(dir);
            coords.push(self.clone());
        }
        return coords;
    }
}

fn main() -> Result<()> {
    let input = aoc::read_one_per_line::<Direction>("data/day9.input")?;
    let mut head = Coord::new();
    let mut tail = Coord::new();
    let mut visited = HashSet::new();
    let mut coords = vec![tail.clone()];
    visited.insert(tail.clone());
    for dir in input {
        head.move_me(dir);
        for coord in tail.follow(&head) {
            coords.push(coord)
        }
    }
    //dbg!(&coords);
    println!("Part 1: {}", coords.iter().unique().count());
    Ok(())
}

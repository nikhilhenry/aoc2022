use anyhow::Result;
use aoc;
use itertools::Itertools;
use std::str::FromStr;

enum Move1 {
    A,
    B,
    C,
}

enum Move2 {
    X(u32),
    Y(u32),
    Z(u32),
}
struct Round {
    player_1: Move1,
    player_2: Move2,
}

impl FromStr for Round {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self> {
        if let Some((player_1, player_2)) = s.split_once(" ") {
            let player_1 = match player_1 {
                "A" => Move1::A,
                "B" => Move1::B,
                "C" => Move1::C,
                _ => panic!(),
            };
            let player_2 = match player_2 {
                "X" => Move2::X(1),
                "Y" => Move2::Y(2),
                "Z" => Move2::Z(3),
                _ => panic!(),
            };
            Ok(Round { player_1, player_2 })
        } else {
            Err(anyhow::format_err!("could split round"))
        }
    }
}

fn compute_score_1(round: &Round) -> u32 {
    match round.player_1 {
        Move1::A => match &round.player_2 {
            Move2::Y(val) => 6 + val,
            Move2::X(val) => 3 + val,
            Move2::Z(val) => *val,
        },
        Move1::B => match &round.player_2 {
            Move2::Z(val) => 6 + val,
            Move2::Y(val) => 3 + val,
            Move2::X(val) => *val,
        },
        Move1::C => match &round.player_2 {
            Move2::X(val) => 6 + val,
            Move2::Z(val) => 3 + val,
            Move2::Y(val) => *val,
        },
    }
}
fn calculate_score() -> Result<u32> {
    Ok(aoc::read_one_per_line::<Round>("./data/day2.input")?
        .iter()
        .map(|round| compute_score_1(round))
        .sum())
}
fn main() -> Result<()> {
    Ok(println!("Part 1: {}", calculate_score()?))
}

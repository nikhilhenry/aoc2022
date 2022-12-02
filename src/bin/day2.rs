use anyhow::Result;
use aoc;
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
                "X" => Move2::X(0),
                "Y" => Move2::Y(3),
                "Z" => Move2::Z(6),
                _ => panic!(),
            };
            Ok(Round { player_1, player_2 })
        } else {
            Err(anyhow::format_err!("could split round"))
        }
    }
}

fn compute_score_2(round: &Round) -> u32 {
    match round.player_1 {
        // rock
        Move1::A => match &round.player_2 {
            Move2::Z(val) => 2 + val,
            Move2::Y(val) => 1 + val,
            Move2::X(_val) => 3,
        },
        // paper
        Move1::B => match &round.player_2 {
            Move2::Z(val) => 3 + val,
            Move2::Y(val) => 2 + val,
            Move2::X(_val) => 1,
        },
        // scissor
        Move1::C => match &round.player_2 {
            Move2::Z(val) => 1 + val,
            Move2::Y(val) => 3 + val,
            Move2::X(_val) => 2,
        },
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
fn calculate_score(score_method: &dyn Fn(&Round) -> u32) -> Result<u32> {
    Ok(aoc::read_one_per_line::<Round>("./data/day2.input")?
        .iter()
        .map(|round| score_method(round))
        .sum())
}
fn main() -> Result<()> {
    println!("Part 1: {}", calculate_score(&compute_score_1)?);
    println!("Part 2: {}", calculate_score(&compute_score_2)?);
    Ok(())
}

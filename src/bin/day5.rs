use anyhow::Result;

fn parse_stack() -> Vec<Vec<char>> {
    let input = include_str!("../../data/day5.sample");
    let (stack, moves) = input.split_once("\n\n").expect("aoc is wrong??");
    let stack: Vec<_> = stack.split("\n").map(|s| String::from(s)).collect();
    let num_stacks = stack
        .iter()
        .rev()
        .next()
        .unwrap()
        .split(" ")
        .flat_map(|val| val.parse::<usize>())
        .last()
        .unwrap();

    let stack = &stack[..stack.len() - 1];
    // construct the 2D vec
    let mut crate_stack: Vec<Vec<char>> = vec![vec![' '; num_stacks * 2]; num_stacks];
    for (col, row) in stack.iter().rev().enumerate() {
        let row = &row[1..];
        for (ix, c) in row.char_indices() {
            if ix % 4 == 0 {
                crate_stack[ix / 4][col] = c;
            }
        }
    }
    // clean the stack
    for stack in crate_stack.iter_mut() {
        stack.retain(|val| val.is_alphabetic());
    }
    return crate_stack;
}

fn main() -> Result<()> {
    println!("{:?}", parse_stack());
    Ok(())
}

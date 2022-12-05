use anyhow::Result;

fn parse_stack(input: &str) -> Vec<Vec<char>> {
    let stack: Vec<_> = input.split("\n").map(|s| String::from(s)).collect();
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

fn parse_moves(input: &str) -> Vec<(usize, usize, usize)> {
    input
        .split("\n")
        .map(|set| {
            set.split(" ")
                .flat_map(|c| c.parse::<usize>())
                .collect::<Vec<_>>()
        })
        .filter(|set| !set.is_empty())
        .map(|set| (set[0], set[1], set[2]))
        .collect()
}

fn main() -> Result<()> {
    let input = include_str!("../../data/day5.input");
    let (stack, moves) = input.split_once("\n\n").expect("aoc is wrong??");
    let mut crate_stack = parse_stack(stack);
    let move_set = parse_moves(moves);
    for (amount, from, to) in move_set.iter() {
        let offset = crate_stack[from - 1].len() - amount;
        let mut temp_set: Vec<_> = crate_stack[from - 1].drain(offset..).rev().collect();
        crate_stack[to - 1].append(&mut temp_set);
    }
    let result: String = crate_stack.iter().flat_map(|col| col.last()).collect();
    println!("{}", result);
    Ok(())
}

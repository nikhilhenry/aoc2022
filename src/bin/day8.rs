use anyhow::Result;
use aoc;
use itertools::Itertools;

#[derive(Debug)]
struct Coord {
    row: usize,
    col: usize,
}

enum Direction {
    Top,
    Bot,
    Left,
    Right,
}

impl Coord {
    fn get_val(&self, matrix: &Vec<Vec<u32>>) -> Option<u32> {
        match matrix.iter().nth(self.row) {
            Some(row) => row.iter().nth(self.col).copied(),
            None => None,
        }
    }

    fn get_cum_val(&self, matrix: &Vec<Vec<u32>>, dir: Direction) -> u32 {
        match dir {
            Direction::Top => (0..self.row)
                .enumerate()
                .filter_map(|(v, _)| {
                    Coord {
                        row: self.row - v,
                        col: self.col,
                    }
                    .get_val(matrix)
                })
                .sum(),
            Direction::Bot => (self.row..matrix.len())
                .enumerate()
                .filter_map(|(v, _)| {
                    Coord {
                        row: self.row + v,
                        col: self.col,
                    }
                    .get_val(matrix)
                })
                .sum(),
            Direction::Left => (0..self.col + 1)
                .enumerate()
                .filter_map(|(v, _)| {
                    Coord {
                        row: self.row,
                        col: self.col - v,
                    }
                    .get_val(matrix)
                })
                .sum(),
            Direction::Right => (self.col..matrix[0].len())
                .enumerate()
                .filter_map(|(v, _)| {
                    Coord {
                        row: self.row,
                        col: self.col + v,
                    }
                    .get_val(matrix)
                })
                .sum(),
        }
    }

    fn am_i_visible(&self, matrix: &Vec<Vec<u32>>) -> bool {
        let my_val = self.get_val(matrix).expect("how did we get here");
        let neighbours = vec![
            Coord {
                // top
                row: self.row - 1,
                col: self.col,
            }
            .get_cum_val(matrix, Direction::Top),
            Coord {
                row: self.row + 1,
                col: self.col,
            }
            .get_cum_val(matrix, Direction::Bot),
            Coord {
                row: self.row,
                col: self.col + 1,
            }
            .get_cum_val(matrix, Direction::Right),
            Coord {
                row: self.row,
                col: self.col - 1,
            }
            .get_cum_val(matrix, Direction::Left),
        ];
        //println!("{:?} : {:?}", self, neighbours);
        for val in neighbours {
            if val < my_val {
                return true;
            }
            // i dont have a neighbour I am an edge value
        }
        return false;
    }
}

fn main() -> Result<()> {
    let input = aoc::read_one_per_line::<String>("./data/day8.sample")?;
    let input = input
        .iter()
        .filter(|s| !s.is_empty())
        .map(|s| s.chars().filter_map(|c| c.to_digit(10)).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    for (idx_row, row) in input.clone()[1..input.len() - 1].iter().enumerate() {
        for (idx_col, col) in row[1..row.len() - 1].iter().enumerate() {
            let coord = Coord {
                row: idx_row + 1,
                col: idx_col + 1,
            };
            println!("{:?} : {}", coord, coord.am_i_visible(&input));
        }
    }

    Ok(())
}

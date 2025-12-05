use std::io::{self, BufRead};

use crate::utils::read;

pub fn run(input: &str) -> io::Result<(u64, u64)> {
    let mut grid: Vec<Vec<bool>>;

    let input = read(input)?;

    for (column_index, line) in input.lines().enumerate() {
        let characters = line.unwrap();
        for (row_index, spot) in characters.chars().enumerate() {
            /*
            grid[column_index].append(match spot {
                '.' => false,
                '@' => true,
            });
            */
        }
        
    }

    Ok((0, 0))
}
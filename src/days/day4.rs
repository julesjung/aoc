use std::io::{self, BufRead};

use crate::utils::read;

pub fn is_accessible(grid: &[Vec<u8>], row_index: usize, column_index: usize) -> bool {
    let row_length = grid[row_index].len();

    let mut neighbors = 0;

    let next_column_index = column_index + 1;

    if let Some(previous_row_index) = row_index.checked_sub(1) {
        if let Some(previous_column_index) = column_index.checked_sub(1) {
            neighbors += grid[previous_row_index][previous_column_index];
        }
        neighbors += grid[previous_row_index][column_index];
        if next_column_index < row_length {
            neighbors += grid[previous_row_index][next_column_index];
        }
    }

    if let Some(previous_column_index) = column_index.checked_sub(1) {
        neighbors += grid[row_index][previous_column_index];
    }
    if next_column_index < row_length {
        neighbors += grid[row_index][next_column_index];
    }

    let next_row_index = row_index + 1;
    if next_row_index < grid.len() {
        if let Some(previous_column_index) = column_index.checked_sub(1) {
            neighbors += grid[next_row_index][previous_column_index];
        }
        neighbors += grid[next_row_index][column_index];
        if next_column_index < row_length {
            neighbors += grid[next_row_index][next_column_index];
        }
    }

    neighbors < 4
}

pub fn run(input: &str) -> io::Result<(u64, u64)> {
    let input = read(input)?;

    let mut part1 = 0;

    let mut grid: Vec<Vec<u8>> = Vec::new();
    let mut new_grid: Vec<Vec<u8>> = Vec::new();

    for line in input.lines() {
        let line = line
            .unwrap()
            .chars()
            .map(|character| match character {
                '.' => 0,
                '@' => 1,
                _ => panic!(),
            })
            .collect();

        grid.push(line);
    }

    for (row_index, row) in grid.iter().enumerate() {
        let mut new_row = Vec::new();
        for (column_index, column) in row.iter().enumerate() {
            if *column == 1 && is_accessible(&grid, row_index, column_index) {
                new_row.push(0);
                part1 += 1;
            } else {
                new_row.push(*column);
            };
        }
        new_grid.push(new_row);
    }

    let mut part2 = part1;
    grid = new_grid.clone();

    loop {
        let mut has_changed = false;

        let mut new_grid = Vec::new();

        for (row_index, row) in grid.iter().enumerate() {
            let mut new_row = Vec::new();

            for (column_index, column) in row.iter().enumerate() {
                if *column == 1 && is_accessible(&grid, row_index, column_index) {
                    new_row.push(0);
                    part2 += 1;
                    has_changed = true;
                } else {
                    new_row.push(*column);
                };
            }
            new_grid.push(new_row);
        }

        if !has_changed {
            break;
        }

        grid = new_grid.clone();
    }

    Ok((part1, part2))
}

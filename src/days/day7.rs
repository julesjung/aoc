use std::{
    io::{self, BufRead}, vec
};

use crate::utils::read;

fn count_branches(manifold: &[String], row_index: usize, column_index: usize) -> u64 {
    if row_index >= manifold.len() {
        return 1;
    }

    let row = &manifold[row_index];

    if row.chars().nth(column_index) == Some('^') {
        let left_branches = count_branches(manifold, row_index + 1, column_index - 1);
        let right_branches = count_branches(manifold, row_index + 1, column_index + 1);
        return left_branches + right_branches;
    }

    count_branches(manifold, row_index + 2, column_index)
}

pub fn run(input: &str) -> io::Result<(u64, u64)> {
    let input = read(input)?;
    let lines = input.lines();

    let mut manifold: Vec<String> = Vec::new();


    for line in lines {
        manifold.push(line?);
    }

    let mut part1: u64 = 0;

    let first_line = &manifold[0];

    let mut beams: Vec<bool> = vec![false; first_line.len()];

    beams[first_line.find('S').unwrap()] = true;

    for line in manifold.iter() {
        let mut next_beams: Vec<bool> = beams.clone();

        for (index, character) in line.chars().enumerate() {
            if character == '^' && beams[index] {
                part1 += 1;
                next_beams[index - 1] = true;
                next_beams[index] = false;
                next_beams[index + 1] = true;
            }
        }

        beams = next_beams;
    }

    let part2: u64 = count_branches(&manifold, 1, first_line.find('S').unwrap());

    Ok((part1, part2))
}

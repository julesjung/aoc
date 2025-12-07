use std::{
    io::{self, BufRead},
    vec,
};

use crate::utils::read;

pub fn run(input: &str) -> io::Result<(u64, u64)> {
    let input = read(input)?;
    let mut lines = input.lines();

    let mut part1: u64 = 0;
    let mut part2: u64 = 0;

    let first_line = lines.next().unwrap()?;

    let mut beams: Vec<bool> = vec![false; first_line.len()];

    beams[first_line.find('S').unwrap()] = true;

    for line in lines {
        let mut next_beams: Vec<bool> = beams.clone();

        let line = line?;

        for (index, character) in line.chars().enumerate() {
            if character == '^' && beams[index] {
                part1 += 1;
                part2 += 2;
                next_beams[index - 1] = true;
                next_beams[index] = false;
                next_beams[index + 1] = true;
            }
        }

        beams = next_beams;
    }

    Ok((part1, part2))
}

use std::{
    io::{self, BufRead},
    vec,
};

use crate::utils::read;

pub fn run(input: &str) -> io::Result<(u64, u64)> {
    let input = read(input)?;
    let lines = input.lines();

    let mut manifold: Vec<String> = Vec::new();

    for line in lines {
        manifold.push(line?);
    }

    let mut part1: u64 = 0;

    let first_line = &manifold[0];

    let mut beams: Vec<u64> = vec![0; first_line.len()];

    beams[first_line.find('S').unwrap()] = 1;

    for line in manifold.iter() {
        let mut future_beams: Vec<u64> = beams.clone();

        for (index, character) in line.chars().enumerate() {
            if character == '^' && beams[index] != 0 {
                part1 += 1;
                future_beams[index - 1] += beams[index];
                future_beams[index] = 0;
                future_beams[index + 1] += beams[index];
            }
        }

        beams = future_beams;
    }

    let part2 = beams.iter().sum();

    Ok((part1, part2))
}

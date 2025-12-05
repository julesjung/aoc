use std::io::{self, BufRead};

use crate::utils::read;

pub fn run(input: &str) -> io::Result<(u64, u64)> {
    let input = read(input)?;

    let mut part1: u32 = 0;
    let mut part2: u32 = 0;
    let mut rotation: u32 = 50;
    let mut inverse_rotation: u32 = 50;

    for line in input.lines() {
        let instruction = line.unwrap();

        let prefix = instruction.chars().next().unwrap();
        let distance = instruction[1..].parse::<u32>().unwrap();

        match prefix {
            'L' => {
                inverse_rotation += distance;
                part2 += inverse_rotation / 100;
                inverse_rotation %= 100;
                rotation = (100 - inverse_rotation) % 100;
            }
            'R' => {
                rotation += distance;
                part2 += rotation / 100;
                rotation %= 100;
                inverse_rotation = (100 - rotation) % 100;
            }
            _ => unreachable!(),
        };

        if rotation == 0 {
            part1 += 1;
        };
    }

    Ok((part1 as u64, part2 as u64))
}

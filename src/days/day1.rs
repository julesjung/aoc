use std::io::{self, BufRead};

use crate::utils::{Codes, read};

pub fn run(input: &str) -> io::Result<Codes> {
    let input = read(input)?;

    let mut part1: i32 = 0;
    let mut part2: i32 = 0;
    let mut rotation: i32 = 50;
    let mut inverse_rotation: i32 = 50;

    for line in input.lines() {
        let instruction = line.unwrap();

        let prefix = instruction.chars().next().unwrap();
        let distance = instruction[1..].parse::<i32>().unwrap();

        let new_rotation = match prefix {
            'L' => {
                part2 += (inverse_rotation + distance) / 100;
                rotation - distance
            }
            'R' => {
                part2 += (rotation + distance) / 100;
                rotation + distance
            }
            _ => unreachable!(),
        };

        rotation = new_rotation.rem_euclid(100);
        inverse_rotation = (-new_rotation).rem_euclid(100);

        if rotation == 0 {
            part1 += 1;
        };
    }

    Ok(Codes::new(Some(part1), Some(part2)))
}

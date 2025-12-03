use std::io::{self, BufRead};

use crate::utils::{Answers, read};

pub fn run(input: &str) -> io::Result<Answers> {
    let input = read(input)?;

    let mut part1: u32 = 0;
    let mut part2: u32 = 0;

    for line in input.lines() {
        let line = line.unwrap();
        let mut digits = line.chars();
        let mut first_number = '0';
        let mut last_number = '0';
        for digit in digits {
            if first_number < last_number {
                first_number = last_number;
                last_number = digit;
            } else if last_number < digit {
                last_number = digit;
            }

        }
        dbg!(first_number, last_number);
    }

    return Ok((None, None))
}
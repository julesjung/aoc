use std::io::{self, BufRead};

use crate::utils::{Answers, read};

#[inline]
fn joltage(rating: &str) -> u32 {
    let mut first_digit = '0';
    let mut last_digit = '0';
    for digit in rating.chars() {
        if first_digit < last_digit {
            first_digit = last_digit;
            last_digit = digit;
        } else if last_digit < digit {
            last_digit = digit;
        }
    }

    let first_digit = first_digit.to_digit(10).unwrap();
    let last_digit = last_digit.to_digit(10).unwrap();

    first_digit * 10 + last_digit
}

pub fn run(input: &str) -> io::Result<Answers> {
    let input = read(input)?;

    let mut part1: u32 = 0;
    let mut part2: u32 = 0;

    for line in input.lines() {
        let rating = line.unwrap();
        part1 += joltage(rating.as_str());
    }

    Ok((Some(part1 as u64), None))
}

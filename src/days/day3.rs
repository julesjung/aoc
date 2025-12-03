use std::io::{self, BufRead};

use crate::utils::{Answers, read};

#[inline]
fn part1_joltage(rating: &str) -> u32 {
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

#[inline]
fn part2_joltage(rating: &str) -> u64 {
    let mut digits: [char; 12] = ['0'; 12];
    for (index, digit) in rating.chars().enumerate() {
        let mut current_index = (12 + index).saturating_sub(rating.len());
        while current_index < 12 {
            if digits[current_index] < digit {
                digits[current_index] = digit;
                for item in digits.iter_mut().skip(current_index + 1) {
                    *item = '0';
                }
                break;
            }
            current_index += 1;
        }
    }

    let joltage: String = digits.iter().collect();

    joltage.parse().unwrap()
}

pub fn run(input: &str) -> io::Result<Answers> {
    let input = read(input)?;

    let mut part1: u32 = 0;
    let mut part2: u64 = 0;

    for line in input.lines() {
        let rating = line.unwrap();
        part1 += part1_joltage(rating.as_str());
        part2 += part2_joltage(rating.as_str());
    }

    Ok((Some(part1 as u64), Some(part2)))
}

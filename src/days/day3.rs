use std::io::{self, BufRead};

use crate::utils::read;

fn find_greatest_combination(rating: &str, digits: &mut [char]) {
    for (index, digit) in rating.chars().enumerate() {
        let mut current_index = (digits.len() + index).saturating_sub(rating.len());
        while current_index < digits.len() {
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
}

#[inline]
fn joltage(rating: &str) -> (u64, u64) {
    let mut part1_digits = ['0'; 2];
    let mut part2_digits = ['0'; 12];

    find_greatest_combination(rating, &mut part1_digits);
    find_greatest_combination(rating, &mut part2_digits);

    let part1_joltage: String = part1_digits.iter().collect();
    let part2_joltage: String = part2_digits.iter().collect();

    (
        part1_joltage.parse().unwrap(),
        part2_joltage.parse().unwrap(),
    )
}

pub fn run(input: &str) -> io::Result<(u64, u64)> {
    let input = read(input)?;

    let mut part1: u64 = 0;
    let mut part2: u64 = 0;

    for line in input.lines() {
        let rating = line.unwrap();
        let joltages = joltage(rating.as_str());
        part1 += joltages.0;
        part2 += joltages.1;
    }

    Ok((part1, part2))
}

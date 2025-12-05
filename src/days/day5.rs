use std::{io::{self, BufRead}, ops::RangeInclusive};

use crate::utils::read;

pub fn run(input: &str) -> io::Result<(u64, u64)> {
    let input = read(input)?;

    let mut part1 = 0;
    let mut part2 = 0;

    let mut lines = input.lines();

    let mut valid_ranges: Vec<RangeInclusive<u64>> = Vec::new();

    for line in lines.by_ref() {
        let line = line?;
        if line.is_empty() {
            break;
        }
        let mut range = line.split("-");
        let mut range_start: u64 = range.next().unwrap().parse().unwrap();
        let mut range_end: u64 = range.next().unwrap().parse().unwrap();

        valid_ranges.retain(|valid_range| {
            if &range_start <= valid_range.end() && valid_range.start() <= &range_end {
                range_start = range_start.min(*valid_range.start());
                range_end = range_end.max(*valid_range.end());
                return false;
            }
            return true;
        });

        valid_ranges.push(range_start..=range_end);
    }

    for line in lines {
        let ingredient_id = line.unwrap().parse().unwrap();
        
        for range in &valid_ranges {
            if range.contains(&ingredient_id) {
                part1 += 1;
                break;
            };
        }
    }

    for range in valid_ranges {
        part2 += range.count() as u64;
    }

    Ok((part1, part2))
}
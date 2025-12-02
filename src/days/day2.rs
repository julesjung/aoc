use std::{fs::read_to_string, io};

use crate::utils::Answers;

fn part1_invalid(id: &str) -> bool {
    let (first_half, second_half) = id.split_at(id.len() / 2);

    first_half == second_half
}

fn part2_invalid(id: &str) -> bool {
    for chunk_size in 1..=id.len() / 2 {
        let mut invalid = true;
        let characters = id.chars().collect::<Vec<char>>();
        let mut chunks = characters.chunks(chunk_size);
        let first_chunk = chunks.next().unwrap();
        for chunk in chunks {
            if chunk != first_chunk {
                invalid = false;
                break;
            }
        }
        if invalid {
            return true;
        }
    }

    false
}

pub fn run(input: &str) -> io::Result<Answers> {
    let input = read_to_string(input)?;
    let mut part1: u64 = 0;
    let mut part2: u64 = 0;

    for range in input.split(",") {
        let mut range = range.split("-");
        let first_id: u64 = range.next().unwrap().parse().unwrap();
        let last_id: u64 = range.next().unwrap().parse().unwrap();

        for id in first_id..=last_id {
            if part1_invalid(id.to_string().as_str()) {
                part1 += id;
                part2 += id;
            } else if part2_invalid(id.to_string().as_str()) {
                part2 += id;
            }
        }
    }

    Ok((Some(part1), Some(part2)))
}

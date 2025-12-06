use std::io::{self, BufRead};

use crate::utils::read;

#[derive(Debug)]
struct MathProblem {
    sum: u128,
    product: u128,
}

impl Default for MathProblem {
    fn default() -> Self {
        Self { sum: 0, product: 1 }
    }
}

impl MathProblem {
    fn new(initial_value: u128) -> Self {
        Self {
            sum: initial_value,
            product: initial_value,
        }
    }

    fn add(&mut self, value: u128) {
        self.sum += value;
        self.product *= value;
    }
}

pub fn run(input: &str) -> io::Result<(u64, u64)> {
    let input = read(input)?;
    let mut lines = input.lines();

    let mut part1: u128 = 0;
    let mut part2: u128 = 0;

    let mut problems: Vec<MathProblem> = Vec::new();
    let mut columns: Vec<String> = Vec::new();

    let first_line = lines.next().unwrap()?;

    for character in first_line.chars() {
        columns.push(character.to_string());
    }

    for line in first_line.split_whitespace() {
        problems.push(MathProblem::new(line.parse().unwrap()));
    }

    for line in lines {
        let line = line?;

        for (index, character) in line.chars().enumerate() {
            columns[index].push(character);
        }

        for (problem_index, value) in line.split_whitespace().enumerate() {
            if value == "+" {
                part1 += problems[problem_index].sum;
            } else if value == "*" {
                part1 += problems[problem_index].product;
            } else {
                problems[problem_index].add(value.parse().unwrap());
            }
        }
    }

    let mut problem = MathProblem::default();

    columns.reverse();

    for column in columns {
        let mut value = String::new();
        for character in column.chars() {
            if character == '+' {
                problem.add(value.parse().unwrap());
                part2 += problem.sum;
            } else if character == '*' {
                problem.add(value.parse().unwrap());
                part2 += problem.product
            } else if character.is_ascii_digit() {
                value.push(character);
            }
        }
        if value.is_empty() {
            problem = MathProblem::default();
        } else {
            problem.add(value.parse().unwrap());
        }
    }

    Ok((part1 as u64, part2 as u64))
}

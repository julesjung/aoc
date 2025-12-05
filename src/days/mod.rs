pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;

use std::io;

pub fn run(day: u8, input: &str) -> io::Result<(u64, u64)> {
    match day {
        1 => day1::run(input),
        2 => day2::run(input),
        3 => day3::run(input),
        4 => day4::run(input),
        5 => day5::run(input),
        _ => unimplemented!(),
    }
}

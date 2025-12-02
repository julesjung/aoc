pub mod day1;
pub mod day2;

use crate::utils::Answers;
use std::io;

pub fn run(day: u8, input: &str) -> io::Result<Answers> {
    match day {
        1 => day1::run(input),
        2 => day2::run(input),
        _ => unimplemented!(),
    }
}

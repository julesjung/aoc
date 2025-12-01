use std::io;

use crate::utils::Codes;

pub mod day1;

pub fn run(day: u8, input: &str) -> io::Result<Codes> {
    match day {
        1 => day1::run(input),
        _ => unimplemented!(),
    }
}

use std::{fs::File, io};

pub fn read(input: &str) -> io::Result<io::BufReader<File>> {
    let input = File::open(input)?;
    let reader = io::BufReader::new(input);
    Ok(reader)
}

pub struct Codes {
    pub part1: Option<i32>,
    pub part2: Option<i32>,
}

impl Codes {
    pub fn new(part1: Option<i32>, part2: Option<i32>) -> Self {
        Self { part1, part2 }
    }
}

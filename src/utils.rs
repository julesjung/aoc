use std::{fs::File, io};

pub fn read(input: &str) -> io::Result<io::BufReader<File>> {
    let input = File::open(input)?;
    let reader = io::BufReader::new(input);
    Ok(reader)
}

pub type Answers = (Option<u64>, Option<u64>);

mod days;
mod utils;

use std::{io, time::Instant};

use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    day: u8,
    input: String,
}

fn main() -> io::Result<()> {
    let cli = Cli::parse();

    let start = Instant::now();

    let (part1, part2) = days::run(cli.day, &cli.input)?;

    let elapsed = start.elapsed();

    if let Some(part1) = part1 {
        println!("Part 1: {}", part1);
    }

    if let Some(part2) = part2 {
        println!("Part 2: {}", part2);
    }

    println!("Total time: {:?}", elapsed);

    Ok(())
}

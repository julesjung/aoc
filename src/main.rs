mod days;
mod utils;

use std::{io, time::Instant};

use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(short, long)]
    day: u8,
    #[arg(short, long)]
    input: String,
}

fn main() -> io::Result<()> {
    let cli = Cli::parse();

    let start = Instant::now();

    let parts = days::run(cli.day, &cli.input)?;

    let elapsed = start.elapsed();

    if let Some(part1) = parts.part1 {
        println!("Part 1: {}", part1);
    }

    if let Some(part2) = parts.part2 {
        println!("Part 2: {}", part2);
    }

    println!("Total time: {:?}", elapsed);

    Ok(())
}

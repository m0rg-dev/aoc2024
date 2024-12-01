use std::path::PathBuf;

use clap::Parser;
use common::DaySolver;
use day01::day_01;

mod common;
mod day01;

#[derive(Parser)]
struct Args {
    day: usize,
    part: usize,
    source: PathBuf,
}

fn main() {
    let days: Vec<Box<DaySolver>> = vec![Box::new(day_01)];

    let args = Args::parse();
    let input = std::fs::read_to_string(args.source).unwrap();
    println!("{}", days[args.day - 1](&input, args.part == 1).unwrap());
}

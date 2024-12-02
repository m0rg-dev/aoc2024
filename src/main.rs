use std::path::PathBuf;

use clap::Parser;
use common::{DaySolver, Part};
use day01::day_01;
use day02::day_02;

mod common;
mod day01;
mod day02;

#[derive(Parser)]
struct Args {
    day: usize,
    part: Part,
    source: PathBuf,
}

fn main() {
    let days: Vec<Box<DaySolver>> = vec![Box::new(day_01), Box::new(day_02)];

    let args = Args::parse();
    let input = std::fs::read_to_string(args.source).unwrap();
    println!("{}", days[args.day - 1](&input, args.part).unwrap());
}

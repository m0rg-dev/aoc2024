use std::path::PathBuf;

use clap::Parser;
use common::{DaySolver, Part};
use day00::day_00;
use day01::day_01;
use day02::day_02;
use day03::day_03;
use day04::day_04;
use day05::day_05;
use day06::day_06;

mod common;
mod day00;
mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;

#[derive(Parser)]
struct Args {
    day: usize,
    part: Part,
    source: PathBuf,
}

fn main() {
    let days: Vec<Box<DaySolver>> = vec![
        Box::new(day_00),
        Box::new(day_01),
        Box::new(day_02),
        Box::new(day_03),
        Box::new(day_04),
        Box::new(day_05),
        Box::new(day_06),
    ];

    let args = Args::parse();
    let input = std::fs::read_to_string(args.source).unwrap();
    println!("{}", days[args.day](&input, args.part).unwrap());
}

use std::{error::Error, str::FromStr};

use thiserror::Error;

pub(crate) type DaySolver = dyn Fn(&str, Part) -> Result<String, Box<dyn Error>>;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Part {
    PartOne,
    PartTwo,
}

#[derive(Error, Debug)]
pub(crate) enum PartParseError {
    #[error("Unknown part {0}")]
    Unknown(String),
}

impl FromStr for Part {
    type Err = PartParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "a" | "one" | "1" => Ok(Self::PartOne),
            "b" | "two" | "2" => Ok(Self::PartTwo),
            _ => Err(PartParseError::Unknown(s.to_string())),
        }
    }
}

impl Part {
    pub fn choose<F, G, A, R>(&self, part_one: F, part_two: G, args: A) -> R
    where
        F: FnOnce(A) -> R,
        G: FnOnce(A) -> R,
    {
        match self {
            Part::PartOne => part_one(args),
            Part::PartTwo => part_two(args),
        }
    }
}

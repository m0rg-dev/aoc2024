use std::{error::Error, str::FromStr};

use thiserror::Error;

pub(crate) type DaySolver = dyn Fn(&str, Part) -> Result<String, Box<dyn Error>>;

#[derive(Clone, Copy, Debug)]
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
    pub fn choose<F, G>(&self, part_one: F, part_two: G) -> Box<DaySolver>
    where
        F: Fn(&str) -> Result<String, Box<dyn Error>> + 'static,
        G: Fn(&str) -> Result<String, Box<dyn Error>> + 'static,
    {
        Box::new(move |input: &str, part: Part| match part {
            Part::PartOne => part_one(input),
            Part::PartTwo => part_two(input),
        })
    }
}

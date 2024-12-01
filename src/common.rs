use std::error::Error;

pub(crate) type DaySolver = dyn Fn(&str, bool) -> Result<String, Box<dyn Error>>;

use std::io::{Error, ErrorKind};

const SAMPLE_INPUT: &str = include_str!("../input/sample.txt");
const TEST_CASE: &str = include_str!("../input/input.txt");

pub enum InputType {
    Sample,
    Test,
}

pub fn get_battery_banks(input_type: InputType) -> Result<Vec<Vec<u8>>, Error> {
    match input_type {
        InputType::Sample => SAMPLE_INPUT,
        InputType::Test => TEST_CASE,
    }.lines()
        .map(|line| {
            line.chars()
                .map(|c| {
                    c.to_digit(10)
                        .map(|d| d as u8)
                        .ok_or_else(|| {
                            Error::new(ErrorKind::InvalidData,
                                format!("Invalid digit '{}'", c))
                        })
                })
                .collect()
        })
        .collect()
}


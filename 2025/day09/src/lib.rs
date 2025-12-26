pub enum InputType {
    Sample,
    Test,
}

const SAMPLE_INPUT: &str = include_str!("../inputs/sample.txt");
const TEST_INPUT: &str = include_str!("../inputs/test.txt");

#[derive(Clone, Copy, Eq, PartialEq)]
pub struct Tile{
    pub x: i64,
    pub y: i64
}


pub fn get_tiles(input_type: InputType) -> Result<Vec<Tile>, String> {
    match input_type {
        InputType::Sample => SAMPLE_INPUT,
        InputType::Test => TEST_INPUT
    }.lines().map(|line| {
        let nums = line.split(',')
                .map(|num_str| num_str.parse::<i64>().map_err(|e| format!("failed to parse number: {}", e)))
                .collect::<Result<Vec<i64>, String>>()?;
        if nums.len() != 2 {
            Err(format!("invalid count of numbers in a line: {}", nums.len()))
        } else {
            Ok(Tile { x: nums[0], y: nums[1] })
        }
    }).collect::<Result<Vec<Tile>, String>>()
}

use std::error::Error;

pub enum InputType {
    Sample,
    Test,
}

const SAMPLE_INPUT: &str = include_str!("../inputs/sample.txt");
const TEST_INPUT: &str = include_str!("../inputs/test.txt");

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum GridCell {
    Empty,
    Paper,
    Removed
}

pub fn get_paper_grid(input_type: InputType) -> Result<Vec<Vec<GridCell>>, String> {
    match input_type {
        InputType::Sample => SAMPLE_INPUT,
        InputType::Test => TEST_INPUT,
    }.lines().map(|line| {
        line.chars().map(|ch| {
            match ch {
                '.' => Ok(GridCell::Empty),
                '@' => Ok(GridCell::Paper),
                _ =>  Err(format!("Invalid character in input: {}", ch)),
            }
        }).collect::<Result<Vec<GridCell>, String>>()
    }).collect::<Result<Vec<Vec<GridCell>>, String>>()
}


pub enum InputType {
    Sample,
    Test,
}

const SAMPLE_INPUT: &str = include_str!("../inputs/sample.txt");
const TEST_INPUT: &str = include_str!("../inputs/test.txt");

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GridCell {
    Empty,
    Beam,
    Start,
    Splitter
}

pub fn get_grid(input_type: InputType) -> Result<Vec<Vec<GridCell>>, String> {
    let input_lines = match input_type {
        InputType::Sample => SAMPLE_INPUT,
        InputType::Test => TEST_INPUT,
    }.lines();

    Ok(input_lines.map(|line| {
        line.chars().map(|ch| {
            match ch {
                '.' => Ok(GridCell::Empty),
                'S' => Ok(GridCell::Start),
                '^' => Ok(GridCell::Splitter),
                _ => Err(format!("Unknown grid cell character: {}", ch)),
            }
        }).collect::<Result<Vec<GridCell>, String>>()
    }).collect::<Result<Vec<Vec<GridCell>>, String>>()?)

}

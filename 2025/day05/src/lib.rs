pub enum InputType {
    Sample,
    Test,
}

const SAMPLE_INPUT: &str = include_str!("../inputs/sample.txt");
const TEST_INPUT: &str = include_str!("../inputs/test.txt");

pub struct KitchenDatabase {
    pub fresh_ranges: Vec<(i64, i64)>,
    pub available_ingredients: Vec<i64>,
}

pub fn get_database(input_type: InputType) -> Result<KitchenDatabase, String> {
    let mut input_lines = match input_type {
        InputType::Sample => SAMPLE_INPUT,
        InputType::Test => TEST_INPUT,
    }.lines();

    let ranges: Vec<(i64, i64)> = input_lines.by_ref().map_while(|line| {
        if line.trim().is_empty() {
            return None;
        }

        let (start_str, end_str) = line.split_once("-")?;

        Some((start_str.trim().parse::<i64>().ok()?, end_str.trim().parse::<i64>().ok()?))
    }).collect();

    let available_ingredients: Vec<i64>= input_lines.skip_while(|line| line.trim().is_empty()).map(|line| {
        line.parse::<i64>().map_err(|e| format!("Failed to parse available ingredient: {}", e))
    }).collect::<Result<Vec<i64>, String>>()?;

    return Ok(KitchenDatabase {
        fresh_ranges: ranges,
        available_ingredients: available_ingredients,
    });
}

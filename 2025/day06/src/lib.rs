use std::fmt;

pub enum InputType {
    Sample,
    Test,
}

const SAMPLE_INPUT: &str = include_str!("../inputs/sample.txt");
const TEST_INPUT: &str = include_str!("../inputs/test.txt");

pub enum Operation {
    Sum,
    Product,
}

pub struct Problem {
    pub nums: Vec<u64>,
    pub operation: Operation,
}

pub struct Worksheet {
    pub problems: Vec<Problem>,
}

impl fmt::Display for Worksheet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (idx, problem) in self.problems.iter().enumerate() {
            let op_str = match problem.operation {
                Operation::Sum => "Sum",
                Operation::Product => "Product",
            };
            writeln!(f, "Problem {}: Operation: {}, Numbers: {:?}", idx + 1, op_str, &problem.nums[0..])?;
        }
        Ok(())
    }
}


pub fn get_worksheet(input_type: InputType) -> Result<Worksheet, String> {
    let mut input_lines = match input_type {
        InputType::Sample => SAMPLE_INPUT,
        InputType::Test => TEST_INPUT,
    }.lines();

    let mut problems = match input_lines.next_back() {
        Some(line) => Ok(line),
        None =>  Err("Input is empty".to_string()),
    }?.split_whitespace().map(|op| match op {
            "*" => Ok(Operation::Product),
            "+" => Ok(Operation::Sum),
            _ => Err(format!("Unknown operation: {}", op)),
        }).map(|op| 
                match op {
                    Ok(operation) => {
                        Ok(Problem { nums: vec![], operation })
                    },
                    Err(e) => Err(e),
                }).collect::<Result<Vec<Problem>, String>>()?;

    input_lines
    .flat_map(|line| {
        line.split_whitespace()
            .map(|s| s.parse::<u64>().map_err(|e| format!("parse err: {}", e)))
            .enumerate()
    })
    .try_for_each(|(idx, num_res)| -> Result<(), String> {
        problems[idx].nums.push(num_res?);
        Ok(())
    })?;


    Ok(Worksheet { problems: problems })
}

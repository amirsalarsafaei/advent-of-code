use std::fmt;

pub enum InputType {
    Sample,
    Test,
}

pub enum InputMode {
    Part1,
    Part2,
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


pub fn get_worksheet(input_type: InputType, input_mode: InputMode) -> Result<Worksheet, String> {
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
    match input_mode {
        InputMode::Part1 => {
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
        },
        InputMode::Part2 => {
            let numbers_input = input_lines.map(|line|line.chars().collect::<Vec<char>>())
                .collect::<Vec<Vec<char>>>();
            let col_size = numbers_input[0].len();
            let mut problem_idx = 0;
            for col in 0..col_size {
                let mut num : Option<u64> = None;
                for (idx, row) in numbers_input.iter().enumerate() {
                    let ch = row[col];
                    if ch.is_whitespace() {
                        continue;
                    }
                    let dig = ch.to_digit(10)
                        .ok_or_else(|| format!("Invalid character '{}' at row {}, col {}", ch, idx, col))? as u64;
                    match num {
                        Some(n) => {
                            num = Some(n * 10 + dig);
                        },
                        None => {
                            num = Some(dig);
                        },
                    }
                }
                if let Some(n) = num {
                    problems[problem_idx].nums.push(n);
                } else {
                    problem_idx += 1;
                }
            }
        },
    }



    Ok(Worksheet { problems: problems })
}

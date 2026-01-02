pub enum InputType {
    Sample,
    Test,
}

const SAMPLE_INPUT: &str = include_str!("../inputs/sample.txt");
const TEST_INPUT: &str = include_str!("../inputs/test.txt");

pub struct Machine {
    pub lights: Vec<bool>,
    pub buttons: Vec<Vec<usize>>,
    pub joltage_requirements: Vec<u32>,
}

pub struct Manual {
    pub machines: Vec<Machine>,
}

pub fn get_manual(input_type: InputType) -> Result<Manual, String> {
    match input_type {
        InputType::Sample => SAMPLE_INPUT,
        InputType::Test => TEST_INPUT,
    }
    .lines()
    .map(|line| {
        line.split_whitespace().try_fold(
            Machine {
                lights: vec![],
                buttons: vec![],
                joltage_requirements: vec![],
            },
            |mut machine, part| -> Result<Machine, String> {
                match part.chars().next() {
                    Some('[') => {
                        machine.lights = part
                            .trim_matches(&['[', ']'][..])
                            .chars()
                            .map(|ch| ch == '#')
                            .collect();
                    }
                    Some('(') => {
                        let button_states = part
                            .trim_matches(&['(', ')'][..])
                            .split(',')
                            .map(|s| s.parse::<usize>().map_err(|e| e.to_string()))
                            .collect::<Result<Vec<usize>, String>>()?;
                        machine.buttons.push(button_states);
                    }
                    Some('{') => {
                        let joltage_values = part
                            .trim_matches(&['{', '}'][..])
                            .split(',')
                            .map(|s| s.parse::<u32>().map_err(|e| e.to_string()))
                            .collect::<Result<Vec<u32>, String>>()?;
                        machine.joltage_requirements = joltage_values;
                    }
                    _ => {
                        return Err(format!("Unexpected part format: {}", part))
                    }
                }
                Ok(machine)
            },
        )
    })
    .collect::<Result<Vec<Machine>, String>>()
    .map(|machines| Manual { machines })
}



#[derive(Debug)]
pub enum TurnDirection {
    Right,
    Left,
}

pub struct Instruction {
    pub direction: TurnDirection,
    pub distance: u32,
}
impl Instruction {
    pub fn new(direction: TurnDirection, distance: u32) -> Self {
        Instruction { direction, distance }
    }
    pub fn relative_distance(&self) -> i32 {
        match self.direction {
            TurnDirection::Right => {
                (self.distance % 100) as i32
            },
            TurnDirection::Left => {
                -((self.distance % 100) as i32) 
            },
        }
    }
}


pub fn get_instructions_from_file(file_path: &str) -> Result<Vec<Instruction>, std::io::Error> {
    let content = std::fs::read_to_string(file_path)?;
    let instructions = content.lines().map(|line| {
        let direction = match line.chars().next() {
            Some('R') => TurnDirection::Right,
            Some('L') => TurnDirection::Left,
            _ => panic!("Invalid direction"),
        };
        let distance = line[1..].parse::<u32>().expect("Invalid distance");
        Instruction { direction, distance }
    }).collect();
    Ok(instructions)
}

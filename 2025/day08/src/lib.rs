use std::fmt;

pub enum InputType {
    Sample,
    Test,
}

const SAMPLE_INPUT: &str = include_str!("../inputs/sample.txt");
const TEST_INPUT: &str = include_str!("../inputs/test.txt");

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub struct JunctionBox {
    pub x: i64,
    pub y: i64,
    pub z: i64,
    pub circuit_id: usize,
}

impl fmt::Display for JunctionBox {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "JunctionBox x: {}, y: {}, z: {}, circuit_id: {}", 
            self.x, self.y, self.z, 
            self.circuit_id)
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Circuit {
    pub junction_boxes: Vec<usize>
}

pub struct Problem {
    pub junction_boxes: Vec<JunctionBox>,
    pub connections: usize,
}


pub fn get_problem(input_type: InputType) -> Result<Problem, String> {

    let junctino_boxes = match input_type {
        InputType::Sample => SAMPLE_INPUT,
        InputType::Test => TEST_INPUT,
    }.lines().enumerate().map(|(i, line)| {
        let coords: Vec<i64> = line.split(',')
            .map(|num_str| num_str.parse::<i64>())
            .collect::<Result<Vec<i64>, _>>()
            .map_err(|e| format!("Failed to parse coordinate: {}", e))?;
        if coords.len() != 3 {
            return Err(format!("Expected 3 coordinates, got {}, line: {}", coords.len(), line));
        }
        Ok(JunctionBox {
            x: coords[0],
            y: coords[1],
            z: coords[2],
            circuit_id: i, 
        })
    }).collect::<Result<Vec<JunctionBox>, String>>()?;

    Ok(Problem {
        junction_boxes: junctino_boxes,
        connections: match input_type {
            InputType::Sample => 10,
            InputType::Test => 1000,
        },
    })
}

pub fn junction_box_distance(jb1: &JunctionBox, jb2: &JunctionBox) -> u64 {
    let dx = (jb1.x - jb2.x) as i64;
    let dy = (jb1.y - jb2.y) as i64;
    let dz = (jb1.z - jb2.z) as i64;
    (dx * dx + dy * dy + dz * dz) as u64
}

pub fn get_connection_pairs(problem: &Problem) -> Vec<(u64, usize, usize)> {
    let mut connection_pairs: Vec<(u64, usize, usize)> = (0..problem.junction_boxes.len()-1).
        flat_map(|i| (i+1..problem.junction_boxes.len()).
            map(move |j| (i, j)).collect::<Vec<(usize, usize)>>()).map(|(i, j)|
            (junction_box_distance(&problem.junction_boxes[i], &problem.junction_boxes[j]), i, j)).collect();

    connection_pairs.sort();
    connection_pairs
}

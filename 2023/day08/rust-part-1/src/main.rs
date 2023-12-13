use std::collections::HashMap;
use std::ops::ControlFlow;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
struct NetworkNode<'a> {
    left: &'a str,
    right: &'a str,
}


#[derive(Debug, PartialEq, Eq)]
struct ParseInstructionError;


impl NetworkNode<'_>{
    fn new(s: &str) -> Result<NetworkNode, ParseInstructionError> {
        let (left, right) = s.strip_prefix("(")
            .and_then(|s| s.strip_suffix(")"))
            .and_then(|s| s.split_once(", "))
            .ok_or(ParseInstructionError)?;
        return Ok(NetworkNode {  left, right })
    }
}


fn main() {
    let input = include_str!("../input.txt");

    let (rl_instructs, network_str) = input.split_once("\n\n").unwrap();

    let network: HashMap<&str, NetworkNode> = network_str
        .split('\n')
        .map(|line| {
            let (node, instruction) = line
                .split_once(" = ")
                .unwrap();

            return (node, NetworkNode::new(instruction).unwrap());
        }).collect();

    println!("{:?}", rl_instructs
        .chars()
        .cycle()
        .enumerate()
        .try_fold("AAA", |position, (step, dir)| {
            if position == "ZZZ" {
                return ControlFlow::Break(step)
            }
            return if dir == 'L' {
                ControlFlow::Continue(network.get(position).unwrap().left)
            } else {
                ControlFlow::Continue(network.get(position).unwrap().right)
            };
        })
    )
}

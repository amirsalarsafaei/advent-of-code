use std::collections::{HashMap, HashSet};
use std::ops::ControlFlow;


fn main() {
    let input = include_str!("../input.txt");

    let (rl_instructs, network_str) = input.split_once("\n\n").unwrap();

    let network: HashMap<&str, (&str, &str)> = network_str
        .split('\n')
        .map(|line| {
            let (node, instruction) = line
                .split_once(" = ")
                .unwrap();

            return (node,
                    instruction.strip_prefix("(")
                        .and_then(|s| s.strip_suffix(")"))
                        .and_then(|s| s.split_once(", ")).unwrap());
        }).collect();

    println!("{:?}", network.keys()
        .filter(|a| a.ends_with('A'))
        .map(
            |node| {
                let tmp = rl_instructs
                    .chars()
                    .enumerate()
                    .cycle()
                    .enumerate()
                    .try_fold(*node, |position, (step, (rl_idx, dir))| {
                        if position.ends_with('Z') {
                            return ControlFlow::Break(step);
                        }
                        return if dir == 'L' {
                            ControlFlow::Continue(network.get(position).unwrap().0)
                        } else {
                            ControlFlow::Continue(network.get(position).unwrap().1)
                        };
                    });
                return if let ControlFlow::Break(c) = tmp {
                    c
                } else {
                    1
                };
            }
        )
        .fold(1, num_integer::lcm)
    )
}

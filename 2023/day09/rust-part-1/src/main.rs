use atoi::atoi;
use itertools::Itertools;

fn main() {
    let input = include_str!("../input.txt");

    println!("{:?}",
             input.lines()
                 .map(
                     |line| {
                         let mut numbers = line.split_whitespace()
                             .map(|a| atoi(a.as_bytes()).unwrap()).collect::<Vec<i64>>();

                         let mut last_elements = Vec::new();
                         last_elements.push(*numbers.last().unwrap());

                         while !numbers.iter().all(|a| *a == 0i64) {
                             numbers = numbers.iter()
                                 .tuple_windows()
                                 .map(|(a, b)| b - a)
                                 .collect();
                             last_elements.push(*numbers.last().unwrap())
                         }

                         return last_elements.iter()
                             .rev()
                             .fold(0i64,
                                   |a, last_element| {
                                       a + *last_element
                                   },
                             );
                     }
                 ).fold(0, |a, b| a + b)
    )
}

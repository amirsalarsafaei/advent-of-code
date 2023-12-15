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

                         let mut first_elements = Vec::new();
                         first_elements.push(*numbers.first().unwrap());

                         while !numbers.iter().all(|a| *a == 0i64) {
                             numbers = numbers.iter()
                                 .tuple_windows()
                                 .map(|(a, b)| b - a)
                                 .collect();
                             first_elements.push(*numbers.first().unwrap())
                         }

                         return first_elements.iter()
                             .rev()
                             .fold(0i64,
                                   |a, first_element| {
                                       first_element - a
                                   },
                             );
                     }
                 ).fold(0, |a, b| a + b)
    )
}

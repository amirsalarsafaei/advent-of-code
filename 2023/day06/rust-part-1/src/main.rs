use std::cmp::min;
use std::cmp::Ordering::*;
use atoi::atoi;

fn main() {
    let input = include_str!("../input.txt");

    let (time, distance) = input.split_once("\n").unwrap();

    println!("{:?}", time.split_once(":").unwrap().1.trim_start()
        .split_whitespace().map(|a| atoi(a.as_bytes()).unwrap())
        .zip(
            distance.split_once(":").unwrap().1.trim_start().split_whitespace().
                map(|a| atoi(a.as_bytes()).unwrap())
        ).map(|(time, distance) : (i64, i64) | {
            (0..=time).map(|t| min((time - t) * t, distance)).
                zip((0..=time).map(|_a| 1i64)).
                reduce(|a, b| {
                    return match a.0.cmp(&b.0) {
                        Equal => (a.0, a.1 + b.1),
                        Less => b,
                        Greater => a
                    }
                }).unwrap().1
        }).reduce(|x, y| x * y).unwrap()
    )
}

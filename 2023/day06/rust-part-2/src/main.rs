use std::cmp::{max, min};
use atoi::atoi;

fn main() {
    let input = include_str!("../input.txt");

    let (time_str, distance_str) = input.split_once("\n").unwrap();

    let (time, distance) = (line_to_i64(time_str), line_to_i64(distance_str));

    let delta :f64 = (time as f64 * time as f64 - 4f64 * distance as f64).sqrt();

    println!("{:?}, {:?}", time, distance);

    let (max_time, min_time) =
        (min(time, ((time as f64 + delta) / 2f64).floor() as i64) ,
         max(0i64, ((time as f64 - delta) / 2f64).ceil() as i64));

    println!("{:?}, {:?}", min_time, max_time);

    println!("{:?}", max_time - min_time + 1)

}


fn line_to_i64(a: &str) -> i64 {
    let mut res = String::new();
    a.split_once(":").unwrap().1.chars().filter(|&a| a != ' ').
        for_each(|ch| {
            res.push(ch)
        });
    return atoi(res.as_bytes()).unwrap()
}
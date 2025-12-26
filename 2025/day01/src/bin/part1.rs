use day01::{get_instructions_from_file};

fn main() {
    let mut pos: i32 = 50;
    let mut reached_zero_cnt: i32 = 0;

    get_instructions_from_file("input/input.txt").unwrap().iter().
        for_each(|instruction| {
            pos = (pos + instruction.relative_distance() + 100) % 100;
            if pos == 0 {
                reached_zero_cnt += 1;
            }
    });


    println!("Reached zero count: {}", reached_zero_cnt);
}

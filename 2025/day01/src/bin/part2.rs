use day01::{get_instructions_from_file};

fn main() {
    let mut pos: i32 = 50;
    let mut passed_zero: u32 = 0;

    get_instructions_from_file("input/input.txt").unwrap().iter().
        for_each(|instruction| {
            let before_pos = pos;
            pos += instruction.relative_distance();
            if pos >= 100 {
                pos = pos - 100;
                passed_zero += 1;
            } else if pos < 0 {
                pos = pos + 100;
                if before_pos != 0 {
                    passed_zero += 1;
                }
            } else if pos == 0 && instruction.relative_distance() != 0 {
                passed_zero += 1;
            }

            passed_zero += instruction.distance / 100;
    });

    println!("Passed zero count: {}", passed_zero);
}

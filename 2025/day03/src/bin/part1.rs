use day03::{get_battery_banks, InputType};
use std::cmp::max;


fn main()  {
    let battery_banks = get_battery_banks(InputType::Test).expect("Failed to get battery banks");

    let total_output = battery_banks.iter().fold(0, |sum , bank| {
        let mut max_digit: Option<u8> = None;
        let mut max_res: u32 = 0;
        for digit in bank {
            if let Some(current_max) = max_digit {
                max_res = max(max_res, current_max as u32 * 10 + *digit as u32);
            }
            max_digit = Some(max(max_digit.unwrap_or(0), *digit));
        }
        return  sum + max_res;
    });

    println!("Total output: {}", total_output);
}

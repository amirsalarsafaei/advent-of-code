use day03::{get_battery_banks, InputType};
use std::cmp::max;

const BATTERIES_TO_TURN: usize = 12;

fn main()  {
    let battery_banks = get_battery_banks(InputType::Test).expect("Failed to get battery banks");

    let total_output = battery_banks.iter().fold(0, |sum , bank| {
        let mut max_res: u64 = 0;
        let mut dp = vec![vec![0;  BATTERIES_TO_TURN + 1]; bank.len()];
        
        dp[0][1] = bank[0] as u64;

        for (i, digit) in bank.iter().enumerate().skip(1) {
            for j in 1..=BATTERIES_TO_TURN {
                dp[i][j] = max(dp[i-1][j], dp[i-1][j-1] * 10 + *digit as u64);
            }
            max_res = max(max_res, dp[i][BATTERIES_TO_TURN]);
        }

        return sum + max_res;
    });

    println!("Total output: {}", total_output);
}

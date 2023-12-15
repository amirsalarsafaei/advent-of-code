use atoi::atoi;

fn main() {
    let input = include_str!("../input.txt");
    println!("{}", input.lines().map(
            |line| {
                let (conditions, duplicate_str) =
                    line.split_once(' ').unwrap();
                let numbers = duplicate_str.split(',')
                    .map(|a| atoi(a.as_bytes()).unwrap()).collect::<Vec<i64>>();

                let mut dp = (0..conditions.len())
                    .map(
                        |a| {
                            (0..=numbers.len()).map(|a| [0i64, 0i64]).collect::<Vec<_>>()
                        }
                    ).collect::<Vec<_>>();

                let conditions_vec = conditions.chars().collect::<Vec<_>>();

                match conditions_vec[0] {
                    '#' => {
                        if conditions.chars()
                            .skip(1)
                            .take((numbers[0] - 1) as usize)
                            .all(|ch| ch == '#'|| ch == '?') {
                            dp[(numbers[0]-1) as usize][1][1] = 1;
                        }
                    },
                    '.' => {
                        dp[0][0][0] = 1;
                    },
                    '?' => {
                        if conditions.chars()
                            .skip(1)
                            .take((numbers[0] - 1) as usize)
                            .all(|ch| ch == '#'|| ch == '?') {
                            dp[(numbers[0]-1) as usize][1][1] = 1;
                        }
                        dp[0][0][0] = 1;
                    },
                    _ => {}
                }

                for i in 1..conditions.len() {
                    match conditions_vec[i]{
                        '#' => {
                            if conditions_vec[i-1] != '#'{
                                for (j, num) in numbers.iter().enumerate() {
                                    if (((i-1) as i64 + num) < (conditions_vec.len() as i64)) &&
                                        conditions.chars()
                                            .skip(i+1)
                                        .take((num- 1) as usize)
                                        .all(|ch| ch == '#'|| ch == '?') {
                                        dp[((i-1) as i64 + num) as usize][j+1][1] += dp[i-1][j][0];
                                    }
                                }
                            }
                        },
                        '.' => {
                            dp[i][0][0] = dp[i-1][0][0] + dp[i-1][0][1];
                            for (j, _) in numbers.iter().enumerate() {
                                dp[i][j+1][0] = dp[i-1][j+1][0] + dp[i-1][j+1][1];
                            }
                        },
                        '?' => {
                            if conditions_vec[i-1] != '#'{
                                for (j, num) in numbers.iter().enumerate() {
                                    if (((i-1) as i64 + num) < (conditions_vec.len() as i64))
                                        && conditions.chars()
                                        .skip(i+1)
                                        .take((num- 1) as usize)
                                        .all(|ch| ch == '#'|| ch == '?') {
                                        dp[((i-1) as i64 + num) as usize][j+1][1] += dp[i-1][j][0];
                                    }
                                }
                            }
                            dp[i][0][0] = dp[i-1][0][0] + dp[i-1][0][1];
                            for (j, _) in numbers.iter().enumerate() {
                                dp[i][j+1][0] = dp[i-1][j+1][0] + dp[i-1][j+1][1];
                            }
                        },
                        _ => {}
                    }
                }
                let res = dp.last().unwrap().last().unwrap();
                return res[0] + res[1];
            }
        )
        .fold(0i64, |a, b| { a + b })
    )
}

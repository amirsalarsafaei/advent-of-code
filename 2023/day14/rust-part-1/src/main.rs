fn main() {
    let input = include_str!("../../input.txt");
    println!("{}", get_ans(input));
}

fn get_ans(table_str: &str) -> i64 {
    let mut ans = 0i64;
    let (n, m) = (table_str.lines().count(), table_str.lines().next().unwrap().len());

    table_str.lines().rev().enumerate().fold(
        (0..m).map(|_a| 0).collect::<Vec<_>>(),
        |a, b| {
            a.iter().zip(b.1.chars()).map(|(cnt, ch)| {
                return match ch {
                    '#' => {
                        ans += (b.0 as i64 + (b.0 as i64 - *cnt + 1)) * *cnt / 2i64;
                        return 0;
                    }
                    'O' => *cnt + 1,
                    _ => *cnt
                };
            }).collect()
        },
    ).iter().for_each(|cnt| {
        ans += (n as i64 + (n as i64 - *cnt + 1)) * *cnt / 2i64;
    });

    return ans
}

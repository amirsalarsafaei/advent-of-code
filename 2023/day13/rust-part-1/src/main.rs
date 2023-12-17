use std::collections::HashSet;

fn main() {
    let input = include_str!("../input.txt");

    println!("{}", input.split("\n\n").map(
        |table_str| {
            let table = table_str.lines()
                .map(|a| a.chars().collect::<Vec<_>>()).collect::<Vec<_>>();

            let (n, m) = (table.len(), table.first().unwrap().len());

            let row_mirrors = (0..n).map(|row| {
                (1..m).filter(|&i| {
                    let mut is_mirror = true;
                    for j in 0..i {
                        let mirrored_idx = 2 * i - j - 1;
                        if mirrored_idx >= m {
                            continue;
                        }
                        if table[row][j] != table[row][mirrored_idx] {
                            is_mirror = false;
                            break;
                        }
                    }
                    return is_mirror;
                }).collect::<HashSet<_>>()
            }
            ).fold((1..m).collect::<HashSet<_>>(),
                   |a, b| {
                       return a.intersection(&b).map(|a| *a).collect::<HashSet<_>>();
                   },
            );

            let col_mirrors = (0..m).map(|col| {
                (1..n).filter(|&i| {
                    let mut is_mirror = true;
                    for j in 0..i {
                        let mirrored_idx = 2 * i - j - 1;
                        if mirrored_idx >= n {
                            continue;
                        }
                        if table[j][col] != table[mirrored_idx][col] {
                            is_mirror = false;
                            break;
                        }
                    }
                    return is_mirror;
                }).collect::<HashSet<_>>()
            }
            ).fold((1..n).collect::<HashSet<_>>(),
                   |a, b| {
                       a.intersection(&b).map(|i| *i).collect::<HashSet<_>>()
                   },
            );

            return col_mirrors.iter().fold(0i64, |a, b| 100i64 * (a + *b as i64)) +
                row_mirrors.iter().fold(0i64, |a, b| (a + *b as i64));
        }
    ).fold(0i64, |a, b| a + b))
}

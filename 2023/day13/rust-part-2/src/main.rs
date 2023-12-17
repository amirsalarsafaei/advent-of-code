use std::collections::HashSet;

fn main() {
    let input = include_str!("../input.txt");

    println!("{}", input.split("\n\n").map(
        |table_str| {
            let table = table_str.lines()
                .map(|a| a.chars().collect::<Vec<_>>()).collect::<Vec<_>>();
            let (n, m) = (table.len(), table.first().unwrap().len());
            let  table_t = (0..m).map(|col| {
                (0..n).map(|row| table[row][col]).collect::<Vec<_>>()
            }).collect::<Vec<_>>();

            let (row_mirrors, col_mirrors) = (
                get_row_mirrors(table),
                get_row_mirrors(table_t)
            );

            return row_mirrors.iter().fold(0i64, |a, b| (a + *b as i64)) +
             col_mirrors.iter().fold(0i64, |a, b| a + 100i64 * *b as i64);
        }
    ).fold(0i64, |a, b| a + b))
}


fn get_row_mirrors(table: Vec<Vec<char>>) -> HashSet<usize>{
    let (n, m) = (table.len(), table.first().unwrap().len());
    return (0..n).map(|row| {
        return ((1..m).filter(|&i| {
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
        }).collect::<HashSet<_>>(),
         (1..m).filter(|&i| {
             let mut is_mirror = true;
             let mut seen_smudge = false;
             for j in 0..i {
                 let mirrored_idx = 2 * i - j - 1;
                 if mirrored_idx >= m {
                     continue;
                 }
                 if table[row][j] != table[row][mirrored_idx] {
                     if seen_smudge {
                         is_mirror = false;
                         break;
                     }
                     seen_smudge = true;
                 }
             }
             return is_mirror && seen_smudge;
         }).collect::<HashSet<_>>())
    }).fold(((1..m).collect::<HashSet<_>>(), HashSet::new()),
            |a, b| {
                return (a.0.intersection(&b.0).map(|i| *i).collect(),
                    a.1.intersection(&b.0).collect::<HashSet<_>>().union(
                        &a.0.intersection(&b.1).collect::<HashSet<_>>()
                    ).map(|i| **i).collect()
                )
            },
    ).1;
}

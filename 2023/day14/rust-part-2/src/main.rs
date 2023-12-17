use std::collections::HashMap;
use std::ops::Add;


const CYCLES: i32 = 1000 * 1000 * 1000;

fn main() {
    let input = include_str!("../../input.txt");

    let mut table = input.lines()
        .map(|a| a.chars().collect::<Vec<_>>()).collect::<Vec<_>>();

    let mut seen_tables = HashMap::new();
    let mut tables: Vec<String> = Vec::new();
    tables.push(String::from(input));
    seen_tables.insert(String::from(input) , 0);
    let mut i = 0;
    loop {
        i += 1;
        for _i in 0..4 {
            table = rotate_right(&mut table);
        }
        let table_str = get_table_str(&table);
        tables.push(table_str.clone());
        if seen_tables.contains_key(&table_str) {
            let first_seen = seen_tables.get(&table_str).unwrap();
            let loop_len = i - first_seen;
            let _tmp = (CYCLES - first_seen) % loop_len;
            let ans_table = &tables[(first_seen + _tmp) as usize];
            println!("{}", get_ans(ans_table.as_str()));
            break
        } else {
            seen_tables.insert(String::from(table_str), i);
        }
    }
}

fn rotate_right(table: &mut Vec<Vec<char>>) -> Vec<Vec<char>> {
    let (n, m) = (table.len(), table[0].len());
    let tmp = pull_lever(table);
    (0..m).map(
        |col| { (0..n).rev().map(|row| tmp[row][col]).collect::<Vec<_>>()})
        .collect::<Vec<_>>()
}

fn pull_lever(table: &mut Vec<Vec<char>>) -> Vec<Vec<char>> {
    let (n, m) = (table.len(), table[0].len());
    let mut cnt = (0..m).map(|_a| 0).collect::<Vec<_>>();

    for i in 0..m {
        for j in (0..n).rev() {
            match table[j][i] {
                '#' => {
                    for z in j+1..=j+cnt[i] {
                        table[z][i] = 'O';
                    }
                    cnt[i] = 0
                },
                'O' => {
                    cnt[i] += 1;
                    table[j][i] = '.';
                }
                _ => {}
            }
        }
        for z in 0..cnt[i] {
            table[z][i] = 'O';
        }
    }
    return table.clone();
}

fn get_table_str(table: &Vec<Vec<char>>) -> String {
    table.iter().map(|a| a.iter().collect::<String>())
        .reduce(|a, b| {
            a.add("\n").add(b.as_str())
        }).unwrap()
}

fn get_ans(table_str: &str) -> i64 {
    table_str.lines().rev().enumerate().fold(
        0i64, |a, (cnt, line)| {
            a + (line.chars().filter(|&a| a == 'O').count() as i64) * (cnt + 1) as i64
        }
    )
}

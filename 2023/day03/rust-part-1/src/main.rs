use std::cmp::min;

fn is_digit(ch: char) -> bool {
    return ch >= '0' && ch <= '9'
}


fn main() {
    let input = include_str!("../input.txt");
    let lines = input.split('\n');
    let n = lines.clone().collect::<Vec<_>>().len();
    let m = lines.clone().last().expect("cant be empty").len();

    let table:Vec<char> = lines.map(|s|{ s.chars().collect::<Vec<char>>()}).
        reduce(|x, y| {  x.into_iter().chain(y.into_iter()).collect::<Vec<char>>() }).
        expect("table cant be empty");

    let table_get = |i, j| -> Option<&char>{return table.get(i as usize * n + j as usize)};

    let is_ok = |i:i32, j:i32| {min(i, j) >= 0 && i < n as i32 && j < m as i32 };
    let has_symbol = |i:i32, j:i32| {
        let ch = *table_get(i, j).expect("should be given correct indices");
        return ch != '.' && !is_digit(ch)
    };
    let mut res:i64 = 0;
    for i in 0..n {
        let mut num:i64 = -1;
        let mut num_symbol = false;
        for j in 0..m {
            let ch = *table_get(i as i32, j as i32).expect("indices should be valid");
            if is_digit(ch) {
                if num == -1 {
                    num = (ch as u8 - ('0' as u8)) as i64;
                } else {
                    num = num * 10 + ((ch as u8 - ('0' as u8)) as i64);
                }
                for d_i in (-1..=1).collect::<Vec<i32>>() {
                    for d_j in (-1..=1).collect::<Vec<i32>>()  {
                        let nei_i:i32 = i as i32 + d_i;
                        let nei_j:i32 = j as i32 + d_j;
                        if is_ok(nei_i, nei_j as i32) && has_symbol(nei_i, nei_j)  {
                            num_symbol = true;
                        }
                    }
                }
            }
            if !is_digit(ch) || j == m-1 {
                if num != -1{
                    if num_symbol {
                        res += num
                    }
                    num = -1;
                    num_symbol = false;
                }
            }
        }
    }
    println!("{}", res)
}

fn main() {
    let input = include_str!("../input.txt");
    let tags = input.clone().chars().filter(|ch| ch == &'#')
        .fold(0i64, |cnt, _ch| {cnt+1});

    let table = input.lines()
        .map(|a| a.chars().collect::<Vec<char>>())
        .collect::<Vec<_>>();

    let n = table.len();
    let m = table.first().unwrap().len();

    let get_char = |i:usize, j:usize| -> char {*table.get(i).unwrap().get(j).unwrap()};

    let mut seen_tags_cnt = 0i64;
    let mut res = 0i64;

    for i in 0..n {
        res += seen_tags_cnt * (tags - seen_tags_cnt);

        let mut seen_tag = false;
        for j in 0..m {
            if get_char(i, j) == '#' {
                seen_tag = true;
                seen_tags_cnt += 1;
            }
        }
        if !seen_tag {
            res += seen_tags_cnt * (tags - seen_tags_cnt);
        }
    }

    seen_tags_cnt = 0;
    for j in 0..m {
        res += seen_tags_cnt * (tags - seen_tags_cnt);

        let mut seen_tag = false;
        for i in 0..n {
            if get_char(i, j) == '#' {
                seen_tag = true;
                seen_tags_cnt += 1;
            }
        }
        if !seen_tag {
            res += seen_tags_cnt * (tags - seen_tags_cnt);
        }
    }

    println!("{}", res)
}

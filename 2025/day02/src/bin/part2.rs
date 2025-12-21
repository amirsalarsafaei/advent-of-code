use day02::{get_ranges_from_file};

fn main() {
    let mut sum = 0;
    get_ranges_from_file("input/input.txt").unwrap().iter().
        for_each(|(start, end)| {
            for i in *start..=*end {
                let s = i.to_string();

                for chunk in 1..s.len() {
                    if s.len() % chunk != 0 {
                        continue
                    }
                    let chuck_cnt = s.len() / chunk;
                    let is_valid = (0..chuck_cnt).fold(true, |acc, n| {
                        let part1 = &s[n*chunk..(n+1)*chunk];
                        let part2 = &s[0..chunk];
                        acc && part1 == part2
                    });
                    if is_valid {
                        sum += i;
                        break;
                    }
                }

            }
       
    });

    println!("The sum of ids is : {}", sum)
}


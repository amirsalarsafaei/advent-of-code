use day02::{get_ranges_from_file};

fn main() {
    let mut sum = 0;
    get_ranges_from_file("input/input.txt").unwrap().iter().
        for_each(|(start, end)| {
            for i in *start..=*end {
                let s = i.to_string();

                let chars: Vec<char> = s.chars().collect();

                

                for chunk in 1..s.len() {
                    if s.len() % chunk != 0 {
                        continue
                    }
                }

            }
       
    });

    println!("The sum of ids is : {}", sum)
}


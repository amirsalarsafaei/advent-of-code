use day02::{get_ranges_from_file};

fn main() {
    let mut sum = 0;
    get_ranges_from_file("input/input.txt").unwrap().iter().
        for_each(|(start, end)| {
            for i in *start..=*end {
                let s = i.to_string();
                let (first_half, second_half) = s.split_at(s.len() / 2);
                if first_half == second_half {
                    sum += i;
                }
            }
       
    });

    println!("The sum of ids is : {}", sum)
}

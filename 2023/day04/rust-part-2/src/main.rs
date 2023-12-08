fn main() {
    let input = include_str!("../input.txt");
    let mut matches: Vec<usize> = input.split("\n").map(|card| {
                 let (card_numbers_str, winning_numbers_str) =
                     card.split_once(":").unwrap().1.split_once('|').unwrap();
                 let (card_numbers , winning_numbers) =
                     (card_numbers_str.split_whitespace(),
                      winning_numbers_str.split_whitespace());

                 return card_numbers.filter(|num| -> bool {
                     return !winning_numbers.clone().find(|a| {return num == a}).is_none()
                 }).count();
             }).collect();
    for (i, ms) in matches.clone().into_iter().enumerate().rev() {
        if 0usize < ms {
            matches[i] = matches[(i + 1)..=(i + ms)].to_vec().into_iter().reduce(|a, b| a + b).unwrap() + 1;
        } else {
            matches[i] = 1;
        }
    }

    println!("{:?}", matches.into_iter().reduce(|a, b| a + b).unwrap())
}

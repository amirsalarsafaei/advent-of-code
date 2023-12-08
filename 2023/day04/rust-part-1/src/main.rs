fn main() {
    let input = include_str!("../input.txt");
    println!("{:?}",
             input.split("\n").map(|card| -> i32 {
                 let (card_numbers_str, winning_numbers_str) =
                     card.split_once(":").unwrap().1.split_once('|').unwrap();
                 let (card_numbers , winning_numbers) =
                     (card_numbers_str.split_whitespace(),
                      winning_numbers_str.split_whitespace());

                 let matches = card_numbers.filter(|num| -> bool {
                     return !winning_numbers.clone().find(|a| {return num == a}).is_none()
                 }).count();

                 eprintln!("{:?}", matches);

                 return if matches == 0 {
                     0
                 } else {
                     1 << (matches - 1)
                 }
             }).reduce(|x, y| {return x + y}).unwrap()
    )
}

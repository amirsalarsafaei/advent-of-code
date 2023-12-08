fn main() {
    let input = include_str!("../input.txt");
    let (seeds_str, maps_str) = input.split_once("\n\n").unwrap();

    let mut seeds: Vec<i64> = seeds_str.split_once(": ").unwrap().1
        .split_whitespace().map(|num| {atoi::atoi(num.as_bytes()).unwrap()}).collect();


    maps_str.split("\n\n").into_iter().for_each(
        |map_str| {
            let tmp_seeds = seeds.clone();
            map_str.split_once("map:\n").unwrap().1.split("\n").
                for_each(|mapping_range_str| {
                    let mapping_range: Vec<i64> = mapping_range_str.split_whitespace().
                        map(|num| {atoi::atoi(num.as_bytes()).unwrap()}).collect();
                    let (dest_st, range_st, range_en) =
                        (mapping_range[0], mapping_range[1], mapping_range[1] + mapping_range[2]);

                    tmp_seeds.iter().enumerate().for_each(
                        |(i, &seed)| {
                            if range_st <= seed && seed < range_en {
                                seeds[i] = dest_st + (seed - range_st);
                            }
                        }
                    )
                });
        }
    );

    println!("{:?}", seeds.iter().min().unwrap())
}

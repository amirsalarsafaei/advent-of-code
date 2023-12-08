use std::cmp::{max, min};

fn main() {
    let input = include_str!("../input.txt");
    let (seeds_str, maps_str) = input.split_once("\n\n").unwrap();

    let seeds: Vec<i64> = seeds_str.split_once(": ").unwrap().1
        .split_whitespace().map(|num| {atoi::atoi(num.as_bytes()).unwrap()}).collect();


    let mut seed_ranges: Vec<(i64, i64)> = seeds.chunks_exact(2).map(|ch| {
        return (ch[0], ch[0] + ch[1])
    }).collect();

    maps_str.split("\n\n").into_iter().for_each(
        |map_str| {
            let mut tmp_seed_ranges = seed_ranges.clone();
            seed_ranges.clear();
            map_str.split_once("map:\n").unwrap().1.split("\n").
                for_each(|mapping_range_str| {
                    let mapping_range: Vec<i64> = mapping_range_str.split_whitespace().
                        map(|num| {atoi::atoi(num.as_bytes()).unwrap()}).collect();
                    let (dest_st, range_st, range_en) =
                        (mapping_range[0], mapping_range[1], mapping_range[1] + mapping_range[2]);
                    let tmp_seed_ranges_iter = tmp_seed_ranges.clone();
                    tmp_seed_ranges.clear();
                    tmp_seed_ranges_iter.iter().for_each(
                        | &seed_range| {
                            let intersection = range_intersection(
                                seed_range,
                                (range_st, range_en)
                            );
                            match intersection {
                                Some(inter) => {
                                    seed_ranges.push((inter.0 - range_st + dest_st,
                                                      inter.1 - range_st + dest_st));
                                    tmp_seed_ranges.push((seed_range.0, inter.0));
                                    tmp_seed_ranges.push((inter.1, seed_range.1));
                                }
                                None => {
                                    tmp_seed_ranges.push(seed_range);
                                }
                            }
                        }
                    )
                });
            seed_ranges.append(&mut tmp_seed_ranges);
            seed_ranges.retain(|a| {a.0 < a.1});
        }
    );

    println!("{:?}",
             seed_ranges.iter().min_by(|a, b| a.0.cmp(&b.0)).unwrap().0)
}


fn range_intersection<I:Ord>((first_range_st, first_range_en) : (I, I),
                         (second_range_st, second_range_en) : (I, I)) -> Option<(I, I)>
{
    let intersection = (max(first_range_st, second_range_st),
                        min(first_range_en, second_range_en));
    return if intersection.0 >= intersection.1 {
        None
    } else {
        Some(intersection)
    }
}
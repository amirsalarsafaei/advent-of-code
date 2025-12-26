use std::cmp::max;

use day09::{InputType, get_tiles};
use itertools::Itertools;

fn main() {
    let tiles = get_tiles(InputType::Test)
        .expect("failed to get tiles");

    let res = (0..tiles.len())
        .tuple_combinations()
        .fold(0, |fmax, (i, j)| {
            let area = (tiles[i].x.abs_diff(tiles[j].x) + 1) * 
                (tiles[i].y.abs_diff(tiles[j].y) + 1);
            max(fmax, area)
        });

    println!("res: {}", res);
}

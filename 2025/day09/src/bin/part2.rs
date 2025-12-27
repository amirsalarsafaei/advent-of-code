use std::cmp::{max, min};

use day09::{get_tiles, InputType};
use itertools::Itertools;

fn main() {
    let tiles = get_tiles(InputType::Test).expect("failed to get tiles");

    let res = (0..tiles.len())
        .tuple_combinations()
        .fold(0, |f_max, (i, j)| {
            let tile_i = tiles[i];
            let tile_j = tiles[j];
            let min_rect_x = min(tile_i.x, tile_j.x);
            let max_rect_x = max(tile_i.x, tile_j.x);
            let min_rect_y = min(tile_i.y, tile_j.y);
            let max_rect_y = max(tile_i.y, tile_j.y);

            let mut is_valid = true;
            (0..tiles.len())
                .chain(std::iter::once(0))
                .tuple_windows()
                .for_each(|(k, o)| {
                    let tile_k = tiles[k];
                    let tile_o = tiles[o];

                    if tile_o.x == tile_k.x {
                        let x = tile_o.x;
                        let min_y = min(tile_o.y, tile_k.y);
                        let max_y = max(tile_o.y, tile_k.y);
                        if x > min_rect_x && x < max_rect_x && 
                            min(max_y, max_rect_y) - max(min_y, min_rect_y) >= 0
                        {
                            is_valid = false;
                            return;
                        }
                    } else {
                        let y = tile_o.y;
                        let min_x = min(tile_o.x, tile_k.x);
                        let max_x = max(tile_o.x, tile_k.x);
                        if y > min_rect_y && y < max_rect_y && 
                            min(max_x, max_rect_x) - max(min_x, min_rect_x) >= 0
                        {
                            is_valid = false;
                            return;
                        }
                    }
                });
            if is_valid {
                max(
                    f_max,
                    (tile_i.x.abs_diff(tile_j.x) + 1) * (tile_i.y.abs_diff(tile_j.y) + 1),
                )
            } else {
                f_max
            }
        });
    println!("res: {}", res);
}

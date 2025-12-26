use std::cmp::{max, min};

use day09::{get_tiles, InputType};
use itertools::Itertools;

fn main() {
    let tiles = get_tiles(InputType::Sample).expect("failed to get tiles");

    let res = (0..tiles.len())
        .tuple_combinations()
        .fold(0, |f_max, (i, j)| {
            let tile_i = tiles[i];
            let tile_j = tiles[j];
            let horiz_lines = [
                (tile_i.x, (tile_i.y, tile_j.y)),
                (tile_j.x, (tile_i.y, tile_j.y)),
            ];
            let vert_lines = [
                (tile_i.y, (tile_i.x, tile_j.x)),
                (tile_j.y, (tile_i.x, tile_j.x)),
            ];

            let mut is_valid = true;
            (0..tiles.len())
                .chain(std::iter::once(0))
                .tuple_windows()
                .for_each(|(k, x)| {
                    let tile_k = tiles[k];
                    let tile_x = tiles[x];
                    for (x, (y1, y2)) in horiz_lines {
                        if x > min(tile_k.x, tile_x.x)
                            && x < max(tile_k.x, tile_x.x)
                            && min(max(y1, y2), max(tile_k.y, tile_x.y))
                                - max(min(y1, y1), min(tile_k.y, tile_x.y))
                                >= 0
                        {
                            is_valid = false;
                            break;
                        }
                    }
                    for (y, (x1, x2)) in vert_lines {
                        if y > min(tile_k.y, tile_x.y)
                            && y < max(tile_k.y, tile_x.y)
                            && min(max(x1, x2), max(tile_k.x, tile_x.x))
                                - max(min(x1, x2), min(tile_k.x, tile_x.x))
                                >= 0
                        {
                            is_valid = false;
                            break;
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

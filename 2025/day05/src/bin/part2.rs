use day05::{get_database, InputType};
use std::cmp::max;
fn main() {
    let mut db = get_database(InputType::Test)
        .expect("failed to get kitchen database");

    db.fresh_ranges.sort();

    let mut max_r: i64 = 0;
    let mut total_ids = 0;

    for (l, r) in db.fresh_ranges {
        total_ids += max(0, r - max(l, max_r) + 1);
        max_r = max(max_r, r + 1);
    }

    println!("Total fresh ingredient IDs: {}", total_ids);
}

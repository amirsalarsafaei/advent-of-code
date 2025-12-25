use day05::{get_database, InputType};

fn main() {
    let db = get_database(InputType::Test)
        .expect("failed to get kitchen database");

    let fresh_ingredients = db.available_ingredients.iter().filter(|ingredient| {
        return db.fresh_ranges.iter().any(|(start, end)| {
            *ingredient >= start && *ingredient <= end
        });
    }).count();

    println!("Total fresh ingredients: {}", fresh_ingredients);
}

use day04::{get_paper_grid, InputType, GridCell};
fn main() {
    let paper_grid = get_paper_grid(InputType::Test).expect("Failed to get paper grid");

    let d: Vec<(i32, i32)> = (-1..=1)
        .flat_map(|x| (-1..=1).map(move |y| (x, y)))
        .filter(|(x, y)| *x != 0 || *y != 0)
        .collect();

    let valid_position = |row: i32, col: i32| -> bool {
        row >= 0 && col >= 0 && (row as usize) < paper_grid.len() && (col as usize) < paper_grid[0].len()
    };

    let is_paper_cell = |row: i32, col: i32| -> bool {
        if valid_position(row, col) {
            return paper_grid[row as usize][col as usize] == GridCell::Paper;
        }
        false
    };

    let total_count = (0..paper_grid.len()).fold(0, |count, row| {
        count + (0..paper_grid[row].len()).fold(0, |count, col| {
            let cell = paper_grid[row][col];
            if cell == GridCell::Paper {
                let reachable = d.iter().map(|(dx, dy)| (row as i32 + dx, col as i32 + dy)).
                    filter(|(nei_row, nei_col)| is_paper_cell(*nei_row, *nei_col)).count() < 4;
                if reachable {
                    return count + 1;
                }
            }
            return count
        })

    });

    println!("Total reachable paper cells: {}", total_count);
}

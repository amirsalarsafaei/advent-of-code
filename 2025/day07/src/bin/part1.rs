use day07::{InputType, get_grid, GridCell};

fn is_beam(cell: &GridCell) -> bool {
    match cell {
        GridCell::Beam | GridCell::Start  => true,
        _ => false,
    }
}


fn main() {
    let mut grid = get_grid(InputType::Test)
        .expect("read grid failed");

    
    let rows = grid.len();
    let cols = grid.get(0).expect("could not find first row").len();

    let is_valid = |i: i32, j: i32| {
        return i < rows as i32 && j < cols as i32 && j >= 0 && i >= 0;
    };

    let mut splits = 0;

    for i in 1..rows {
        for j in 0..cols {
            match grid[i][j] {
                GridCell::Empty =>  {
                    if is_beam(&grid[i-1][j]) {
                        grid[i][j] = GridCell::Beam;
                    }
                }
                GridCell::Splitter => {
                    if is_beam(&grid[i-1][j]) {
                        splits += 1;
                        if is_valid(i as i32, j as i32 - 1) {
                            grid[i][j-1] = GridCell::Beam;
                        }
                        if is_valid(i as i32, j as i32 +1) {
                            grid[i][j+1] = GridCell::Beam;
                        }
                    }
                }
                _ => {},
            };
        }
    }

    println!("Number of splitters activated: {}", splits);
}

use day04::{get_paper_grid, InputType, GridCell};

struct Solver {
    paper_grid: Vec<Vec<GridCell>>,
    nei_count: Vec<Vec<usize>>,
    d: Vec<(i32, i32)>,
    counter: usize,
}

impl Solver {
    fn new(input: InputType) -> Self {
        Self {
            paper_grid: get_paper_grid(input).expect("Failed to get paper grid"),
            nei_count: vec![vec![0; 0]; 0],
            d: (-1..=1)
                .flat_map(|x| (-1..=1).map(move |y| (x, y)))
                .filter(|(x, y)| *x != 0 || *y != 0)
                .collect(),
            counter: 0,
        }
    }

    fn run(&mut self) -> i64{
        self.counter = 0;
        self.count_initial_neighbours();

        for row in 0..self.paper_grid.len() {
            for col in 0..self.paper_grid[row].len() {
                if self.paper_grid[row][col] == GridCell::Paper
                    && self.nei_count[row][col] < 4
                {
                    self.remove_paper_cell(
                        row,
                        col,
                    );
                }
            }
        }

        return self.counter as i64;
    }

    fn count_initial_neighbours(&mut self) {
        self.nei_count = (0..self.paper_grid.len())
            .map(|row| {
                (0..self.paper_grid[row].len())
                    .map(|col| {
                        self.d.iter()
                            .map(|(dx, dy)| (row as i32 + dx, col as i32 + dy))
                            .filter(|(r, c)| self.is_paper_cell(*r, *c))
                            .count()
                    })
                    .collect()
            })
            .collect();
    }

    fn is_valid_position(&self, row: i32, col: i32) -> bool {
        row >= 0 && col >= 0 && (row as usize) < self.paper_grid.len() && (col as usize) < self.paper_grid[0].len()
    }

    fn is_paper_cell(&self, row: i32, col: i32) -> bool {
        if self.is_valid_position(row, col) {
            return self.paper_grid[row as usize][col as usize] == GridCell::Paper;
        }
        false
    }

    fn remove_paper_cell(&mut self, row: usize, col: usize) {
        if self.paper_grid[row][col] != GridCell::Paper {
            return;
        }
        self.counter += 1;

        self.paper_grid[row][col] = GridCell::Removed;
        self.d.clone().iter().map(|(dx, dy)| (row as i32 + dx, col as i32 + dy))
            .for_each(|(nei_row, nei_col)| {
                if self.is_paper_cell( nei_row, nei_col) {
                    self.nei_count[nei_row as usize][nei_col as usize] -= 1;
                    if self.nei_count[nei_row as usize][nei_col as usize] < 4 {
                        self.remove_paper_cell( nei_row as usize, nei_col as usize);
                    }
                }
            }
        );
    }

}

fn main() {
    println!("Answer: {}", Solver::new(InputType::Test).run());
}


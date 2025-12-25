use day06::{get_worksheet, InputType, Operation, InputMode};
fn main() {
    let worksheet = get_worksheet(InputType::Test, InputMode::Part1)
        .expect("read worksheet failed");


    let total_sum = worksheet.problems.iter().map(|problem| {
        match problem.operation {
            Operation::Sum => problem.nums.iter().sum::<u64>(),
            Operation::Product => problem.nums.iter().product::<u64>(),
        }
    }).sum::<u64>();

    println!("Total result: {}", total_sum);
}

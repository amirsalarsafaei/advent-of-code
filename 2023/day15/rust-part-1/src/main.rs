fn main() {
    println!("{}", include_str!("../../sample.txt")
        .split(",")
        .map(
            |s| {
                s.chars().map(|a| a as i32).collect::<Vec<_>>()
            }
        ).map(
            |a| {
                a.iter().fold(0i64,
                              |b, c| {
                                  return (((b + *c as i64) * 17) % 256) as i64;
                              },
                )
            }
        ).fold(0i64, |a, b| a + b)
    )
}

pub fn get_ranges_from_file(file_path: &str) -> Result<Vec<(u64, u64)>, std::io::Error> {
    let content = std::fs::read_to_string(file_path)?;
    let ranges = content.lines().next().unwrap().split(',').map(|range| {
        let parts: Vec<&str> = range.split('-').collect();
        if parts.len() != 2 {
            panic!("Invalid range format");
        }
        let start = parts[0].parse::<u64>().expect("Invalid start of range");
        let end = parts[1].parse::<u64>().expect("Invalid end of range");
        (start, end)
    }).collect();
    Ok(ranges)
}

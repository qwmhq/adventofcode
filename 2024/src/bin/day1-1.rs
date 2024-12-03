use std::fs::File;
use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    let path = "inputs/day1";
    let mut left_list = Vec::new();
    let mut right_list = Vec::new();
    let mut distance_sum = 0;

    let file = File::open(path)?;
    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        let values: Vec<u64> = line
            .split_whitespace()
            .filter_map(|s| s.parse::<u64>().ok())
            .collect();
        left_list.push(values[0]);
        right_list.push(values[1]);
    }

    left_list.sort();
    right_list.sort();

    for i in 0..left_list.len() {
        let dist = (left_list[i] as i64 - right_list[i] as i64).unsigned_abs();
        distance_sum += dist;
    }

    println!("sum of distances: {distance_sum}");

    Ok(())
}

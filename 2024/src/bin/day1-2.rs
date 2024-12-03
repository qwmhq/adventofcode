use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    let path = "inputs/day1";
    let mut left_list = Vec::new();
    let mut right_dict = HashMap::new();
    let mut similarity_score = 0;

    let file = File::open(path)?;
    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        let values: Vec<u64> = line
            .split_whitespace()
            .filter_map(|s| s.parse::<u64>().ok())
            .collect();
        left_list.push(values[0]);
        right_dict
            .entry(values[1])
            .and_modify(|x| *x += 1)
            .or_insert(1);
    }

    for i in left_list {
        if let Some(x) = right_dict.get(&i) {
            similarity_score += x * i;
        }
    }

    println!("similarity score: {similarity_score}");

    Ok(())
}

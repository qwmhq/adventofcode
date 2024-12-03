use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    let path = "inputs/day3";
    let mut result = 0;

    let file = File::open(path)?;
    let reader = io::BufReader::new(file);

    let re = Regex::new(r"(do\(\))|(don't\(\))|mul\(([0-9]+,[0-9]+)\)").unwrap();

    let mut mul_enabled = true;
    for line in reader.lines() {
        let line = line?;
        for captures in re.captures_iter(&line) {
            if let Some(_) = captures.get(1) {
                mul_enabled = true;
            } else if let Some(_) = captures.get(2) {
                mul_enabled = false;
            } else if let Some(capture) = captures.get(3) {
                if mul_enabled {
                    let operands: Vec<_> = capture
                        .as_str()
                        .split(',')
                        .map(|s| s.parse::<u64>().unwrap())
                        .collect();
                    result += operands[0] * operands[1];
                }
            }
        }
    }

    println!("result: {result}");

    Ok(())
}

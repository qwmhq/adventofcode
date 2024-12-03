use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    let path = "input";
    let mut result = 0;

    let file = File::open(path)?;
    let reader = io::BufReader::new(file);

    let re = Regex::new(r"mul\(([0-9]+),([0-9]+)\)").unwrap();

    for line in reader.lines() {
        let line = line?;
        for (_, [f1, f2]) in re.captures_iter(&line).map(|caps| caps.extract()) {
            result += f1.parse::<u64>().unwrap() * f2.parse::<u64>().unwrap();
        }
    }

    println!("result: {result}");

    Ok(())
}

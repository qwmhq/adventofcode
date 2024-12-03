use std::fs::File;
use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    let path = "inputs/day2";
    let mut reports = Vec::new();
    let mut safe_reports = 0;

    let file = File::open(path)?;
    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        let values: Vec<u64> = line
            .split_whitespace()
            .filter_map(|s| s.parse::<u64>().ok())
            .collect();
        reports.push(values);
    }

    for report in reports {
        if is_safe(&report) {
            safe_reports += 1;
        }
    }

    println!("safe reports: {safe_reports}");

    Ok(())
}

fn is_safe(report: &[u64]) -> bool {
    let increasing = report[1] > report[0];
    for i in 1..report.len() {
        if (increasing && report[i] < report[i - 1])
            || (!increasing && report[i] > report[i - 1])
            || (!(1..=3).contains(&report[i].abs_diff(report[i - 1])))
        {
            return false;
        }
    }
    true
}

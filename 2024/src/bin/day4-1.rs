use std::fs::File;
use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    let path = "inputs/day4";
    let mut result = 0;

    let file = File::open(path)?;
    let reader = io::BufReader::new(file);

    let mut vertical_lines = Vec::new();
    let mut diagonal_lines_lr = Vec::new();
    let mut diagonal_lines_rl = Vec::new();

    for (i, line) in reader.lines().enumerate() {
        let line = line?;
        result += line.matches("XMAS").count();
        result += line.matches("SAMX").count();

        for (j, ch) in line.chars().enumerate() {
            if i == 0 {
                vertical_lines.push(String::from(ch));
            } else {
                vertical_lines[j].push(ch);
            }

            if i <= j {
                if i == 0 {
                    diagonal_lines_lr.push(String::from(ch));
                } else {
                    diagonal_lines_lr[j - i].push(ch);
                }
            } else {
                if j == 0 {
                    diagonal_lines_lr.push(String::from(ch));
                } else {
                    diagonal_lines_lr[line.len() + i - j - 1].push(ch);
                }
            }
        }

        for (j, ch) in line.chars().rev().enumerate() {
            if i <= j {
                if i == 0 {
                    diagonal_lines_rl.push(String::from(ch));
                } else {
                    diagonal_lines_rl[j - i].push(ch);
                }
            } else {
                if j == 0 {
                    diagonal_lines_rl.push(String::from(ch));
                } else {
                    diagonal_lines_rl[line.len() + i - j - 1].push(ch);
                }
            }
        }
    }

    for lines in [vertical_lines, diagonal_lines_lr, diagonal_lines_rl] {
        for line in &lines {
            result += line.matches("XMAS").count();
            result += line.matches("SAMX").count();
        }
    }

    println!("result: {result}");

    Ok(())
}

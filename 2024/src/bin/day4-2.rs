use std::fs::File;
use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    let path = "inputs/day4";
    let mut result = 0;

    let file = File::open(path)?;
    let reader = io::BufReader::new(file);

    let mut matrix = Vec::new();

    for line in reader.lines() {
        let line = line?;
        matrix.push(line.chars().collect::<Vec<_>>());
    }

    for i in 1..matrix.len() - 1 {
        for j in 1..matrix[i].len() - 1 {
            if matrix[i][j] != 'A' {
                continue;
            }

            let mut s1 = String::new();
            s1.push(matrix[i - 1][j - 1]);
            s1.push(matrix[i][j]);
            s1.push(matrix[i + 1][j + 1]);
            if s1 != "MAS" && s1 != "SAM" {
                continue;
            }

            let mut s2 = String::new();
            s2.push(matrix[i - 1][j + 1]);
            s2.push(matrix[i][j]);
            s2.push(matrix[i + 1][j - 1]);
            if s2 != "MAS" && s2 != "SAM" {
                continue;
            }

            result += 1;
        }
    }

    println!("result: {result}");

    Ok(())
}

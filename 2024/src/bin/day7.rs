use std::fs::File;
use std::io::{self, BufRead};

use itertools::{repeat_n, Itertools};

fn main() {
    let input_path = "inputs/day7";

    let equations = parse_input(input_path);

    println!(
        "part 1: total calibration result: {:?}",
        solve_part1(&equations)
    );

    println!(
        "part 2: total calibration result: {:?}",
        solve_part2(&equations)
    );
}

fn solve_part1(equations: &Vec<(u64, Vec<u64>)>) -> u64 {
    let operators = ["+", "*"];

    let mut calibration_result = 0;
    for eq in equations.iter() {
        let (lhs, rhs) = eq;
        for perm in repeat_n(operators.iter(), rhs.len() - 1).multi_cartesian_product() {
            let eq_result = apply_operators(rhs, perm).unwrap();
            if eq_result == *lhs {
                calibration_result += lhs;
                break;
            }
        }
    }
    calibration_result
}

fn solve_part2(equations: &Vec<(u64, Vec<u64>)>) -> u64 {
    let operators = ["+", "*", "||"];
    let mut calibration_result = 0;
    for eq in equations.iter() {
        let (lhs, rhs) = eq;
        for perm in repeat_n(operators.iter(), rhs.len() - 1).multi_cartesian_product() {
            let eq_result = apply_operators(rhs, perm).unwrap();
            if eq_result == *lhs {
                calibration_result += lhs;
                break;
            }
        }
    }
    calibration_result
}

fn apply_operators(values: &Vec<u64>, operators: Vec<&&str>) -> Option<u64> {
    if values.len() - 1 != operators.len() {
        return None;
    }

    let mut values = values.clone();
    let mut operators = operators;

    while let Some(op) = operators.pop() {
        let x = values.pop().unwrap();
        let y = values.pop().unwrap();
        if *op == "+" {
            values.push(x + y);
        } else if *op == "*" {
            values.push(x * y);
        } else if *op == "||" {
            let exp = ((y as f64).log10() + 1.0).floor();
            values.push((x * 10_u64.pow(exp as u32)) + y);
        }
    }
    Some(values[0])
}

fn parse_input(path: &str) -> Vec<(u64, Vec<u64>)> {
    let file = File::open(path).expect("unable to read input file");
    let reader = io::BufReader::new(file);

    reader
        .lines()
        .map(|line| {
            let line = line.expect("couldn't read line");
            let eq = line.split(':').collect::<Vec<_>>();
            let lhs = eq[0].parse::<u64>().expect("couldn't parse numbers");
            let rhs = eq[1]
                .split_whitespace()
                .map(|x| x.parse::<u64>().expect("couldn't parse numbers"))
                .rev()
                .collect::<Vec<_>>();
            return (lhs, rhs);
        })
        .collect::<Vec<_>>()
}

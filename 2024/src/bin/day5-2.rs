use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    let path = "inputs/day5";
    let mut result = 0;

    let file = File::open(path)?;
    let reader = io::BufReader::new(file);

    let mut dict: HashMap<u64, Vec<u64>> = HashMap::new();

    let mut lines = reader.lines();
    while let Some(line) = lines.next() {
        let line = line?;

        if line.is_empty() {
            break;
        }

        let values = line
            .split('|')
            .map(|x| x.parse::<u64>().unwrap())
            .collect::<Vec<_>>();

        dict.entry(values[0])
            .and_modify(|x| x.push(values[1]))
            .or_insert(vec![values[1]]);
    }

    'outer: while let Some(line) = lines.next() {
        let line = line?;
        let mut values = line
            .split(',')
            .map(|x| x.parse::<u64>().unwrap())
            .collect::<Vec<_>>();

        let compare_fn = |a: &u64, b: &u64| {
            if dict.get(a).is_some_and(|successors| successors.contains(b)) {
                return Ordering::Less;
            } else if dict.get(b).is_some_and(|successors| successors.contains(a)) {
                return Ordering::Greater;
            } else {
                return Ordering::Equal;
            }
        };

        for (i, value) in values.iter().enumerate() {
            if let Some(successors) = dict.get(value) {
                for j in 0..i {
                    if successors.contains(&values[j]) {
                        values.sort_by(compare_fn);
                        result += values[values.len() / 2];
                        continue 'outer;
                    }
                }
            }
        }
    }

    println!("result: {result}");

    Ok(())
}

use std::{collections::HashMap, fs::File, io::Read};

fn main() {
    let path = "inputs/day11";
    let stones = parse_input(path);

    println!(
        "part 1: stones after blinking 25 times: {:?}",
        solve_part1(&stones)
    );

    println!(
        "part 2: stones after blinking 75 times: {:?}",
        solve_part2(&stones)
    );
}

type Stone = u64;

fn parse_input(path: &str) -> HashMap<Stone, u64> {
    let mut file = File::open(path).expect("couldn't open input file");
    let mut buf = String::new();
    file.read_to_string(&mut buf).expect("problem reading file");

    let mut stones = HashMap::new();

    buf.split_whitespace()
        .filter_map(|x| x.parse::<u64>().ok())
        .for_each(|s| {
            stones.entry(s).and_modify(|count| *count += 1).or_insert(1);
        });
    stones
}

fn digits(x: u64) -> u64 {
    ((x as f64).log10() + 1.0).floor() as u64
}

fn split(x: u64) -> (u64, u64) {
    let exp = (digits(x) / 2) as u32;
    let divisor = 10_u64.pow(exp);
    (x / divisor, x % divisor)
}

fn blink(stones: HashMap<Stone, u64>) -> HashMap<Stone, u64> {
    let mut new_stones = HashMap::new();

    for (stone, count) in stones.into_iter() {
        let stone_digits = digits(stone);
        if stone == 0 {
            new_stones
                .entry(1)
                .and_modify(|c| *c += count)
                .or_insert(count);
        } else if stone_digits % 2 == 0 {
            let split = split(stone);
            new_stones
                .entry(split.0)
                .and_modify(|c| *c += count)
                .or_insert(count);
            new_stones
                .entry(split.1)
                .and_modify(|c| *c += count)
                .or_insert(count);
        } else {
            new_stones
                .entry(stone * 2024)
                .and_modify(|c| *c += count)
                .or_insert(count);
        }
    }
    new_stones
}

fn solve_part1(stones: &HashMap<Stone, u64>) -> u64 {
    let mut stones = stones.clone();
    for _ in 1..=25 {
        stones = blink(stones);
    }
    stones.into_values().sum()
}

fn solve_part2(stones: &HashMap<Stone, u64>) -> u64 {
    let mut stones = stones.clone();
    for _ in 1..=75 {
        stones = blink(stones);
    }
    stones.into_values().sum()
}

use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{self, BufRead},
};

use itertools::Itertools;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Point(i32, i32);

fn main() {
    let path = "inputs/day8";

    let map = parse_map(&path);
    let map_width = map[0].len() as i32;
    let map_height = map.len() as i32;
    let antennas = find_antennas(&map);

    println!(
        "part 1: unique antinode locations: {:?}",
        solve_part1(&antennas, map_width, map_height)
    );
    println!(
        "part 2: unique antinode locations: {:?}",
        solve_part2(&antennas, map_width, map_height)
    );
}

fn solve_part1(antennas: &HashMap<char, Vec<Point>>, map_width: i32, map_height: i32) -> u64 {
    let mut antinodes: HashSet<Point> = HashSet::new();

    for (_antenna, locations) in antennas.iter() {
        for pair in locations.iter().combinations(2) {
            get_antinodes((pair[0], pair[1])).iter().for_each(|p| {
                if p.0 >= 0 && p.0 < map_width && p.1 >= 0 && p.1 < map_height {
                    antinodes.insert(*p);
                }
            });
        }
    }
    antinodes.len() as u64
}

fn get_antinodes(pair: (&Point, &Point)) -> Vec<Point> {
    let (a, b) = pair;
    let diff = (a.0 - b.0, a.1 - b.1);
    vec![
        Point(a.0 + diff.0, a.1 + diff.1),
        Point(b.0 - diff.0, b.1 - diff.1),
    ]
}

fn solve_part2(antennas: &HashMap<char, Vec<Point>>, map_width: i32, map_height: i32) -> u64 {
    let mut antinodes: HashSet<Point> = HashSet::new();

    for (_antenna, locations) in antennas.iter() {
        for pair in locations.iter().combinations(2) {
            get_antinodes_using_updated_model((pair[0], pair[1]), map_width, map_height)
                .iter()
                .for_each(|p| {
                    antinodes.insert(*p);
                });
        }
    }
    antinodes.len() as u64
}

fn get_antinodes_using_updated_model(
    pair: (&Point, &Point),
    map_width: i32,
    map_height: i32,
) -> Vec<Point> {
    let (a, b) = pair;
    let mut antinodes = Vec::new();

    let diff = reduce((a.0 - b.0, a.1 - b.1));

    let mut point = *a;
    while point.0 >= 0 && point.0 < map_width && point.1 >= 0 && point.1 < map_height {
        antinodes.push(point);
        point = Point(point.0 + diff.0, point.1 + diff.1);
    }

    let mut point = *a;
    while point.0 >= 0 && point.0 < map_width && point.1 >= 0 && point.1 < map_height {
        antinodes.push(point);
        point = Point(point.0 - diff.0, point.1 - diff.1);
    }
    antinodes
}

fn gcd(a: i32, b: i32) -> i32 {
    if b == 0 {
        a.abs()
    } else {
        gcd(b, a % b)
    }
}

fn reduce(pair: (i32, i32)) -> (i32, i32) {
    let divisor = gcd(pair.0, pair.1);
    (pair.0 / divisor, pair.1 / divisor)
}

fn find_antennas(map: &Vec<Vec<char>>) -> HashMap<char, Vec<Point>> {
    let mut antennas = HashMap::new();
    for y in 0..map.len() {
        for x in 0..map[0].len() {
            if map[y][x] != '.' {
                let point = Point(x as i32, y as i32);
                antennas
                    .entry(map[y][x])
                    .and_modify(|z: &mut Vec<Point>| z.push(point))
                    .or_insert(vec![point]);
            }
        }
    }
    antennas
}

fn parse_map(path: &str) -> Vec<Vec<char>> {
    let file = File::open(path).expect(&format!("unable to open file {path}"));
    let reader = io::BufReader::new(file);

    reader
        .lines()
        .map(|line| line.expect("couldn't parse line").chars().collect())
        .collect()
}

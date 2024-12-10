use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn is_in_map(&self, map: &Vec<Vec<u8>>) -> bool {
        self.x >= 0 && self.x < map[0].len() as i32 && self.y >= 0 && self.y < map.len() as i32
    }

    fn value(&self, map: &Vec<Vec<u8>>) -> Option<u8> {
        if self.is_in_map(map) {
            Some(map[self.y as usize][self.x as usize])
        } else {
            None
        }
    }

    fn viable_neighbours(&self, map: &Vec<Vec<u8>>) -> Vec<Self> {
        [
            Point {
                x: self.x + 1,
                ..*self
            },
            Point {
                x: self.x - 1,
                ..*self
            },
            Point {
                y: self.y + 1,
                ..*self
            },
            Point {
                y: self.y - 1,
                ..*self
            },
        ]
        .into_iter()
        .filter(|p| {
            p.value(map)
                .is_some_and(|v| v == self.value(map).unwrap() + 1)
        })
        .collect()
    }
}

fn main() {
    let path = "inputs/day10";
    let topological_map = parse_map(path);

    println!(
        "part 1: trailheads score: {:?}",
        solve_part1(&topological_map)
    );

    println!(
        "part 2: trailheads ratings: {:?}",
        solve_part2(&topological_map)
    );
}

fn parse_map(path: &str) -> Vec<Vec<u8>> {
    let file = File::open(path).expect("couldn't open file");
    let reader = BufReader::new(file);

    reader
        .lines()
        .map(|line| {
            line.expect("couldn't parse line")
                .chars()
                .filter_map(|c| c.to_digit(10).map(|x| x as u8))
                .collect()
        })
        .collect()
}

fn find_trailhead_score(map: &Vec<Vec<u8>>, head: Point, visited: &mut HashSet<Point>) -> u32 {
    visited.insert(head);

    if head.value(map).is_some_and(|v| v == 9) {
        return 1;
    }

    let viable_neighbours = head
        .viable_neighbours(map)
        .into_iter()
        .filter(|n| !visited.contains(n))
        .collect::<Vec<_>>();

    if viable_neighbours.len() == 0 {
        return 0;
    }

    let score = viable_neighbours
        .into_iter()
        .map(|n| find_trailhead_score(map, n, visited))
        .sum();

    score
}

fn solve_part1(map: &Vec<Vec<u8>>) -> u32 {
    let map_width = map[0].len();
    let map_height = map.len();

    let mut scores_sum = 0;

    for y in 0..map_height {
        for x in 0..map_width {
            if map[y][x] == 0 {
                let mut visited: HashSet<Point> = HashSet::new();
                let score = find_trailhead_score(
                    map,
                    Point {
                        x: x as i32,
                        y: y as i32,
                    },
                    &mut visited,
                );
                scores_sum += score;
            }
        }
    }
    scores_sum
}

fn find_trailhead_rating(
    map: &Vec<Vec<u8>>,
    head: Point,
    score_cache: &mut Vec<Vec<Option<u32>>>,
) -> u32 {
    if let Some(score) = score_cache[head.y as usize][head.x as usize] {
        return score;
    }

    if head.value(map).is_some_and(|v| v == 9) {
        return 1;
    }

    let viable_neighbours = head.viable_neighbours(map).into_iter().collect::<Vec<_>>();

    if viable_neighbours.len() == 0 {
        return 0;
    }

    let score = viable_neighbours
        .into_iter()
        .map(|n| find_trailhead_rating(map, n, score_cache))
        .sum();

    score_cache[head.y as usize][head.x as usize] = Some(score);

    score
}

fn solve_part2(map: &Vec<Vec<u8>>) -> u32 {
    let map_width = map[0].len();
    let map_height = map.len();

    let mut ratings_sum = 0;

    let mut score_cache: Vec<Vec<Option<u32>>> = vec![vec![None; map_width]; map_height];

    for y in 0..map_height {
        for x in 0..map_width {
            if map[y][x] == 0 {
                let score = find_trailhead_rating(
                    map,
                    Point {
                        x: x as i32,
                        y: y as i32,
                    },
                    &mut score_cache,
                );
                ratings_sum += score;
            }
        }
    }
    ratings_sum
}

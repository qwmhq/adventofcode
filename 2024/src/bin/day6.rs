use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead, Write};

fn main() {
    let path = "inputs/day6";

    let map = parse_map(path);

    println!("part 1 result: {:?}", solve_part1(&map));
    println!("part 2 result: {:?}", solve_part2(&map));
}

fn solve_part1(map: &Vec<Vec<char>>) -> u64 {
    let mut map = map.clone();
    let initial_position = get_position(&map).unwrap();
    let mut position = initial_position.clone();

    while let Some(next_position) = position.get_next_on_map(&map) {
        position = next_position;
        set_marker_at_position(&mut map, &position, 'X');
    }
    get_visited_positions(&map)
}

fn print_map_to_file(map: &Vec<Vec<char>>, filepath: &str) {
    // write the final map to an output file
    let mut file = File::create(filepath).expect("couldn't create file");
    for line in map.into_iter().map(|x| String::from_iter(x)) {
        writeln!(file, "{}", line).expect("couldn't write to file");
    }
}

fn mark_map(map: &mut Vec<Vec<char>>, position: &Position, next_position: &Position) {
    let prev_marker = get_marker_at_position(&map, &position);
    let mut new_marker = match position.direction {
        Direction::North | Direction::South => '|',
        Direction::East | Direction::West => '-',
    };
    if position.direction != next_position.direction
        || prev_marker == '|' && new_marker == '-'
        || prev_marker == '-' && new_marker == '|'
    {
        new_marker = '+';
    }
    set_marker_at_position(map, position, new_marker);
}

fn solve_part2(map: &Vec<Vec<char>>) -> u64 {
    let map = map.clone();
    let initial_position = get_position(&map).unwrap();

    // get all the positions that would be visited by the guard normally
    let mut visited_without_obstacles: HashSet<Position> = HashSet::new();
    let mut position = initial_position.clone();
    visited_without_obstacles.insert(initial_position);
    while let Some(next_position) = position.get_next_on_map(&map) {
        visited_without_obstacles.insert(next_position);
        position = next_position;
    }

    let mut obstruction_positions = 0;
    for y in 0..map.len() {
        for x in 0..map[0].len() {
            if initial_position.x as usize == x && initial_position.y as usize == y
                || map[y][x] == '#'
            {
                continue;
            }
            let mut test_map = map.clone();
            test_map[y][x] = 'O';

            let mut p = initial_position;
            let mut visited: HashSet<Position> = HashSet::new();

            visited.insert(p);
            while let Some(next_p) = p.get_next_on_map(&test_map) {
                if visited.contains(&next_p) {
                    obstruction_positions += 1;
                    println!(
                        "(x: {x}, y: {y}); obstruction_positions so far: {obstruction_positions}"
                    );
                    break;
                }
                p = next_p;
                visited.insert(next_p);
            }
        }
    }
    return obstruction_positions;
}

fn position_within_map(map: &Vec<Vec<char>>, position: &Position) -> bool {
    position.x >= 0
        && position.x < map[0].len() as i64
        && position.y >= 0
        && position.y < map.len() as i64
}

fn get_marker_at_position(map: &Vec<Vec<char>>, position: &Position) -> char {
    map[position.y as usize][position.x as usize]
}

fn set_marker_at_position(map: &mut Vec<Vec<char>>, position: &Position, c: char) {
    map[position.y as usize][position.x as usize] = c;
}

fn parse_map(path: &str) -> Vec<Vec<char>> {
    let file = File::open(path).expect("unable to read input file");
    let reader = io::BufReader::new(file);

    let mut map: Vec<Vec<char>> = Vec::new();

    for line in reader.lines() {
        let line = line.expect("problem reading line");
        map.push(line.chars().collect::<Vec<_>>());
    }

    map
}

fn get_position(map: &Vec<Vec<char>>) -> Option<Position> {
    let direction;
    for i in 0..map.len() {
        for j in 0..map[i].len() {
            if "^V<>".contains(map[i][j]) {
                if map[i][j] == '^' {
                    direction = Direction::North;
                } else if map[i][j] == '>' {
                    direction = Direction::East;
                } else if map[i][j] == 'V' {
                    direction = Direction::South;
                } else {
                    direction = Direction::West;
                }
                return Some(Position {
                    x: j as i64,
                    y: i as i64,
                    direction,
                });
            }
        }
    }
    return None;
}

fn get_visited_positions(map: &Vec<Vec<char>>) -> u64 {
    let mut visited_positions = 0;
    for i in 0..map.len() {
        for j in 0..map[i].len() {
            if map[i][j] == 'X' || "^V<>".contains(map[i][j]) {
                visited_positions += 1;
            }
        }
    }
    visited_positions
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Position {
    x: i64,
    y: i64,
    direction: Direction,
}

impl Position {
    fn get_next(&self) -> Self {
        let mut new_position = self.clone();
        match new_position.direction {
            Direction::North => new_position.y -= 1,
            Direction::East => new_position.x += 1,
            Direction::South => new_position.y += 1,
            Direction::West => new_position.x -= 1,
        }
        new_position
    }

    fn get_next_on_map(&self, map: &Vec<Vec<char>>) -> Option<Self> {
        let mut new_position = self.get_next();

        if position_within_map(map, &new_position) {
            if "#O".contains(get_marker_at_position(map, &new_position)) {
                new_position = Position {
                    direction: self.direction.turn_right(),
                    ..*self
                }
                .get_next();
            }
            return Some(new_position);
        }
        return None;
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn turn_right(&self) -> Self {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }
}

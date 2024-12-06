use std::fs::File;
use std::io::{self, BufRead, Write};

fn main() -> io::Result<()> {
    let path = "inputs/day6";

    let map = parse_map(path)?;

    // println!("part 1 result: {:?}", solve_part1(&map));
    println!("part 2 result: {:?}", solve_part2(&map));

    Ok(())
}

fn solve_part1(map: &Vec<Vec<char>>) -> u64 {
    let mut map = map.clone();
    let initial_position = get_position(&map).unwrap();
    let mut position = initial_position.clone();
    loop {
        map[position.y as usize][position.x as usize] = 'X';
        move_once(&map, &mut position);

        if position.x < 0
            || position.x >= map[0].len() as i64
            || position.y < 0
            || position.y >= map.len() as i64
        {
            break;
        }
    }
    get_visited_positions(&map)
}

fn solve_part2(map: &Vec<Vec<char>>) -> u64 {
    // idea is to put an obstacle anywhere that would result in a right turn unto a path that has
    // been taken previously in the same direction that would result from the right turn
    let mut map = map.clone();
    let initial_position = get_position(&map).unwrap();
    let mut position = initial_position.clone();

    let mut obstruction_positions = 0;
    loop {
        let prev_position = position;
        let prev_marker = map[position.y as usize][position.x as usize];
        let mut new_marker = match position.direction {
            Direction::North | Direction::South => '|',
            Direction::East | Direction::West => '-',
        };

        let position_after_right_turn = Position {
            direction: position.direction.turn_right(),
            ..position
        }
        .get_next();

        if position_after_right_turn.x >= 0
            && position_after_right_turn.x < map[0].len() as i64
            && position_after_right_turn.y >= 0
            && position_after_right_turn.y < map.len() as i64
        {
            let marker =
                map[position_after_right_turn.y as usize][position_after_right_turn.x as usize];
            let direction = position_after_right_turn.direction;
            if marker == '+'
                || marker == '|' && (direction == Direction::North || direction == Direction::South)
                || marker == '-' && (direction == Direction::East || direction == Direction::West)
            {
                obstruction_positions += 1;
            }
        }

        move_once(&map, &mut position);

        if prev_position.direction != position.direction
            || prev_marker == '|' && new_marker == '-'
            || prev_marker == '-' && new_marker == '|'
        {
            new_marker = '+';
        }

        map[prev_position.y as usize][prev_position.x as usize] = new_marker;

        if position.x < 0
            || position.x >= map[0].len() as i64
            || position.y < 0
            || position.y >= map.len() as i64
        {
            break;
        }
    }

    // write the final map to an output file
    map[initial_position.y as usize][initial_position.x as usize] = '^';
    let mut file = File::create("output_day6").expect("couldnt create file");
    for line in map.into_iter().map(|x| String::from_iter(x)) {
        writeln!(file, "{}", line).expect("unable to write to file");
    }

    obstruction_positions
}

fn parse_map(path: &str) -> io::Result<Vec<Vec<char>>> {
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);

    let mut map: Vec<Vec<char>> = Vec::new();

    for line in reader.lines() {
        let line = line?;
        map.push(line.chars().collect::<Vec<_>>());
    }

    Ok(map)
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
            if map[i][j] == 'X' {
                visited_positions += 1;
            }
        }
    }
    visited_positions
}

fn move_once(map: &Vec<Vec<char>>, position: &mut Position) {
    let new_position = position.get_next();

    let x = new_position.x;
    let y = new_position.y;

    if x >= 0 && x < map[0].len() as i64 && y >= 0 && y < map.len() as i64 {
        if map[y as usize][x as usize] == '#' {
            position.direction = position.direction.turn_right();
            *position = position.get_next();
            return;
        }
    }

    *position = new_position;
}

#[derive(Clone, Copy, Debug)]
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
}

#[derive(Clone, Copy, Debug, PartialEq)]
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

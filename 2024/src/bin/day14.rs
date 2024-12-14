use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use image::RgbImage;
use regex::Regex;

// const MAP_WIDTH: i32 = 11;
// const MAP_HEIGHT: i32 = 7;

const MAP_WIDTH: i32 = 101;
const MAP_HEIGHT: i32 = 103;

#[derive(Clone, Debug)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Clone, Debug)]
struct Velocity {
    x: i32,
    y: i32,
}

enum Quadrant {
    First,
    Second,
    Third,
    Fourth,
}

#[derive(Clone, Debug)]
struct Robot {
    position: Position,
    velocity: Velocity,
}

impl Robot {
    fn r#move(&mut self) {
        self.position.x = ((self.position.x + self.velocity.x) + MAP_WIDTH) % MAP_WIDTH;
        self.position.y = ((self.position.y + self.velocity.y) + MAP_HEIGHT) % MAP_HEIGHT;
    }
    fn get_quadrant(&self) -> Option<Quadrant> {
        let half_width = MAP_WIDTH / 2;
        let half_height = MAP_HEIGHT / 2;
        if (0..half_width).contains(&self.position.x) && (0..half_height).contains(&self.position.y)
        {
            Some(Quadrant::First)
        } else if (half_width + 1..MAP_WIDTH).contains(&self.position.x)
            && (0..half_height).contains(&self.position.y)
        {
            Some(Quadrant::Second)
        } else if (half_width + 1..MAP_WIDTH).contains(&self.position.x)
            && (half_height + 1..MAP_HEIGHT).contains(&self.position.y)
        {
            Some(Quadrant::Third)
        } else if (0..half_width).contains(&self.position.x)
            && (half_height + 1..MAP_HEIGHT).contains(&self.position.y)
        {
            Some(Quadrant::Fourth)
        } else {
            None
        }
    }
}

fn main() {
    let path = "inputs/day14";
    let mut robots = parse_input(path);

    println!("part 1: safety factor: {:?}", solve_part1(&mut robots));
    part2(&robots);
}

fn parse_input(path: &str) -> Vec<Robot> {
    let file = File::open(path).expect("couldn't open input file");
    let reader = BufReader::new(file);

    let re = Regex::new(r"-?\d+").unwrap();

    reader
        .lines()
        .map(|line| {
            let line = line.expect("problem reading line in input file");
            let values: Vec<i32> = re
                .find_iter(&line)
                .filter_map(|m| m.as_str().parse().ok())
                .collect();

            return Robot {
                position: Position {
                    x: values[0],
                    y: values[1],
                },
                velocity: Velocity {
                    x: values[2],
                    y: values[3],
                },
            };
        })
        .collect()
}

fn solve_part1(robots: &Vec<Robot>) -> u64 {
    let mut robots = robots.clone();
    for _ in 1..=100 {
        robots.iter_mut().for_each(|robot| robot.r#move());
    }
    let (mut q1, mut q2, mut q3, mut q4) = (0, 0, 0, 0);
    robots.iter().for_each(|robot| {
        if let Some(quadrant) = robot.get_quadrant() {
            match quadrant {
                Quadrant::First => q1 += 1,
                Quadrant::Second => q2 += 1,
                Quadrant::Third => q3 += 1,
                Quadrant::Fourth => q4 += 1,
            }
        }
    });
    q1 * q2 * q3 * q4
}

fn render_robots(robots: &Vec<Robot>, i: u32) {
    let mut img = RgbImage::new(MAP_WIDTH as u32, MAP_HEIGHT as u32);
    for (x, y, pixel) in img.enumerate_pixels_mut() {
        let robot_count = robots
            .iter()
            .filter(|r| r.position.x == x as i32 && r.position.y == y as i32)
            .count();
        if robot_count > 0 {
            *pixel = image::Rgb([255, 255, 255]);
        }
    }
    img.save(format!("outputs/day14/robots/{i}.png")).unwrap();
}

fn part2(robots: &Vec<Robot>) {
    let mut robots = robots.clone();
    render_robots(&robots, 0);

    for i in 1..=10000 {
        robots.iter_mut().for_each(|robot| robot.r#move());
        render_robots(&robots, i);
    }
}

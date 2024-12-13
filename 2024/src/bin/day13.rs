use std::{fs::File, io::Read};

use regex::Regex;

#[derive(Debug)]
struct Equation {
    // ax + by = c
    a: i64,
    b: i64,
    c: i64,
}

impl Equation {
    fn new(a: i64, b: i64, c: i64) -> Self {
        Self { a, b, c }
    }
    fn scale(&self, factor: i64) -> Self {
        Self {
            a: self.a * factor,
            b: self.b * factor,
            c: self.c * factor,
        }
    }
    fn subtract(&self, other: &Self) -> Self {
        Self {
            a: self.a - other.a,
            b: self.b - other.b,
            c: self.c - other.c,
        }
    }
    fn solve_simultaneous(eq1: &Self, eq2: &Self) -> (i64, i64) {
        // solve by eliminating a's
        let lcm = lcm(eq1.a, eq2.a);

        let factor1 = lcm / eq1.a;
        let eq1_scaled = eq1.scale(factor1);

        let factor2 = lcm / eq2.a;
        let eq2_scaled = eq2.scale(factor2);

        let eq3 = eq1_scaled.subtract(&eq2_scaled);
        let y = eq3.c / eq3.b;

        let x = (eq2.c - (eq2.b * y)) / eq2.a;

        (x, y)
    }
}

fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 {
        a.abs()
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a: i64, b: i64) -> i64 {
    (a * b) / gcd(a, b)
}

fn main() {
    let path = "inputs/day13";
    let machines = parse_input(path);

    println!("part 1: fewest tokens: {:?}", solve_part1(&machines));
    println!("part 2: fewest tokens: {:?}", solve_part2(&machines));
}

fn parse_input(path: &str) -> Vec<(Equation, Equation)> {
    let mut file = File::open(path).expect("couldn't open input file");
    let mut buf = String::new();
    file.read_to_string(&mut buf)
        .expect("problem reaading input file");

    let re = Regex::new(
        r#"Button A: X\+(\d+), Y\+(\d+)
Button B: X\+(\d+), Y\+(\d+)
Prize: X=(\d+), Y=(\d+)"#,
    )
    .unwrap();

    re.captures_iter(&buf)
        .map(|caps| {
            let (_, [a1, a2, b1, b2, c1, c2]) = caps.extract::<6>();
            let [a1, b1, c1] = [
                a1.parse().unwrap(),
                b1.parse().unwrap(),
                c1.parse().unwrap(),
            ];
            let eq1 = Equation::new(a1, b1, c1);
            let [a2, b2, c2] = [
                a2.parse().unwrap(),
                b2.parse().unwrap(),
                c2.parse().unwrap(),
            ];
            let eq2 = Equation::new(a2, b2, c2);
            return (eq1, eq2);
        })
        .collect()
}

fn solve_part1(machines: &Vec<(Equation, Equation)>) -> u64 {
    machines
        .iter()
        .map(|m| {
            let (eq1, eq2) = m;
            let (a, b) = Equation::solve_simultaneous(eq1, eq2);
            if a >= 0
                && a <= 100
                && b >= 0
                && b <= 100
                && (a * eq1.a + b * eq1.b == eq1.c)
                && (a * eq2.a + b * eq2.b == eq2.c)
            {
                return ((a * 3) + b) as u64;
            }
            return 0;
        })
        .sum()
}

fn solve_part2(machines: &Vec<(Equation, Equation)>) -> u64 {
    machines
        .iter()
        .map(|m| {
            let (eq1, eq2) = m;
            let eq1 = Equation {
                c: eq1.c + 10000000000000,
                ..*eq1
            };
            let eq2 = Equation {
                c: eq2.c + 10000000000000,
                ..*eq2
            };
            let (a, b) = Equation::solve_simultaneous(&eq1, &eq2);
            if (a >= 0)
                && (b >= 0)
                && (a * eq1.a + b * eq1.b == eq1.c)
                && (a * eq2.a + b * eq2.b == eq2.c)
            {
                return ((a * 3) + b) as u64;
            }
            return 0;
        })
        .sum()
}

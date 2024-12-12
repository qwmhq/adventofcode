use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Plot {
    x: usize,
    y: usize,
}

struct PlotNeighbours {
    up: Option<Plot>,
    down: Option<Plot>,
    right: Option<Plot>,
    left: Option<Plot>,
}

impl Plot {
    fn new(x: usize, y: usize) -> Self {
        Plot { x, y }
    }

    fn is_in_map(&self, map: &Vec<Vec<char>>) -> bool {
        self.x < map[0].len() && self.y < map.len()
    }

    fn get_plant(&self, map: &Vec<Vec<char>>) -> Option<char> {
        if self.is_in_map(map) {
            Some(map[self.y as usize][self.x as usize])
        } else {
            None
        }
    }

    fn is_same_plant(&self, other: &Self, map: &Vec<Vec<char>>) -> bool {
        let plant1 = self.get_plant(map);
        let plant2 = other.get_plant(map);
        plant1.is_some() && plant2.is_some() && plant1.unwrap() == plant2.unwrap()
    }

    fn neighbours(&self, map: &Vec<Vec<char>>) -> Vec<Self> {
        let map_width = map[0].len();
        let map_height = map.len();

        let mut neighbours = Vec::new();
        if self.x + 1 < map_width {
            neighbours.push(Self::new(self.x + 1, self.y));
        }
        if self.x > 0 {
            neighbours.push(Self::new(self.x - 1, self.y));
        }
        if self.y + 1 < map_height {
            neighbours.push(Self::new(self.x, self.y + 1));
        }
        if self.y > 0 {
            neighbours.push(Self::new(self.x, self.y - 1));
        }
        neighbours
            .into_iter()
            .filter(|p| p.is_same_plant(self, map))
            .collect()
    }

    fn neighbours2(&self, map: &Vec<Vec<char>>) -> PlotNeighbours {
        let mut neighbours = PlotNeighbours {
            up: None,
            down: None,
            right: None,
            left: None,
        };

        if self.y > 0 {
            let up = Self::new(self.x, self.y - 1);
            if up.is_same_plant(self, map) {
                neighbours.up = Some(up);
            }
        }
        if self.x > 0 {
            let left = Self::new(self.x - 1, self.y);
            if left.is_same_plant(self, map) {
                neighbours.left = Some(left);
            }
        }
        let down = Self::new(self.x, self.y + 1);
        if down.is_same_plant(self, map) {
            neighbours.down = Some(down);
        }
        let right = Self::new(self.x + 1, self.y);
        if right.is_same_plant(self, map) {
            neighbours.right = Some(right);
        }
        neighbours
    }

    fn get_plots_in_region(&self, map: &Vec<Vec<char>>) -> Vec<Self> {
        let mut visited: HashSet<Self> = HashSet::new();
        self.plots_in_region(map, &mut visited)
    }

    fn plots_in_region(&self, map: &Vec<Vec<char>>, visited: &mut HashSet<Self>) -> Vec<Self> {
        if visited.contains(self) {
            return vec![];
        }
        visited.insert(*self);
        let mut plots = self
            .neighbours(map)
            .into_iter()
            .map(|plot| plot.plots_in_region(map, visited))
            .flatten()
            .collect::<Vec<_>>();
        plots.push(*self);
        plots
    }
}

fn main() {
    let path = "inputs/day12";
    let map = parse_map(path);

    println!("part 1: total fencing price: {:?}", solve_part1(&map));
    println!(
        "part 2: total discounted fencing price: {:?}",
        solve_part2(&map)
    );
}

fn parse_map(path: &str) -> Vec<Vec<char>> {
    let file = File::open(path).expect("couldn't open input file");
    let reader = BufReader::new(file);

    reader
        .lines()
        .map(|line| {
            let line = line.expect("couldn't read line from input");
            line.chars().collect()
        })
        .collect()
}

fn region_perimeter(region: &Vec<Plot>, map: &Vec<Vec<char>>) -> u64 {
    let mut perimeter = 0;
    region
        .into_iter()
        .for_each(|plot| perimeter += (4 - plot.neighbours(map).len()) as u64);
    perimeter
}

fn solve_part1(map: &Vec<Vec<char>>) -> u64 {
    let map_width = map[0].len();
    let map_height = map.len();

    let mut fencing_price = 0;
    let mut visited: HashSet<Plot> = HashSet::new();

    for y in 0..map_height {
        for x in 0..map_width {
            if visited.contains(&Plot::new(x, y)) {
                continue;
            }

            let plot = Plot::new(x, y);
            let region = plot.get_plots_in_region(map);
            let area = region.len() as u64;
            let perimeter = region_perimeter(&region, map);

            fencing_price += area * perimeter;

            region.into_iter().for_each(|plot| {
                visited.insert(plot);
            });
        }
    }
    fencing_price
}

fn region_sides(region: &Vec<Plot>, map: &Vec<Vec<char>>) -> u64 {
    let mut sides = 0;
    region.into_iter().for_each(|plot| {
        let neighbours = plot.neighbours2(map);
        if neighbours.left.is_none() && neighbours.up.is_none() {
            sides += 1;
        }
        if neighbours.left.is_none() && neighbours.down.is_none() {
            sides += 1;
        }
        if neighbours.right.is_none() && neighbours.up.is_none() {
            sides += 1;
        }
        if neighbours.right.is_none() && neighbours.down.is_none() {
            sides += 1;
        }
        if neighbours.left.is_some()
            && neighbours.up.is_some()
            && !Plot::new(plot.x - 1, plot.y - 1).is_same_plant(plot, map)
        {
            sides += 1;
        }
        if neighbours.left.is_some()
            && neighbours.down.is_some()
            && !Plot::new(plot.x - 1, plot.y + 1).is_same_plant(plot, map)
        {
            sides += 1;
        }
        if neighbours.right.is_some()
            && neighbours.up.is_some()
            && !Plot::new(plot.x + 1, plot.y - 1).is_same_plant(plot, map)
        {
            sides += 1;
        }
        if neighbours.right.is_some()
            && neighbours.down.is_some()
            && !Plot::new(plot.x + 1, plot.y + 1).is_same_plant(plot, map)
        {
            sides += 1;
        }
    });
    sides
}

fn solve_part2(map: &Vec<Vec<char>>) -> u64 {
    let map_width = map[0].len();
    let map_height = map.len();

    let mut fencing_price = 0;
    let mut visited: HashSet<Plot> = HashSet::new();

    for y in 0..map_height {
        for x in 0..map_width {
            if visited.contains(&Plot::new(x, y)) {
                continue;
            }

            let plot = Plot::new(x, y);
            let region = plot.get_plots_in_region(map);
            let area = region.len() as u64;
            let sides = region_sides(&region, map);

            fencing_price += area * sides;

            region.into_iter().for_each(|plot| {
                visited.insert(plot);
            });
        }
    }
    fencing_price
}

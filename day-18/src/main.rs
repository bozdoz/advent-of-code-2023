use core::panic;
use std::{time::Instant, fs, vec};
use lib::{get_part, Point};

struct Grid {
    nodes: Vec<Point>,
    boundary_count: usize,
}

impl Grid {
    fn new(contents: &str, part: u8) -> Self {
        let mut nodes = vec![];

        let lines = contents.lines();
        let mut cur = Point{x: 0, y: 0};
        let mut boundary_count: usize = 0;

        let get_instructions = if part == 1 {
            |parts: Vec<&str>| {
                let dir = parts[0].to_owned();
                let steps = parts[1].parse::<isize>().expect("steps are a number");
                
                (dir, steps)
            }
        } else {
            |parts: Vec<&str>| {
                let color = parts[2];
                let dir = usize::from_str_radix(&color[7..8], 16).expect("direction to be a number");
                let dir = match dir {
                    0 => "R",
                    1 => "D",
                    2 => "L",
                    3 => "U",
                    _ => panic!("can't find dir!")
                }.to_owned();
                let steps = usize::from_str_radix(&color[2..7], 16).expect("steps to be a number") as isize;

                (dir, steps)
            }
        };

        for line in lines {
            let parts: Vec<&str> = line.split_whitespace().collect();

            let (dir, steps) = get_instructions(parts);

            let inc = match dir.as_str() {
                "U" => Point{ x: 0, y: -steps },
                "R" => Point{ x: steps, y: 0 },
                "D" => Point{ x: 0, y: steps },
                "L" => Point{ x: -steps, y: 0 },
                _ => panic!("no direction?")
            };

            boundary_count += steps as usize;
            cur = cur + inc;
            nodes.push(cur);
        }

        Self { nodes, boundary_count }
    }
    // shoelace formula
    fn area(&self) -> isize {
        let mut points = self.nodes.clone();
        
        // make sure it's closed
        points.push(points[0]);

        // 2A = (x1 * y2 - y1 * x2) + (x2 * y3...)...

        let sum: isize = points
            .windows(2)
            .map(|p| { p[0].x * p[1].y - p[0].y * p[1].x })
            .sum();

        let boundary_count = self.boundary_count as isize;

        // I don't know how to read math hieroglyphs
        // effed up somewhere? seems too complicated
        sum.abs() / 2 - boundary_count / 2 + 1 + boundary_count
    }
}

fn part_one(grid: &Grid) -> isize {
    grid.area()
}

fn part_two(grid: &Grid) -> isize {
    grid.area()
}

fn main() {
    let (one, two) = get_part();
    let start = Instant::now();
    let contents = fs::read_to_string("./src/input.txt").unwrap();

    
    if one {
        let now = Instant::now();
        let grid = Grid::new(&contents, 1);
        let ans = part_one(&grid);
        println!("Part one: {:?} {:?}", ans, now.elapsed());
    }

    if two {
        let now = Instant::now();
        let grid = Grid::new(&contents, 2);
        let ans = part_two(&grid);
        println!("Part two: {:?} {:?}", ans, now.elapsed());
    }

    println!("Time: {:?}", start.elapsed())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("./example.txt");

    #[test]
    fn test_part_one() {
        let grid = Grid::new(EXAMPLE, 1);
        let ans = part_one(&grid);

        assert_eq!(ans, 62);
    }

    #[test]
    fn test_part_two() {
        let grid = Grid::new(EXAMPLE, 2);
        let ans = part_two(&grid);

        assert_eq!(ans, 952408144115);
    }
}

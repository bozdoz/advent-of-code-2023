use std::{ time::Instant, fs, collections::HashSet };
use lib::{ get_part, Point };

struct Universe {
    grid: HashSet<Point>,
}

impl Universe {
    fn new(contents: &str) -> Self {
        let lines = contents.lines();
        let mut grid = HashSet::new();
        let mut height = 0;
        let mut width = 0;
        let mut x_set = HashSet::new();
        let mut y_set = HashSet::new();

        for (y, line) in lines.enumerate() {
            for (x, char) in line.chars().enumerate() {
                if char == '#' {
                    dbg!(x, y);
                    grid.insert(Point { x: x as isize, y: y as isize });
                    x_set.insert(x);
                    y_set.insert(y);
                }
                if y == 0 && x > width {
                    width = x;
                }
            }
            if y > height {
                height = y;
            }
        }

        // find empty col/rows and expand
        for y in 0..=height {
            if !y_set.contains(&y) {
                // empty row!
                println!("empty row! {}", y);
            }
        }

        for x in 0..=width {
            if !x_set.contains(&x) {
                // empty col! 
                println!("empty col: {}", x);
            }
        }

        Self { grid }
    }
}

fn part_one() -> usize {
    0
}

fn part_two() -> usize {
    0
}

fn main() {
    let (one, two) = get_part();
    let start = Instant::now();
    let contents = fs::read_to_string("./src/input.txt").unwrap();

    if one {
        let now = Instant::now();
        let ans = part_one();
        println!("Part one: {:?} {:?}", ans, now.elapsed());
    }

    if two {
        let now = Instant::now();
        let ans = part_two();
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
        let grid = Universe::new(EXAMPLE);
        let ans = part_one();

        assert_eq!(ans, 10);
    }

    #[test]
    fn test_part_two() {
        let ans = part_two();

        assert_eq!(ans, 0);
    }
}

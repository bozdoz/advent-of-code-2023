use std::{time::Instant, fs};
use lib::get_part;

/** identify the direction of the reflection, with the col/row */
enum Reflection {
    Horizontal(usize),
    Vertical(usize),
    None
}

#[derive(Debug, PartialEq)]
enum Item {
    Ash,
    Rock
}

struct Pattern {
    // uh oh, not going with hashmap
    grid: Vec<Vec<Item>>
}

impl Pattern {
    fn new(contents: &str) -> Self {
        let lines = contents.lines();
        let mut grid = vec![];

        // maybe don't need enumerate 
        for line in lines {
            let mut row = vec![];
            for char in line.chars() {
                // avoided using a `match` here, since the third option is panic
                row.push(if char == '.' {
                    Item::Ash
                } else {
                    Item::Rock
                });
            }
            grid.push(row);
        }

        Self { grid }
    }

    fn find_reflection_point(&self) -> Reflection {
        // size 2 means we look for side-by-side matches first
        dbg!(&self.grid);
        for row in self.grid.windows(2) {
            if row[0] == row[1] {
                dbg!(row);
            }            
        }

        // we might not have any reflection
        Reflection::None
    }
}

fn part_one(patterns: &Vec<Pattern>) -> usize {
    patterns[0].find_reflection_point();
    1
}

fn part_two() -> usize {
    0
}

fn main() {
    let (one, two) = get_part();
    let start = Instant::now();
    let contents = fs::read_to_string("./src/input.txt").unwrap();

    let patterns = contents.lines().map(Pattern::new).collect();

    if one {
        let now = Instant::now();
        let ans = part_one(&patterns);
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
        let patterns = EXAMPLE.split("\n\n").map(Pattern::new).collect();
        let ans = part_one(&patterns);

        assert_eq!(ans, 0);
    }

    #[test]
    fn test_part_two() {
        let ans = part_two();

        assert_eq!(ans, 0);
    }
}

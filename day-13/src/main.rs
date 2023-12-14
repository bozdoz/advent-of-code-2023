use std::{time::Instant, fs};
use lib::get_part;

/** identify the direction of the reflection, with the col/row */
#[derive(PartialEq)]
enum Reflection {
    Horizontal(usize),
    Vertical(usize),
    None
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Item {
    Ash,
    Rock
}

// TODO: this should be a binary, and we can check for 
// XOR, then count ones to see if there's any diff
struct Pattern {
    // uh oh, not going with hashmap
    grid: Vec<Vec<Item>>,
    // lazy way to check column equality, just flip it sideways
    transposed: Vec<Vec<Item>>
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

        
        let rows = grid.len();
        let cols = grid[0].len();

        let mut transposed: Vec<Vec<Item>> = vec![vec![Item::Ash; rows]; cols];

        for r in 0..rows  {
            for c in 0..cols {
                // Transpose the elements
                transposed[c][r] = grid[r][c];
            }
        }

        Self { grid, transposed }
    }

    // pass grid or transposed to check rows or cols
    fn is_reflection_at(grid: &Vec<Vec<Item>>, i: usize) -> bool {
        // expand outwards from index
        let prev = grid.iter().rev().skip(grid.len() - i);
        let next = grid.iter().skip(i + 2);

        for (p, n) in prev.zip(next) {
            if p != n {
                return false
            }
        }

        true
    }

    fn find_reflection_point(&self) -> Reflection {
        // size 2 means we look for side-by-side matches first
        for (i, row) in self.grid.windows(2).enumerate() {
            if row[0] == row[1] {
                // we have one pair; extend the search to pair everything
                if Self::is_reflection_at(&self.grid, i) {
                    return Reflection::Horizontal(i + 1);
                }
            }
        }

        // vertical is harder, because we have to make new vectors?
        // let's be lazy and use the transposed
        for (i, col) in self.transposed.windows(2).enumerate() {
            if col[0] == col[1] {
                // we have one pair; extend the search to pair everything
                if Self::is_reflection_at(&self.transposed, i) {
                    return Reflection::Vertical(i + 1);
                }
            }
        }

        // we might not have any reflection
        Reflection::None
    }
}

fn part_one(patterns: &Vec<Pattern>) -> usize {
    patterns.iter().map(|p| {
        match p.find_reflection_point() {
            Reflection::Horizontal(n) => n * 100,
            Reflection::Vertical(n) => n,
            Reflection::None => 0 
        }
    }).sum()
}

fn part_two() -> usize {
    0
}

fn main() {
    let (one, two) = get_part();
    let start = Instant::now();
    let contents = fs::read_to_string("./src/input.txt").unwrap();

    let patterns = contents.split("\n\n").map(Pattern::new).collect();

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
    fn test_reflection_point() {
        let patterns: Vec<Pattern> = EXAMPLE.split("\n\n").map(Pattern::new).collect();

        if let Reflection::Horizontal(n) = patterns[1].find_reflection_point() {
            assert_eq!(n, 4, "n isn't 4!");
        } else {
            assert!(false, "HEY! n is not horizontal");
        };
        
        if let Reflection::Vertical(n) = patterns[0].find_reflection_point() {
            assert_eq!(n, 5, "n isn't 5!");
        } else {
            assert!(false, "HEY! n is not vertical");
        };
    }

    #[test]
    fn test_part_one() {
        let patterns = EXAMPLE.split("\n\n").map(Pattern::new).collect();
        let ans = part_one(&patterns);

        assert_eq!(ans, 405);
    }

    #[test]
    fn test_part_two() {
        let ans = part_two();

        assert_eq!(ans, 0);
    }
}

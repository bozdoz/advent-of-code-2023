use std::{ time::Instant, fs };
use lib::get_part;

type Point = (usize, usize);

struct Universe {
    grid: Vec<Point>,
    empty_cols: Vec<usize>,
    empty_rows: Vec<usize>,
}

impl Universe {
    fn new(contents: &str) -> Self {
        let lines = contents.lines();
        let mut grid = Vec::new();
        let mut empty_cols = Vec::new();
        let mut empty_rows = Vec::new();
        let mut height = 0;
        let mut width = 0;

        for (y, line) in lines.enumerate() {
            height += 1;
            width = line.len();
            for (x, char) in line.chars().enumerate() {
                if char == '#' {
                    grid.push((x, y));
                }
            }
        }

        for x in 0..width {
            if grid.iter().any(|(px, _)| *px == x) {
                continue;
            }
            empty_cols.push(x);
        }

        for y in 0..height {
            if grid.iter().any(|(_, py)| *py == y) {
                continue;
            }
            empty_rows.push(y);
        }

        Self { grid, empty_cols, empty_rows }
    }

    fn get_manhattan_distance(&self, gap_distance: usize) -> usize {
        (0..self.grid.len() - 1)
            .flat_map(|i| {
                (i + 1..self.grid.len()).map(move |j| {
                    let dx = self.grid[i].0.abs_diff(self.grid[j].0);
                    let dy = self.grid[i].1.abs_diff(self.grid[j].1);
                    let min_x = self.grid[i].0.min(self.grid[j].0);
                    let min_y = self.grid[i].1.min(self.grid[j].1);
                    let max_x = min_x + dx;
                    let max_y = min_y + dy;

                    let empty_x = self.empty_cols
                        .iter()
                        .filter(|x| **x >= min_x && **x <= max_x)
                        .count();
                    let empty_y = self.empty_rows
                        .iter()
                        .filter(|y| **y >= min_y && **y <= max_y)
                        .count();

                    let gap = gap_distance - 1;
                    let dist = dx + empty_x * gap + dy + empty_y * gap;

                    return dist;
                })
            })
            .sum()
    }
}

fn part_one(uni: &Universe) -> usize {
    uni.get_manhattan_distance(2)
}

fn part_two(uni: &Universe) -> usize {
    uni.get_manhattan_distance(1000000)
}

fn main() {
    let (one, two) = get_part();
    let start = Instant::now();
    let contents = fs::read_to_string("./src/input.txt").unwrap();
    let universe = Universe::new(&contents);

    if one {
        let now = Instant::now();
        let ans = part_one(&universe);
        println!("Part one: {:?} {:?}", ans, now.elapsed());
    }

    if two {
        let now = Instant::now();
        let ans = part_two(&universe);
        println!("Part two: {:?} {:?}", ans, now.elapsed());
    }

    println!("Time: {:?}", start.elapsed())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("./example.txt");

    const EXAMPLE_2: &str = "#..
...
..#";

    #[test]
    fn test_simple() {
        let grid = Universe::new(EXAMPLE_2);

        assert_eq!(grid.empty_cols, vec![1]);
        assert_eq!(grid.empty_rows, vec![1]);

        let ans = grid.get_manhattan_distance(2);

        assert_eq!(ans, 6);
    }

    #[test]
    fn test_part_one() {
        let grid = Universe::new(EXAMPLE);

        assert_eq!(grid.empty_cols, vec![2, 5, 8]);
        assert_eq!(grid.empty_rows, vec![3, 7]);

        let ans = grid.get_manhattan_distance(2);

        assert_eq!(ans, 374);
    }

    #[test]
    fn test_part_two() {
        let grid = Universe::new(EXAMPLE);

        assert_eq!(grid.get_manhattan_distance(10), 1030);
        assert_eq!(grid.get_manhattan_distance(100), 8410);
    }
}

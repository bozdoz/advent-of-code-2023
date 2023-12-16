use std::{ time::Instant, fs };
use lib::{ get_part };

#[derive(Debug, Clone, Copy, PartialEq)]
enum Rock {
    Rounded,
    Cube,
    None,
}

#[derive(Debug, Clone)]
struct Platform {
    grid: Vec<Vec<Rock>>,
}

impl Platform {
    fn new(contents: &str) -> Self {
        let mut grid = vec![];

        let lines = contents.lines();

        for line in lines {
            let row = line
                .chars()
                .map(|char| {
                    match char {
                        'O' => Rock::Rounded,
                        '#' => Rock::Cube,
                        _ => Rock::None,
                    }
                })
                .collect();

            grid.push(row);
        }

        Self { grid }
    }

    fn shifted(&self) -> Self {
        // I don't think I want to mutate the original grid
        let mut clone = self.clone();

        // need to iterate len instead of iter
        let height = clone.grid.len();
        let width = clone.grid[0].len();

        for y in 1..height {
            for x in 0..width {
                let rock = &clone.grid[y][x];
                if *rock == Rock::Rounded {
                    // try to go up
                    for mut u in (0..y).rev() {
                        if clone.grid[u][x] == Rock::None {
                            if u != 0 {
                                continue;
                            } 
                        } else {
                            // update u
                            u += 1;
                            
                            if y == u {
                                // ignore self
                                break;
                            }
                        }
                        
                        // move to prev position
                        clone.grid[u][x] = Rock::Rounded;
                        // erase current
                        clone.grid[y][x] = Rock::None;

                        break;
                    }
                }
            }
        }

        clone
    }

    fn total_load(grid: Vec<Vec<Rock>>) -> usize {
        let height = grid.len();
        let width = grid[0].len();

        let mut sum = 0;

        for y in 0..height {
            for x in 0..width {
                if grid[y][x] == Rock::Rounded {
                    sum += height - y;
                }
            }
        }

        sum
    }
}

fn part_one(platform: &Platform) -> usize {
    let shifted = platform.shifted();

    Platform::total_load(shifted.grid)
}

fn part_two() -> usize {
    0
}

fn main() {
    let (one, two) = get_part();
    let start = Instant::now();
    let contents = fs::read_to_string("./src/input.txt").unwrap();

    let platform = Platform::new(&contents);

    if one {
        let now = Instant::now();
        let ans = part_one(&platform);
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
        let platform = Platform::new(EXAMPLE);
        let ans = part_one(&platform);

        assert_eq!(ans, 136);
    }

    #[test]
    fn test_part_two() {
        let ans = part_two();

        assert_eq!(ans, 0);
    }
}

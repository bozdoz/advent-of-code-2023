use std::{
    collections::{ BinaryHeap, HashSet },
    fs,
    time::Instant,
    usize,
    vec,
};
use lib::get_part;

struct Grid {
    cells: Vec<Vec<u32>>,
    width: usize,
    height: usize,
}

impl Grid {
    fn new(data: &str) -> Self {
        let mut cells = vec![];

        for (_, row) in data.lines().enumerate() {
            let mut new_row = vec![];
            for (_, cell) in row.chars().enumerate() {
                let num = cell.to_digit(10).expect("numbered cell");

                new_row.push(num);
            }

            cells.push(new_row);
        }

        let width = cells[0].len();
        let height = cells.len();

        Self { cells, width, height }
    }

    fn get(&self, cell: (isize, isize)) -> Option<&u32> {
        if cell.0 < 0 || cell.1 < 0 {
            return None;
        }

        self.cells
            .get(cell.0 as usize)
            .map(|x| { x.get(cell.1 as usize) })
            .flatten()
    }
}

const DIRS: [(i8, i8); 4] = [
    (-1, 0), // top
    (0, 1), // right
    (1, 0), // bottom
    (0, -1), // left
];

type Dir = (i8, i8);

#[derive(PartialEq, Eq, Ord)]
struct State {
    cost: usize,
    pos: (usize, usize),
    dir: Dir,
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        // make it min heap
        Some(other.cost.cmp(&self.cost))
    }
}

fn search(grid: &Grid, lower: usize, upper: usize) -> usize {
    // min heap with cost and previous direction
    let mut heap: BinaryHeap<State> = BinaryHeap::new();
    let mut visited = HashSet::new();
    let end = (grid.height - 1, grid.width - 1);

    // start at top left; end at bottom right
    heap.push(State {
        cost: 0,
        pos: (0, 0),
        dir: (1, 0),
    });

    while let Some(State { cost, pos, dir }) = heap.pop() {
        // visited?
        if visited.contains(&(pos, dir)) {
            continue;
        }

        // done?
        if pos == end {
            return cost;
        }

        // get next
        let opposite = (dir.0 * -1, dir.1 * -1);

        for d in DIRS {
            if d == opposite {
                // we just came from here; can't reverse direction
                continue;
            }

            // we just advanced this direction; can't do it again
            if cost != 0 && d == dir {
                continue;
            }

            let mut cur_pos = pos;
            let mut cur_cost = cost;

            // all three steps could be viable
            for i in 1..=upper {
                let next = (
                    (cur_pos.0 as isize) + (d.0 as isize),
                    (cur_pos.1 as isize) + (d.1 as isize),
                );

                // if in grid...
                if let Some(&next_val) = grid.get(next) {
                    cur_pos = (next.0 as usize, next.1 as usize);
                    cur_cost += next_val as usize;

                    if i >= lower {
                        heap.push(State {
                            cost: cur_cost,
                            pos: cur_pos,
                            dir: d,
                        });
                    }
                } else {
                    break;
                }
            }
        }

        visited.insert((pos, dir));
    }

    0
}

fn part_one(grid: &Grid) -> usize {
    search(grid, 1, 3)
}

fn part_two(grid: &Grid) -> usize {
    search(grid, 4, 10)
}

fn main() {
    let (one, two) = get_part();
    let start = Instant::now();
    let contents = fs::read_to_string("./src/input.txt").unwrap();
    let grid = Grid::new(&contents);

    if one {
        let now = Instant::now();
        let ans = part_one(&grid);
        println!("Part one: {:?} {:?}", ans, now.elapsed());
    }

    if two {
        let now = Instant::now();
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
        let ans = part_one(&Grid::new(EXAMPLE));

        assert_eq!(ans, 102);
    }

    #[test]
    fn test_part_two() {
        let ans = part_two(&Grid::new(EXAMPLE));

        assert_eq!(ans, 94);
    }
}

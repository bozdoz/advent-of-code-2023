use std::{ time::Instant, fs, collections::HashMap, vec };
use lib::{ get_part, Point };

struct CellValue {
    char: char,
    // couldn't do [Cell, 2] :(
    neighbours: Vec<Point>,
}

struct Grid {
    cells: HashMap<Point, CellValue>,
    starting_cell: Point,
}

// hey! another lifetime!
struct GridLoop<'a> {
    grid: &'a Grid,
    prev: Option<Point>,
    cur: Option<Point>,
}

// double lifetime
impl<'a> GridLoop<'a> {
    fn new(grid: &'a Grid) -> Self {
        GridLoop { grid, prev: None, cur: None }
    }
}

impl Iterator for GridLoop<'_> {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        let start = self.grid.starting_cell;

        if self.prev.is_none() {
            // start with cell next to starting cell
            self.prev = Some(start);
            self.cur = Some(self.grid.get_cell(&start).neighbours[0]);

            return self.cur;
        }

        let cur = self.cur.unwrap();

        // finish if we reach the start
        if cur == start {
            return None;
        }

        // get next
        let prev = self.prev.unwrap();
        let value = self.grid.get_cell(&cur);
        let next = value.neighbours
            .iter()
            .find(|x| {
                // double de-reference again...
                **x != prev
            })
            .unwrap();

        self.prev = Some(cur);
        self.cur = Some(*next);

        self.cur
    }
}

impl Grid {
    fn new(contents: &str) -> Self {
        let lines = contents.lines();
        let mut cells = HashMap::new();
        let mut starting_cell = Point { x: 0, y: 0 };

        for (y, line) in lines.enumerate() {
            for (x, char) in line.chars().enumerate() {
                cells.insert(
                    Point { x: x as isize, y: y as isize },
                    CellValue { char, neighbours: vec![] }
                );
            }
        }

        // get neighbours
        // need &mut to set neighbours
        for (cell, value) in &mut cells {
            let neighbours: Vec<Point> = match value.char {
                '|' => {
                    vec![
                        *cell + Point { x: 0, y: -1 },
                        *cell + Point { x: 0, y: 1 }
                    ]
                }
                '-' => {
                    vec![
                        *cell + Point { x: -1, y: 0 },
                        *cell + Point { x: 1, y: 0 }
                    ]
                }
                'L' => {
                    vec![
                        *cell + Point { x: 0, y: -1 },
                        *cell + Point { x: 1, y: 0 }
                    ]
                }
                'J' => {
                    vec![
                        *cell + Point { x: 0, y: -1 },
                        *cell + Point { x: -1, y: 0 }
                    ]
                }
                '7' => {
                    vec![
                        *cell + Point { x: 0, y: 1 },
                        *cell + Point { x: -1, y: 0 }
                    ]
                }
                'F' => {
                    vec![
                        *cell + Point { x: 0, y: 1 },
                        *cell + Point { x: 1, y: 0 }
                    ]
                }
                // ignore '.'
                '.' => {
                    continue;
                }
                'S' => {
                    // using Copy, Clone traits
                    starting_cell = *cell;

                    vec![]
                }
                c => { panic!("what did you do?, {}", c) }
            };

            value.neighbours = neighbours;
        }

        // find starting point neighbours
        let top = starting_cell + Point { x: 0, y: -1 };
        let bottom = starting_cell + Point { x: 0, y: 1 };
        let left = starting_cell + Point { x: -1, y: 0 };
        let right = starting_cell + Point { x: 1, y: 0 };

        let starting_point_neighbours: Vec<Point> = [top, bottom, left, right]
            .iter()
            .filter_map(|neigh| {
                if let Some(v2) = cells.get(&neigh) {
                    if v2.neighbours.iter().any(|x| *x == starting_cell) {
                        return Some(*neigh);
                    }
                }
                None
            })
            .collect();

        if let Some(v) = cells.get_mut(&starting_cell) {
            v.neighbours = starting_point_neighbours;
        }

        // first time returning Self?
        Self { cells, starting_cell }
    }

    fn get_cell(&self, p: &Point) -> &CellValue {
        self.cells.get(p).unwrap()
    }

    fn walk_iter(&self) -> usize {
        GridLoop::new(self).into_iter().count()
    }

    // shoelace formula
    fn area(&self) -> isize {
        let mut points: Vec<Point> = GridLoop::new(self).collect();

        // make sure it's closed
        points.push(points[0]);

        // 2A = (x1 * y2 - y1 * x2) + (x2 * y3...)...

        let sum: isize = points
            .windows(2)
            .map(|p| { p[0].x * p[1].y - p[0].y * p[1].x })
            .sum();

        // I don't know how to read math hieroglyphs
        sum.abs() / 2
    }
}

fn part_one(grid: &Grid) -> usize {
    grid.walk_iter() / 2
}

fn part_two(grid: &Grid) -> isize {
    let area = grid.area();
    let boundary_count = grid.walk_iter() as isize;

    // Pick's theorem
    // a = area
    // i = interior
    // b = boundary
    // a = i + b/2 - 1
    // i = a - b/2 + 1
    return area - boundary_count / 2 + 1;
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
        let grid = Grid::new(EXAMPLE);
        let ans = part_one(&grid);

        assert_eq!(ans, 8);
    }

    const EXAMPLE_2: &str =
        "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L
";

    #[test]
    fn test_part_two() {
        let grid = Grid::new(EXAMPLE_2);
        let ans = part_two(&grid);

        assert_eq!(ans, 10);
    }
}

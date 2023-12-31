use std::{ time::Instant, fs, collections::{ HashMap, HashSet } };
use lib::{ get_part, Point };

enum Cell {
    Empty,
    Slope(char),
}

// TODO: get all cells neighbours up-front with cost associated with traveling
// shrink nodes that have only two neighbours
struct Grid {
    cells: HashMap<Point, Cell>,
    start: Point,
    end: Point,
}

#[derive(Clone)]
struct GridState {
    path: HashSet<Point>,
    current: Point,
}

impl Grid {
    fn new(contents: &str) -> Self {
        let mut cells = HashMap::new();
        let mut start = Point { x: 0, y: 0 };
        let mut end = Point { x: 0, y: 0 };

        let lines: Vec<&str> = contents.lines().collect();
        let height = lines.len() - 1;

        for (y, line) in lines.iter().enumerate() {
            for (x, char) in line.chars().enumerate() {
                match char {
                    '.' => {
                        let k = Point { x: x as isize, y: y as isize };
                        cells.insert(k, Cell::Empty);

                        if y == 0 {
                            start = k;
                        } else if y == height {
                            end = k;
                        }

                    }
                    '^' | '>' | '<' | 'v' => {
                        let k = Point { x: x as isize, y: y as isize };
                        cells.insert(k, Cell::Slope(char));
                    }
                    _ => (),
                }
            }
        }

        Self { cells, start, end }
    }

    fn paths(&self, part: u8) -> usize {
        let mut queue: Vec<GridState> = vec![];
        let mut finished = 0;

        // start
        queue.push(GridState { path: HashSet::new(), current: self.start });

        // TIL
        let get_next = if part == 1 {
            Grid::get_next_states
        } else {
            Grid::get_next_states_two
        };

        while let Some(state) = queue.pop() {
            // check if end
            if state.current == self.end {
                let len = state.path.len();
                
                if len > finished {
                    finished = len;                    
                    println!("{}", len);
                }

                continue;
            }

            // get next states
            let states = get_next(self, state);

            // push states to queue
            // TIL: need to clone because HashSet's can't Copy?
            states.iter().for_each(|s| queue.push(s.clone()));
        }

        finished
    }

    fn get_next_states(&self, state: GridState) -> Vec<GridState> {
        // get adjacent paths that aren't in current path
        let cur = state.current;
        let cell = self.cells.get(&cur).expect("cell should be here");

        const NORTH: Point = Point { x: 0, y: -1 };
        const EAST: Point = Point { x: 1, y: 0 };
        const SOUTH: Point = Point { x: 0, y: 1 };
        const WEST: Point = Point { x: -1, y: 0 };

        // if currently on a slope, you need to move to a direction
        if let Cell::Slope(dir) = cell {
            let current = match dir {
                '^' => { cur + NORTH }
                '>' => { cur + EAST }
                'v' => { cur + SOUTH }
                '<' => { cur + WEST }
                n => { panic!("dir isn't an arrow: {}", n) }
            };
            if state.path.contains(&current) {
                return vec![];
            }

            let mut path = state.path.clone();

            // Is there a better way to do this?
            path.insert(current);

            return vec![GridState {
                current,
                path,
            }];
        }

        let mut next = vec![];
        let neighbours = [NORTH, SOUTH, EAST, WEST];

        for n in neighbours {
            let current = cur + n;

            if state.path.contains(&current) {
                continue;
            }

            if let Some(c) = self.cells.get(&current) {
                match c {
                    Cell::Empty => {
                        let mut path = state.path.clone();

                        // Is there a better way to do this?
                        path.insert(current);

                        next.push(GridState {
                            current,
                            path,
                        });
                    }
                    Cell::Slope(d) => {
                        // check if we can move into the slope
                        match (d, n) {
                            ('^', SOUTH) => { continue }
                            ('>', WEST) => { continue }
                            ('<', EAST) => { continue }
                            ('v', NORTH) => { continue }
                            _ => {
                                let mut path = state.path.clone();

                                // Is there a better way to do this?
                                path.insert(current);

                                next.push(GridState {
                                    current,
                                    path,
                                });
                            }
                        }
                    }
                }
            }
        }

        next
    }

    fn get_next_states_two(&self, state: GridState) -> Vec<GridState> {
        // get adjacent paths that aren't in current path
        let cur = state.current;
        
        const NORTH: Point = Point { x: 0, y: -1 };
        const EAST: Point = Point { x: 1, y: 0 };
        const SOUTH: Point = Point { x: 0, y: 1 };
        const WEST: Point = Point { x: -1, y: 0 };

        let mut next = vec![];
        let neighbours = [NORTH, SOUTH, EAST, WEST];

        for n in neighbours {
            let current = cur + n;

            if state.path.contains(&current) {
                continue;
            }

            if let Some(_) = self.cells.get(&current) {
                let mut path = state.path.clone();

                // Is there a better way to do this?
                path.insert(current);

                next.push(GridState {
                    current,
                    path,
                });
            }
        }

        next
    }
}

fn part_one(grid: &Grid) -> usize {
    grid.paths(1)
}

fn part_two(grid: &Grid) -> usize {
    grid.paths(2)
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

        assert_eq!(ans, 94);
    }

    #[test]
    fn test_part_two() {
        let grid = Grid::new(EXAMPLE);
        let ans = part_two(&grid);

        assert_eq!(ans, 154);
    }
}

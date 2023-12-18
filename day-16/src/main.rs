use std::{ time::Instant, fs, collections::{ HashMap, BinaryHeap }, vec };
use lib::{ get_part, Point };

const UP: u8 = 0b1000;
const RIGHT: u8 = 0b0100;
const DOWN: u8 = 0b0010;
const LEFT: u8 = 0b0001;

#[derive(Debug, Clone)]
enum Artifact {
    // .
    Empty,
    // /
    MirrorAsc,
    // \
    MirrorDesc,
    // |
    SplitterV,
    // -
    SplitterH,
}

#[derive(Debug, Clone)]
struct Tile {
    /** binary for up, right, down, left: e.g. 0101 */
    beams: u8,
    artifact: Artifact,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
struct BeamState {
    point: Point,
    direction: u8,
}

#[derive(Clone)]
struct Grid {
    cells: HashMap<Point, Tile>
}

impl Grid {
    fn new(contents: &str) -> Self {
        let mut cells = HashMap::new();
        let lines: Vec<&str> = contents.lines().collect();

        for (y, line) in lines.iter().enumerate() {
            for (x, char) in line.chars().enumerate() {
                let artifact: Artifact = match char {
                    '.' => Artifact::Empty,
                    '/' => Artifact::MirrorAsc,
                    '\\' => Artifact::MirrorDesc,
                    '|' => Artifact::SplitterV,
                    '-' => Artifact::SplitterH,
                    _ => panic!("can't find char: {}", char),
                };

                let point = Point { x: x as isize, y: y as isize };

                let tile = Tile {
                    artifact,
                    beams: 0b0,
                };

                cells.insert(point, tile);
            }
        }

        Self { cells }
    }
    // updates all tiles with the beams
    fn traverse(&self, start: BeamState) -> Self {
        // clone! 
        let mut clone = self.clone();
        // TODO: try using a plain vec instead to benchmark
        // get queue
        let mut queue: BinaryHeap<BeamState> = BinaryHeap::new();

        queue.push(start);

        // while queue...
        while let Some(state) = queue.pop() {
            if let Some(tile) = clone.cells.get_mut(&state.point) {
                // if tile already has beam direction, remove this path
                if tile.beams & state.direction != 0 {
                    continue
                }
                
                // add beam to grid
                tile.beams |= state.direction;
            } else {
                // off of grid if None
                continue;
            }

            // get next states
            let states = clone.get_next_states(state);

            // push states to queue
            states.iter().for_each(|s| queue.push(*s));
        }

        clone
    }
    fn get_next_states(&self, state: BeamState) -> Vec<BeamState> {
        let tile = self.cells.get(&state.point);

        if tile.is_none() {
            return vec![];
        }
        let tile = tile.unwrap();

        // is this too much?
        match tile.artifact {
            Artifact::Empty => {
                let next_point: Point = match state.direction {
                    UP => { state.point + Point { x: 0, y: -1 } }
                    RIGHT => { state.point + Point { x: 1, y: 0 } }
                    DOWN => { state.point + Point { x: 0, y: 1 } }
                    LEFT => { state.point + Point { x: -1, y: 0 } }
                    _ => panic!("direction not in NEWS"),
                };
                return vec![BeamState {
                    direction: state.direction,
                    point: next_point,
                }];
            }
            Artifact::MirrorAsc => {
                match state.direction {
                    UP => {
                        return vec![BeamState {
                            direction: RIGHT,
                            point: state.point + Point { x: 1, y: 0 },
                        }];
                    }
                    RIGHT => { 
                        return vec![BeamState {
                            direction: UP,
                            point: state.point + Point { x: 0, y: -1 },
                        }];
                    }
                    DOWN => { 
                        return vec![BeamState {
                            direction: LEFT,
                            point: state.point + Point { x: -1, y: 0 },
                        }];
                     }
                    LEFT => { 
                        return vec![BeamState {
                            direction: DOWN,
                            point: state.point + Point { x: 0, y: 1 },
                        }];
                     }
                    _ => panic!("direction not in NEWS"),
                }
            }
            Artifact::MirrorDesc => {
                match state.direction {
                    UP => {
                        return vec![BeamState {
                            direction: LEFT,
                            point: state.point + Point { x: -1, y: 0 },
                        }];
                    }
                    RIGHT => { 
                        return vec![BeamState {
                            direction: DOWN,
                            point: state.point + Point { x: 0, y: 1 },
                        }];
                    }
                    DOWN => { 
                        return vec![BeamState {
                            direction: RIGHT,
                            point: state.point + Point { x: 1, y: 0 },
                        }];
                     }
                    LEFT => { 
                        return vec![BeamState {
                            direction: UP,
                            point: state.point + Point { x: 0, y: -1 },
                        }];
                     }
                    _ => panic!("direction not in NEWS"),
                }
            }
            Artifact::SplitterV => {
                match state.direction {
                    UP | DOWN => {
                        let next_point = if state.direction == UP {
                            state.point + Point { x: 0, y: -1 }
                        } else {
                            state.point + Point { x: 0, y: 1 }
                        };

                        return vec![BeamState {
                            direction: state.direction,
                            point: next_point,
                        }];
                    }
                    RIGHT | LEFT => {
                        return vec![
                            BeamState {
                                direction: UP,
                                point: state.point + Point { x: 0, y: -1 },
                            },
                            BeamState {
                                direction: DOWN,
                                point: state.point + Point { x: 0, y: 1 },
                            }
                        ];
                    }
                    _ => panic!("direction not in NEWS"),
                };
            }
            Artifact::SplitterH => {
                match state.direction {
                    RIGHT | LEFT => {
                        let next_point = if state.direction == RIGHT {
                            state.point + Point { x: 1, y: 0 }
                        } else {
                            state.point + Point { x: -1, y: 0 }
                        };
                        
                        return vec![BeamState {
                            direction: state.direction,
                            point: next_point,
                        }];
                    }
                    UP | DOWN => {
                        return vec![
                            BeamState {
                                direction: RIGHT,
                                point: state.point + Point { x: 1, y: 0 },
                            },
                            BeamState {
                                direction: LEFT,
                                point: state.point + Point { x: -1, y: 0 },
                            }
                        ];
                    }
                    _ => panic!("direction not in NEWS"),
                };
            }
        }
    }
    fn get_energized(&self) -> usize {
        self.cells.iter().filter(|x| x.1.beams != 0).count()
    }
}

fn part_one(grid: &Grid) -> usize {
    // start at top-left, pointing right
    let traversed = grid.traverse(BeamState {
        point: Point { x: 0, y: 0 },
        direction: RIGHT,
    });

    traversed.get_energized()
}

fn part_two(grid: &Grid) -> usize {
    let mut best = 0;

    let mut run_it = |x, y, direction| {
        let cloned = grid.traverse(BeamState {
            direction,
            point: Point { x: x as isize, y: y as isize },
        });

        // first time using max
        best = best.max(cloned.get_energized());
    };

    let height = grid.cells.keys().map(|c| c.y).max().unwrap();
    let width = grid.cells.keys().map(|c| c.x).max().unwrap();
    
    for x in 0..=width {
        run_it(x, 0, DOWN);
        run_it(x, height, UP);
    }

    for y in 0..=height {
        run_it(0, y, RIGHT);
        run_it(width, y, LEFT);
    }
    
    best
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

        assert_eq!(ans, 46);
    }

    #[test]
    fn test_part_two() {
        let grid = Grid::new(EXAMPLE);
        let ans = part_two(&grid);

        assert_eq!(ans, 51);
    }
}

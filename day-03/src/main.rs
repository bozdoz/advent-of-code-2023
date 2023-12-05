use std::{ time::Instant, collections::HashMap, str::FromStr, fs };
use lib::get_part;

#[derive(Debug, Eq, Hash, PartialEq, Copy, Clone)]
struct Cell {
    x: isize,
    y: isize,
}

#[derive(Debug)]
enum CellValue {
    Symbol(char),
    /** number, len */
    Number(i32, isize),
    /** end of a number points to the beginning (part two) */
    Pointer(Cell),
}

struct Neighbours {
    start: Cell,
    end: Cell,
    cur: Option<Cell>,
}

impl Neighbours {
    // use Neighbours::new to avoid passing a `cur` value
    fn new(start: Cell, end: Cell) -> Self {
        Neighbours {
            // get top-left of this cell
            start: Cell { x: start.x - 1, y: start.y - 1 },
            // get bottom-right of this cell
            end: Cell {
                x: end.x + 1,
                y: end.y + 1,
            },
            // start at None to indicate first iteration
            cur: None,
        }
    }
}

impl Iterator for Neighbours {
    type Item = Cell;

    fn next(&mut self) -> Option<Self::Item> {
        // left -> right, top -> bottom
        if self.cur.is_none() {
            // start top-left!
            self.cur = Some(self.start);

            return self.cur;
        }

        // current is definitely defined
        let mut cur = self.cur.unwrap();

        // we're done if we reach the end
        if cur == self.end {
            return None;
        }

        // increase x
        cur.x += 1;

        // check if we hit the end of the row
        if cur.x > self.end.x {
            // wrap to next line
            cur.y += 1;
            cur.x = self.start.x;
        // check if we're in the middle cells (not boundary)
        } else if
            cur.x != self.start.x &&
            cur.x != self.end.y &&
            cur.y != self.start.y &&
            cur.y != self.end.y
        {
            // inside boundary
            cur.x = self.end.x;
        }

        self.cur = Some(cur);

        return self.cur;
    }
}

#[derive(Debug)]
struct Grid {
    cells: HashMap<Cell, CellValue>,
}

// so I can use .parse() on a string
impl FromStr for Grid {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines = s.lines();
        let mut map: HashMap<Cell, CellValue> = HashMap::new();

        for (y, row) in lines.enumerate() {
            let mut x = 0;
            let max = row.len() as isize;
            let mut chars = row.chars();

            while let Some(mut char) = chars.next() {
                x += 1;
                if char.is_digit(10) {
                    // parse number
                    let pos = Cell { x, y: y as isize };
                    // avoid 'move' of pos (below)
                    let mut pos_x = x;
                    let mut num = char.to_digit(10).unwrap() as i32;

                    while let Some(next) = chars.next() {
                        x += 1;

                        if next.is_digit(10) {
                            num =
                                num * 10 + (next.to_digit(10).unwrap() as i32);

                            // Doh! check if we're done the loop
                            if x != max {
                                continue;
                            } else {
                                // ignore the next line
                                char = '.';
                            }
                        } else {
                            char = next;
                        }

                        map.insert(
                            pos,
                            CellValue::Number(num, (x as isize) - pos_x)
                        );

                        // add pointers to the number cell for all other digits
                        // #[i_am_lazy]
                        while pos_x < x - 1 {
                            pos_x += 1;
                            map.insert(
                                Cell { x: pos_x, y: pos.y },
                                CellValue::Pointer(pos)
                            );
                        }

                        break;
                    }
                }
                if char == '.' {
                    continue;
                }
                // symbol
                let pos = Cell { x, y: y as isize };
                map.insert(pos, CellValue::Symbol(char));
            }
        }

        Ok(Grid { cells: map })
    }
}

impl Grid {
    fn has_neighbouring_symbol(&self, cell: &Cell, len: &isize) -> bool {
        let start = cell;
        let end = Cell { x: cell.x + len - 1, y: cell.y };
        let neighbours = Neighbours::new(*start, end);

        // 🐴
        for neigh in neighbours {
            if let Some(value) = self.cells.get(&neigh) {
                match value {
                    CellValue::Symbol(_) => {
                        return true;
                    }
                    _ => (),
                }
            }
        }

        false
    }
}

fn part_one(grid: &Grid) -> i128 {
    let mut sum: i128 = 0;

    // iterate numbers
    // without `.iter` it moves/consumes the variables and can't be used again
    for (cell, value) in grid.cells.iter() {
        match value {
            CellValue::Number(num, len) => {
                if grid.has_neighbouring_symbol(cell, len) {
                    sum += *num as i128;
                }
            }
            _ => (),
        }
    }

    sum
}

fn part_two(grid: &Grid) -> usize {
    let mut sum = 0;

    // iterate '*' signs and get those neighbours
    for (cell, value) in grid.cells.iter() {
        if let CellValue::Symbol(sym) = value {
            if sym == &'*' {
                // get neighbouring numbers
                // but avoid adding the same cell twice
                let mut parts: HashMap<Cell, usize> = HashMap::new();

                let start = *cell;
                let end = *cell;
                let neighbours = Neighbours::new(start, end);

                for neigh in neighbours {
                    if let Some(n) = grid.cells.get(&neigh) {
                        match n {
                            CellValue::Number(v, _) => {
                                parts.insert(neigh, *v as usize);
                            }
                            CellValue::Pointer(c) => {
                                let p = grid.cells.get(c).unwrap();

                                // how deep are we here?
                                if let CellValue::Number(v, _) = p {
                                    parts.insert(*c, *v as usize);
                                }
                            }
                            _ => (),
                        }
                    }
                }

                if parts.len() == 2 {
                    let prod = parts.values().product::<usize>();
                    sum += prod;
                }
            }
        }
    }

    sum
}

fn main() {
    let (one, two) = get_part();
    let start = Instant::now();
    let contents = fs::read_to_string("./src/input.txt").unwrap();

    let grid: Grid = contents.parse().expect("didn't get a grid?");

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
    fn test_parser() {
        let grid = EXAMPLE.parse::<Grid>();

        assert!(grid.is_ok())
    }

    #[test]
    fn test_neighbours() {
        let start = Cell { x: 1, y: 1 };
        let end = Cell { x: 1, y: 1 };
        let neigh = Neighbours::new(start, end);

        let cells: Vec<Cell> = neigh.collect();

        assert!(cells.len() == 8);
        assert!(cells[0] == Cell { x: 0, y: 0 });
        assert!(cells[1] == Cell { x: 1, y: 0 });
        assert!(cells[2] == Cell { x: 2, y: 0 });
        assert!(cells[3] == Cell { x: 0, y: 1 });
        assert!(cells[4] == Cell { x: 2, y: 1 });
        assert!(cells[5] == Cell { x: 0, y: 2 });
        assert!(cells[6] == Cell { x: 1, y: 2 });
        assert!(cells[7] == Cell { x: 2, y: 2 });
    }

    #[test]
    fn test_small_grid() {
        let grrr = "467.\n...*";

        let grid: Grid = grrr.parse().unwrap();

        let ans = part_one(&grid);

        assert_eq!(ans, 467);
    }

    #[test]
    fn test_side() {
        let grrr = "....\n617*\n....";

        let grid: Grid = grrr.parse().unwrap();

        let ans = part_one(&grid);

        assert_eq!(ans, 617);
    }

    #[test]
    fn test_part_one() {
        let grid = EXAMPLE.parse::<Grid>().unwrap();

        let ans = part_one(&grid);

        assert_eq!(ans, 4361);
    }

    #[test]
    fn test_eol() {
        let grrr = "..*\n617";

        let grid: Grid = grrr.parse().unwrap();

        let ans = part_one(&grid);

        assert_eq!(ans, 617);
    }

    #[test]
    fn test_eol_zero() {
        let grrr = "...\n617";

        let grid: Grid = grrr.parse().unwrap();

        let ans = part_one(&grid);

        assert_eq!(ans, 0);
    }

    #[test]
    fn test_pointers() {
        let grrr = "617.";

        let grid: Grid = grrr.parse().unwrap();

        if
            let CellValue::Number(_, _) = grid.cells
                .get(&(Cell { x: 1, y: 0 }))
                .unwrap()
        {
            assert!(true);
        } else {
            assert!(false);
        }

        let pointer2 = grid.cells.get(&(Cell { x: 2, y: 0 }));

        assert!(pointer2.is_some());

        let pointer3 = grid.cells.get(&(Cell { x: 3, y: 0 }));

        assert!(pointer3.is_some());

        let pointer4 = grid.cells.get(&(Cell { x: 4, y: 0 }));

        assert!(pointer4.is_none());
    }

    #[test]
    fn test_part_two() {
        let grid = EXAMPLE.parse::<Grid>().unwrap();

        let ans = part_two(&grid);

        assert_eq!(ans, 467835);
    }
}

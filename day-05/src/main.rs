use std::{ time::Instant, fs, vec };
use lib::get_part;

// first custom typing
/** (from, to, range len) */
type Mapping = (usize, usize, usize);

#[derive(Debug)]
struct Almanac {
    seeds: Vec<usize>,
    // cut to the chase
    seed_location: Vec<Vec<Mapping>>,
}

impl Almanac {
    fn new(data: &str) -> Self {
        let mut lines = data.lines();

        // get seeds from first line
        let first = lines.next().unwrap();
        let seeds: Vec<usize> = first
            .split_once(": ")
            .unwrap()
            .1.split_whitespace()
            .map(|x| { x.parse().unwrap() })
            .collect();

        let mut seed_location: Vec<Vec<Mapping>> = vec![];
        let mut cur_map: Vec<Mapping> = vec![];

        // skip blank line & first "map:" line
        for line in lines.skip(2) {
            if line.is_empty() {
                continue;
            }
            if line.ends_with("map:") {
                // push map
                seed_location.push(cur_map);

                cur_map = vec![];
                continue;
            }
            // else, this is a row of numbers
            let nums: Vec<usize> = line
                .split_whitespace()
                .map(|x| { x.parse().unwrap() })
                .collect();

            cur_map.push((nums[0], nums[1], nums[2]));
        }

        // push last map
        seed_location.push(cur_map);

        Almanac { seeds, seed_location }
    }

    fn seed_to_location(&self, seed: usize) -> usize {
        let mut cur = seed;
        'outer: for map in self.seed_location.iter() {
            for mapping in map.iter() {
                let mapped = x_to_y_map(cur, mapping);
                match mapped {
                    Some(n) => {
                        cur = n;
                        // go to next map
                        continue 'outer;
                    }
                    None => (),
                }
            }
        }

        cur
    }

    // reverse
    fn location_to_seed(&self, location: usize) -> Option<usize> {
        let mut cur = location;

        'outer: for map in self.seed_location.iter().rev() {
            for mapping in map.iter() {
                let mapped = y_to_x_map(cur, mapping);
                match mapped {
                    Some(n) => {
                        cur = n;
                        // go to next map
                        continue 'outer;
                    }
                    None => (),
                }
            }
        }

        for seed in self.seeds.chunks(2) {
            if (seed[0]..seed[0]+seed[1]).contains(&cur) {
                return Some(cur)
            }
        }

        None
    }
}

fn x_to_y_map(n: usize, mapping: &Mapping) -> Option<usize> {
    let (y, x, len) = *mapping;

    if n < x || n > x + len {
        return None;
    }

    // get diff of x and y?
    if x > y {
        let diff = x - y;

        return Some(n - diff);
    }
    let diff = y - x;

    Some(n + diff)
}

fn y_to_x_map(n: usize, mapping: &Mapping) -> Option<usize> {
    let (x, y, len) = *mapping;

    if n < x || n > x + len {
        return None;
    }

    // get diff of x and y?
    if x > y {
        let diff = x - y;

        return Some(n - diff);
    }
    let diff = y - x;

    Some(n + diff)
}

fn part_one(almanac: &Almanac) -> usize {
    let mut smallest = usize::MAX;

    for seed in almanac.seeds.iter() {
        let location = almanac.seed_to_location(*seed);

        if location < smallest {
            smallest = location;
        }
    }

    smallest
}

fn part_two(almanac: &Almanac) -> usize {
    let smallest = usize::MAX;

    for l in 0usize.. {
        // just go crazy
        if almanac.location_to_seed(l).is_some() {
            return l;
        }
    }

    smallest
}

fn main() {
    let (one, two) = get_part();
    let start = Instant::now();
    let contents = fs::read_to_string("./src/input.txt").unwrap();

    let almanac = Almanac::new(contents.as_str());

    if one {
        let now = Instant::now();
        let ans = part_one(&almanac);
        println!("Part one: {:?} {:?}", ans, now.elapsed());
    }

    if two {
        let now = Instant::now();
        let ans = part_two(&almanac);
        println!("Part two: {:?} {:?}", ans, now.elapsed());
    }

    println!("Time: {:?}", start.elapsed())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("./example.txt");

    #[test]
    fn test_x_y_map() {
        assert_eq!(x_to_y_map(55, &(50, 98, 2)), None);
        
        assert_eq!(x_to_y_map(79, &(52, 50, 48)), Some(81));
    }

    #[test]
    fn test_seed_to_location() {
        let almanac = Almanac::new(EXAMPLE);

        assert_eq!(almanac.seed_to_location(79), 82);
        assert_eq!(almanac.seed_to_location(14), 43);
        assert_eq!(almanac.seed_to_location(55), 86);
        assert_eq!(almanac.seed_to_location(13), 35);
    }

    #[test]
    fn test_part_one() {
        let almanac = Almanac::new(EXAMPLE);
        let ans = part_one(&almanac);

        assert_eq!(ans, 35);
    }

    #[test]
    fn test_location_to_seed() {
        let almanac = Almanac::new(EXAMPLE);

        assert_eq!(almanac.location_to_seed(82), Some(79));
        assert_eq!(almanac.location_to_seed(86), Some(55));
        assert_eq!(almanac.location_to_seed(0), None);
    }

    #[test]
    fn test_part_two() {
        let almanac = Almanac::new(EXAMPLE);
        let ans = part_two(&almanac);

        assert_eq!(ans, 46);
    }
}

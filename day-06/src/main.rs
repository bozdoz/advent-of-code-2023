use lib::get_part;
use std::{fs, time::Instant};

struct Race {
    time: usize,
    distance: usize,
}

// TODO: can I have a struct without a field? or impl methods on a non-struct?
struct Races {
    // TODO: why can't this be `[Race]`; I thought that was more flexible typing?
    data: Vec<Race>,
}

impl Races {
    fn new(content: &str) -> Self {
        let time = vec![];
        let distance = vec![];
        let lines = content.lines().flat_map(f);
        let data = vec![];

        Races { data }
    }
}

fn part_one() -> usize {
    0
}

fn part_two() -> usize {
    0
}

fn main() {
    let (one, two) = get_part();
    let start = Instant::now();
    let contents = fs::read_to_string("./src/input.txt").unwrap().as_str();

    if one {
        let now = Instant::now();
        let ans = part_one();
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
    fn test_parsing() {
        let races = Races::new(EXAMPLE);

        assert_eq!(races.data.len(), 3);
    }

    #[test]
    fn test_part_one() {
        let ans = part_one();

        assert_eq!(ans, 0);
    }
}

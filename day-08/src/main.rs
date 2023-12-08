use std::{time::Instant, fs, collections::HashMap};
use lib::get_part;

#[derive(Debug, PartialEq)]
enum Dir {
    L,
    R
}

// first time with lifetimes?
struct Network<'a> {
    instructions: Vec<Dir>,
    elements: HashMap<&'a str, (&'a str, &'a str)>,
    /** part 2 keys are not just "AAA", but anything ending in "A" */
    start_keys: Vec<&'a str>
}

impl Network<'_> {
    fn new(contents: &str) -> Network<'_> {
        let mut lines = contents.lines();
        
        let instructions: Vec<Dir> = lines.next().unwrap().chars().map(|x| {
            if x == 'L' {
                return Dir::L
            }
            Dir::R
        }).collect();

        let mut elements = HashMap::new();

        for line in lines.skip(1) {
            let key = &line[..3];
            let left = &line[7..10];
            let right = &line[12..15];

            elements.insert(key, (left, right));
        }

        Network { instructions, elements, start_keys: vec![] }
    }
    fn add_start_keys(&mut self) {
        let mut start_keys: Vec<&str> = vec![];
        for (key, _) in self.elements.iter() {
            if key.chars().nth(2).unwrap() == 'A' {
                start_keys.push(key);
            }
        }

        self.start_keys = start_keys;
    }
    fn count_path_from(&self, keys: &Vec<&str>) -> (usize, Vec<&str>) {
        // iterate instructions, return # of iterations
        let mut key = keys[0];
        let mut i = 0;
        let len = self.instructions.len();

        loop {
            let (l, r) = self.elements.get(key).unwrap();
            
            key = match self.instructions[i % len] {
                Dir::L => { l }
                Dir::R => { r }
            };

            i += 1;

            if key.chars().nth(2).unwrap() == 'Z' { 
                return (i, vec!["AAA"])
            }
        }
    }
}

fn part_one(network: &Network) -> usize {
    network.count_path_from(&vec!["AAA"]).0
}

fn part_two(network: &mut Network) -> usize {
    network.add_start_keys();
    network.count_path_from(&network.start_keys).0
}

fn main() {
    let (one, two) = get_part();
    let start = Instant::now();
    let contents = fs::read_to_string("./src/input.txt").unwrap();

    let mut network = Network::new(contents.as_str());

    if one {
        let now = Instant::now();
        let ans = part_one(&network);
        println!("Part one: {:?} {:?}", ans, now.elapsed());
    }

    if two {
        let now = Instant::now();
        let ans = part_two(&mut network);
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
        let network = Network::new(EXAMPLE);
        
        assert_eq!(network.instructions.len(), 3);
        // ne!
        assert_ne!(network.instructions[0], Dir::R);
        assert_eq!(network.instructions[0], Dir::L);
        assert_eq!(network.instructions[1], Dir::L);
        assert_eq!(network.instructions[2], Dir::R);

        assert_eq!(network.elements.len(), 3);
    }

    #[test]
    fn test_part_one() {
        let network = Network::new(EXAMPLE);
        let ans = part_one(&network);

        assert_eq!(ans, 6);
    }

    const EXAMPLE_2: &str = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";

    #[test]
    fn test_start_keys() {
        let mut network = Network::new(EXAMPLE_2);
        
        assert_eq!(network.start_keys.len(), 0);
        
        network.add_start_keys();

        assert_eq!(network.start_keys.len(), 2);
    }

    #[test]
    fn test_part_two() {
        let mut network = Network::new(EXAMPLE_2);
        let ans = part_two(&mut network);

        assert_eq!(ans, 6);
    }
}

use lib::get_part;
use std::{ fs, time::Instant };

#[derive(Debug)]
struct Race {
    time: usize,
    dist: usize,
}

impl Race {
    fn list(contents: &str) -> Vec<Self> {
        let lines: Vec<_> = contents
            .lines()
            .map(|l| {
                l.split_once(":")
                    .unwrap()
                    .1.split_whitespace()
                    .map(|x| { x.parse::<usize>().unwrap() })
                    .collect()
            })
            .collect();
        let time: &Vec<usize> = &lines[0];
        let dist: &Vec<usize> = &lines[1];

        let mut data = vec![];

        for (i, t) in time.iter().enumerate() {
            data.push(Race { time: *t, dist: dist[i] });
        }

        data
    }

    fn one_big_one(contents: &str) -> Self {
        let lines: Vec<_> = contents
            .lines()
            .map(|l| {
                l.split_once(":")
                    .unwrap()
                    .1.split_whitespace()
                    .collect::<Vec<&str>>()
                    .join("")
                    .parse::<usize>().unwrap()
            })
            .collect();

        Race { time: lines[0], dist: lines[1] }
    }
}

fn get_dist_for_hold(time: usize) -> Vec<usize> {
    let mut ret = vec![];

    for t in 1..time {
        // boat speed increases 1 mm/ms
        ret.push(t * (time - t));
    }

    ret
}

fn part_one(races: Vec<Race>) -> usize {
    races.iter().map(|r: &Race| {
        get_dist_for_hold(r.time).iter().filter(|x: &&usize| {
            **x > r.dist
        }).count()
    }).product()
}

fn part_two(race: Race) -> usize {
    get_dist_for_hold(race.time).iter().filter(|x| {
        **x > race.dist
    }).count()
}

fn main() {
    let (one, two) = get_part();
    let start = Instant::now();
    let contents = fs::read_to_string("./src/input.txt").unwrap();

    let races = Race::list(contents.as_str());

    if one {
        let now = Instant::now();
        let ans = part_one(races);
        println!("Part one: {:?} {:?}", ans, now.elapsed());
    }

    if two {
        let now = Instant::now();
        let ans = part_two(Race::one_big_one(contents.as_str()));
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
        let races = Race::list(EXAMPLE);

        assert_eq!(races.len(), 3);
    }

    #[test]
    fn test_dist_holds() {
        assert_eq!(get_dist_for_hold(2), [1]);
        assert_eq!(get_dist_for_hold(3), [2, 2]);
        assert_eq!(get_dist_for_hold(7), [6, 10, 12, 12, 10, 6]);
    }

    #[test]
    fn test_part_one() {
        let races = Race::list(EXAMPLE);
        let ans = part_one(races);

        assert_eq!(ans, 288);
    }

    #[test]
    fn test_one_big_one() {
        let race = Race::one_big_one(EXAMPLE);

        assert_eq!(race.time, 71530);
        assert_eq!(race.dist, 940200);
    }

    #[test]
    fn test_part_two() {
        let race = Race::one_big_one(EXAMPLE);
        let ans = part_two(race);

        assert_eq!(ans, 71503);
    }
}

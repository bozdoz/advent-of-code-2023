use std::{ time::Instant, fs, vec };
use lib::get_part;

fn get_extrapolation(history: Vec<isize>) -> isize {
    let mut diffs = vec![history];
    loop {
        let cur = &diffs[diffs.len() - 1];
        let mut next: Vec<isize> = vec![];
        // rev'ing twice to skip last
        for (i, v) in cur.iter().rev().skip(1).rev().enumerate() {
            let diff = cur[i + 1] - v;

            next.push(diff);
        }

        if next.iter().all(|x| x == &0) {
            break;
        }

        diffs.push(next);
    }
    // now get the last value of all diffs
    let mut prediction = 0;

    for diff in diffs.iter().rev() {
        prediction += diff.last().expect("is this what you like?");
    }

    prediction
}

fn parse(contents: &str) -> Vec<Vec<isize>> {
    contents
        .lines()
        .map(|l| {
            l.split_whitespace()
                .map(|x| x.parse().expect("is idiomatic"))
                .collect()
        })
        .collect()
}

fn part_one(histories: &Vec<Vec<isize>>) -> isize {
    histories
        .iter()
        .map(|x| get_extrapolation(x.to_vec()))
        .sum()
}

fn part_two(histories: &Vec<Vec<isize>>) -> isize {
    histories
        .iter()
        .map(|x|
            get_extrapolation({
                let mut v = x.to_vec();
                // reverse to get left-side instead
                v.reverse();
                v
            })
        )
        .sum()
}

fn main() {
    let (one, two) = get_part();
    let start = Instant::now();
    let contents = fs::read_to_string("./src/input.txt").unwrap();

    let histories = parse(&contents);

    if one {
        let now = Instant::now();
        let ans = part_one(&histories);
        println!("Part one: {:?} {:?}", ans, now.elapsed());
    }

    if two {
        let now = Instant::now();
        let ans = part_two(&histories);
        println!("Part two: {:?} {:?}", ans, now.elapsed());
    }

    println!("Time: {:?}", start.elapsed())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("./example.txt");

    #[test]
    fn test_extrapolation() {
        assert_eq!(get_extrapolation(vec![0, 3, 6, 9, 12, 15]), 18);
    }

    #[test]
    fn test_part_one() {
        let histories = parse(EXAMPLE);
        let ans = part_one(&histories);

        assert_eq!(ans, 114);
    }

    #[test]
    fn test_part_two() {
        let histories = parse(EXAMPLE);
        let ans = part_two(&histories);

        assert_eq!(ans, 2);
    }
}

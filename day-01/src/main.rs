use std::{ time::Instant, fs };
use lib::get_part;
use regex::Regex;

const FILENAME: &str = "./src/input.txt";

fn part_one(lines: &Vec<&str>) -> u16 {
    let mut numbers: Vec<u16> = vec![];

    for line in lines {
        let mut n: Vec<u8> = vec![];
        for char in line.chars() {
            if char.is_digit(10) {
                n.push(char.to_digit(10).unwrap() as u8);
            }
        }
        if n.is_empty() {
            // running example from part 2
            return 0;
        }
        let first = n[0];
        let last = n[n.len() - 1];
        numbers.push((first * 10 + last) as u16);
    }

    numbers.iter().sum()
}

fn replace_numbers(num: &str) -> &str {
    match num {
        "one" => "1",
        "two" => "2",
        "three" => "3",
        "four" => "4",
        "five" => "5",
        "six" => "6",
        "seven" => "7",
        "eight" => "8",
        "nine" => "9",
        n => n,
    }
}

/** just return two numbers */
fn replace_first_and_last_nums(s: &str) -> String {
    let re = Regex::new(
        r"(one|two|three|four|five|six|seven|eight|nine|\d)"
    ).unwrap();
    let mut two = vec!["0", "0"];

    // get first match
    if let Some(first) = re.find(s) {
        two[0] = replace_numbers(first.as_str());
    }

    // get last match
    for i in (0..=s.len() - 1).rev() {
        if let Some(last) = re.find(&s[i..]) {
            two[1] = replace_numbers(last.as_str());
            break;
        }
    }

    two.join("")
}

fn part_two(lines: &Vec<&str>) -> u16 {
    // need to force this to be a String
    let new_lines: Vec<String> = lines
        .iter()
        .map(|line| { replace_first_and_last_nums(line) })
        .collect();

    // need a new variable to convert String to &str
    // (can't do it above as the reference belongs to the map function)
    // cannot return value referencing function parameter `x`
    // returns a value referencing data owned by the current function
    let new_lines: Vec<&str> = new_lines
        .iter()
        .map(|x| x.as_str())
        .collect();

    // replace the string numbers and run part_one again
    part_one(&new_lines)
}

fn main() {
    let (one, two) = get_part();
    let start = Instant::now();
    let contents = fs
        ::read_to_string(FILENAME)
        .expect("couldn't open input file");

    let lines: Vec<&str> = contents.lines().collect();

    if one {
        let now = Instant::now();
        let ans = part_one(&lines);
        println!("Part one: {} {:?}", ans, now.elapsed());
    }

    if two {
        let now = Instant::now();
        let ans = part_two(&lines);
        println!("Part two: {} {:?}", ans, now.elapsed());
    }

    println!("Time: {:?}", start.elapsed())
}

#[cfg(test)]
mod tests {
    // Import the necessary items from the standard library for testing
    use super::*;

    // A basic unit test for the main function
    #[test]
    fn test_number_parser() {
        let input = vec!["oneight"];

        let ans = part_two(&input);

        // Use the assert_eq! macro to check if the output matches the expected result
        assert_eq!(ans, 18);
    }
}

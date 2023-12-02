#!/bin/bash

NEW_DAY=$1

usage() {
    cat >&2 <<END_USAGE

Create a new boilerplate directory

USAGE:
    ./create-day.sh 01
END_USAGE
}

if [ -z $NEW_DAY ]; then
  echo "Provide ## for new day directory"
	usage
  exit 1
fi

cargo new "day-${NEW_DAY}" || usage

cd "day-${NEW_DAY}"

cargo add --path ../lib

# create input files for testing and solving
touch ./src/example.txt
touch ./src/input.txt

cat > src/main.rs <<EOF
use std::{ time::Instant, fs };
use lib::get_part;

const FILENAME: &str = "./src/example.txt";

fn part_one() {

}

fn part_two() {
    
}

fn main() {
    let (one, two) = get_part();
    let start = Instant::now();
    let contents = fs
        ::read_to_string(FILENAME)
        .expect("couldn't open input file");

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

    #[test]
    fn test_number_parser() {
        let ans = part_one();

        assert_eq!(ans, ());
    }
}
EOF
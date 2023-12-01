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

cat > src/main.rs <<EOF
use lib::get_part;

fn main() {
    let (one, two) = get_part();

    println!("{} {}", one, two);
}
EOF
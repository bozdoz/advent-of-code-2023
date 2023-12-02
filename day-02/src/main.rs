use std::{ time::Instant, fs, vec };
use lib::get_part;

const FILENAME: &str = "./src/input.txt";

#[derive(Default, Debug)]
struct Set {
    red: u32,
    green: u32,
    blue: u32,
}

#[derive(Default, Debug)]
struct Game {
    sets: Vec<Set>
}

impl Game {
    // parse a game from the input
    fn new(line: &str) -> Game {
        let info: &str = line.split(": ").nth(1).unwrap();
        let sets = info.split("; ");
        let mut game = Game::default();

        for set in sets {
            let mut cur_set = Set::default();
            let dice = set.split(", ");

            for die in dice {
                let mut split = die.split(" ");
                let (num, color ) = (split.next().unwrap(), split.next().unwrap());
                
                match color {
                    "red" => {
                        cur_set.red = num.parse().expect("red not a number");
                    }
                    "green" => {
                        cur_set.green = num.parse().expect("green not a number");
                    }
                    "blue" => {
                        cur_set.blue = num.parse().expect("blue not a number");
                    }
                    n => panic!("not a color: {}", n)
                }
            }

            game.sets.push(cur_set);
        }

        game
    }
}

fn part_one(games: &Vec<Game>) -> usize {
    let mut possible = 0;
    
    'outer: for i in 0..games.len() {
        let game = &games[i];
        
        // iterate sets
        for set in &game.sets {
            // rules: only 12 red cubes, 13 green cubes, and 14 blue cubes
            if set.red > 12 || set.green > 13 || set.blue > 14 {
                continue 'outer;
            }
        }
        possible += i+1;
    }

    possible
}

fn part_two(games: &Vec<Game>) -> u32 {
    let mut power_sum: u32 = 0;

    for game in games {
        // get max for all colors in all sets
        let mut max_colors = Set::default();
        for set in &game.sets {
            if max_colors.red < set.red {
                max_colors.red = set.red;
            }

            if max_colors.green < set.green {
                max_colors.green = set.green;
            }

            if max_colors.blue < set.blue {
                max_colors.blue = set.blue;
            }
        }

        power_sum += max_colors.red * max_colors.green * max_colors.blue;
    }

    power_sum
}

fn main() {
    let (one, two) = get_part();
    let start = Instant::now();
    let contents = fs
        ::read_to_string(FILENAME)
        .expect("couldn't open input file");

    let lines: Vec<&str> = contents.lines().collect();
    let mut games: Vec<Game> = vec![];

    for line in lines {
        games.push(Game::new(line));
    }

    if one {
        let now = Instant::now();
        let ans = part_one(&games);
        println!("Part one: {:?} {:?}", ans, now.elapsed());
    }

    if two {
        let now = Instant::now();
        let ans = part_two(&games);
        println!("Part two: {:?} {:?}", ans, now.elapsed());
    }

    println!("Time: {:?}", start.elapsed())
}

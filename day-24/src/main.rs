use std::{ time::Instant, fs, str::FromStr, io::Error, vec };
use lib::get_part;

struct Vec3d(isize, isize, isize);

impl FromStr for Vec3d {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let pos = s
            .split(", ")
            .map(|x| x.parse::<isize>().unwrap())
            .collect::<Vec<isize>>();

        Result::Ok(Self(pos[0], pos[1], pos[2]))
    }
}

struct Hailstone {
    position: Vec3d,
    velocity: Vec3d,
    slope: isize,
    intercept: isize,
}

impl Hailstone {
    fn new(position: Vec3d, velocity: Vec3d) -> Self {
        let (x1, y1, z1) = (position.0, position.1, position.2);
        let (x2, y2, z2) = (x1 + velocity.0, y1 + velocity.1, z1 + velocity.2);

        // TODO: this is 2d
        let slope = (y2 - y1) / (x2 - x1);
        
        // TODO: this is 2d
        let intercept = y1 - slope * x1;

        Self {
            position,
            velocity,
            slope,
            intercept
        }
    }

    fn intersects(&self, b: Hailstone) -> Vec3d {
        // x0​= (b2​−b1) / (a1​−a2)
        // ​​y0​= a1*(b2​−b1​​)/(​a1​−a2​)+b1​
    }
}

fn parseHailstones(contents: &str) -> Vec<Hailstone> {
    let mut hailstones = vec![];
    let lines = contents.lines();

    for line in lines {
        let (pos, vel) = line.split_once(" @ ").expect("expected '@'");

        let position = pos.parse().expect("position");
        let velocity = vel.parse().expect("velocity");

        hailstones.push(Hailstone::new(
            position,
            velocity,
        ));
    }

    return hailstones;
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
    let contents = fs::read_to_string("./src/input.txt").unwrap();

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
    fn test_part_one() {
        let ans = part_one();

        assert_eq!(ans, 0);
    }

    #[test]
    fn test_part_two() {
        let ans = part_two();

        assert_eq!(ans, 0);
    }
}

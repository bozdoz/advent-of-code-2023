use std::{
    time::Instant,
    fs,
    str::FromStr,
    io::Error,
    vec,
};
use lib::get_part;

struct Vec3d(isize, isize, isize);

impl FromStr for Vec3d {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let pos = s
            .split(", ")
            .map(|x| {
                let num = x.trim().parse::<isize>();

                if num.is_err() {
                    panic!("what is this? {}", x);
                }

                num.unwrap()
            })
            .collect::<Vec<isize>>();

        Result::Ok(Self(pos[0], pos[1], pos[2]))
    }
}

struct Hailstone {
    position: Vec3d,
    velocity: Vec3d,
    // dang
    slope: f64,
    intercept: f64,
}

impl Hailstone {
    fn new(position: Vec3d, velocity: Vec3d) -> Self {
        let (x1, y1, z1) = (position.0, position.1, position.2);
        let (x2, y2, _z2) = (x1 + velocity.0, y1 + velocity.1, z1 + velocity.2);

        // TODO: this is 2d
        let slope = ((y2 - y1) as f64) / ((x2 - x1) as f64);

        // TODO: this is 2d
        let intercept = (y1 as f64) - slope * (x1 as f64);

        Self {
            position,
            velocity,
            slope,
            intercept,
        }
    }

    // TODO: 2d
    // and wow f64
    fn intersects(&self, b: &Hailstone) -> Result<(f64, f64, f64), &str> {
        // x0​= (b2​−b1) / (a1​−a2)
        // ​​y0​= a1*x0+b1​
        let div: f64 = (self.slope - b.slope) as f64;

        if div == (0 as f64) {
            return Err("divide by zero");
        }

        if b.slope == self.slope {
            return Err("parallel");
        }

        let x = ((b.intercept - self.intercept) as f64) / div;
        let y = (self.slope as f64) * x + (self.intercept as f64);

        return Ok((x, y, 0 as f64));
    }

    fn is_past(&self, point: (f64, f64, f64)) -> bool {
        let (x, y) = (self.position.0 as f64, self.position.1 as f64);
        let (vx, vy) = (self.velocity.0 as f64, self.velocity.1 as f64);

        #[allow(unused_parens)]
        return (
            (vx < 0.0 && point.0 > x) ||
            (vx > 0.0 && point.0 < x) ||
            (vy < 0.0 && point.1 > y) ||
            (vy > 0.0 && point.1 < y)
        );
    }
}

fn parse_hailstones(contents: &str) -> Vec<Hailstone> {
    let mut hailstones = vec![];
    let lines = contents.lines();

    for line in lines {
        let (pos, vel) = line.split_once(" @ ").expect("expected '@'");

        let position = pos.parse().expect("position");
        let velocity = vel.parse().expect("velocity");

        hailstones.push(Hailstone::new(position, velocity));
    }

    return hailstones;
}

fn get_intersecting_count(
    hailstones: &Vec<Hailstone>,
    boundary: (f64, f64)
) -> usize {
    let mut count = 0;
    let r = boundary.0..boundary.1;

    for (i, h1) in hailstones.iter().enumerate() {
        for (j, h2) in hailstones.iter().enumerate() {
            if j <= i {
                // already seen this hailstone
                continue;
            }
            let intersection = h1.intersects(h2);

            if let Ok(intersection) = intersection {
                if r.contains(&intersection.0) && r.contains(&intersection.1) {
                    if !h1.is_past(intersection) && !h2.is_past(intersection) {
                        count += 1;
                        // println!("Yes: {}, {}, {:?}", i, j, (
                        //     intersection.0,
                        //     intersection.1,
                        // ));
                    } else {
                        // println!("No (past): {}, {}, {:?}", i, j, (
                        //     intersection.0,
                        //     intersection.1,
                        // ));
                    }
                } else {
                    // println!("No (contains): {}, {}, {:?}", i, j, (
                    //     intersection.0,
                    //     intersection.1,
                    // ));
                }
            } else {
                // println!("No: {}, {}, {}", i, j, intersection.unwrap_err());
            }
        }
    }

    count
}

fn part_one(hailstones: &Vec<Hailstone>) -> usize {
    let boundary = (200000000000000., 400000000000000.);
    
    get_intersecting_count(hailstones, boundary)
}

fn part_two() -> usize {
    0
}

fn main() {
    let (one, two) = get_part();
    let start = Instant::now();
    let contents = fs::read_to_string("./src/input.txt").unwrap();
    let hailstones = parse_hailstones(&contents);

    if one {
        let now = Instant::now();
        let ans = part_one(&hailstones);
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
    fn test_intersects() {
        let a = Hailstone {
            intercept: 2.0,
            slope: 3.0,
            position: Vec3d(0, 0, 0),
            velocity: Vec3d(0, 0, 0),
        };
        let b = Hailstone {
            intercept: -9.0,
            slope: 4.0,
            position: Vec3d(0, 0, 0),
            velocity: Vec3d(0, 0, 0),
        };

        assert_eq!(a.intersects(&b).unwrap(), (11.0, 35.0, 0.0));
    }

    #[test]
    fn test_intersections() {
        let hailstones: Vec<Hailstone> = parse_hailstones(EXAMPLE);

        let count = get_intersecting_count(&hailstones, (7.0, 27.0));

        assert_eq!(count, 2);
    }

    #[test]
    fn test_part_two() {
        let ans = part_two();

        assert_eq!(ans, 0);
    }
}

use std::{ env, ops::Add };

pub fn get_part() -> (bool, bool) {
    let args = env::args().skip(1);

    let mut hasone = false;
    let mut hastwo = false;

    for arg in args {
        if arg.contains(&String::from("one")) {
            hasone = true;
        }
        if arg.contains(&String::from("two")) {
            hastwo = true;
        }
    }

    if !hasone && !hastwo {
        // run them both by default
        hasone = true;
        hastwo = true;
    }

    (hasone, hastwo)
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone, PartialOrd, Ord)]
pub struct Point {
    pub x: isize,
    pub y: isize,
}

impl Add for Point {
    type Output = Point;
    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

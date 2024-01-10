use std::{time::Instant, fs, vec};
use lib::get_part;

// focal length ranging from 1 through 9
// The result of running the HASH algorithm on the label indicates the correct box for that step.
// "-" means to remove the lens and shift 
// "=" means that label/focal length goes in the box

#[derive(Debug)]
enum Action {
    Add,
    Remove
}


#[derive(Debug)]
struct Lens<'a> {
    label: &'a str,
    focal_len: usize,
    box_num: usize,
    action: Action
}

impl<'a> Lens<'a> {
    fn new(data: &'a str) -> Self {
        let (label, focal_len ) = data.split_once(&['-', '=']).expect("Wanted data to be splittable by - or =");
        
        let mut action = Action::Add;
        
        let focal_len = focal_len.parse::<usize>();
        
        let focal_len = if focal_len.is_ok() {
            focal_len.unwrap()
        } else {
            // side effects ?
            action = Action::Remove;
            0
        };
        
        let box_num = hash(label);

        Self { label, focal_len, box_num, action }
    }
}

fn hash(content: &str) -> usize {
    let mut cur = 0;

    for char in content.chars() {
        cur += char as usize;
        cur *= 17;
        cur %= 256;
    }

    cur
}

fn part_one(contents: &str) -> usize {
    contents.trim().split(",").map(hash).sum()
}

fn part_two(content: &str) -> usize {
    let lenses = content.trim().split(",").map(Lens::new);

    // don't know how to make this:
    // [Vec<Lens>, 256]
    let mut boxes: Vec<Vec<Lens>> = (0..256).map(|_| vec![]).collect();

    for lens in lenses {
        let this = &mut boxes[lens.box_num];
        let some_index = this.iter().position(|x| x.label == lens.label);

        match lens.action {
            Action::Add => {
                if let Some(index) = some_index {
                    // replace
                    this.remove(index);
                    this.insert(index, lens);
                } else {
                    // add
                    boxes[lens.box_num].push(lens);
                }
            },
            Action::Remove => {
                if let Some(index) = some_index {
                    this.remove(index);
                }
            }
        }
    }

    boxes.iter().map(|v| {
        v.iter().enumerate().map(|(slot, l)| {
            (1 + l.box_num) * (slot + 1) * l.focal_len
        }).sum::<usize>()
    }).sum()
}

fn main() {
    let (one, two) = get_part();
    let start = Instant::now();
    let contents = fs::read_to_string("./src/input.txt").unwrap();

    if one {
        let now = Instant::now();
        let ans = part_one(&contents);
        println!("Part one: {:?} {:?}", ans, now.elapsed());
    }

    if two {
        let now = Instant::now();
        let ans = part_two(&contents);
        println!("Part two: {:?} {:?}", ans, now.elapsed());
    }

    println!("Time: {:?}", start.elapsed())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("./example.txt");

    #[test]
    fn test_hash() {
        assert_eq!(hash("rn=1"), 30);
    }

    #[test]
    fn test_part_one() {
        let ans = part_one(EXAMPLE);

        assert_eq!(ans, 1320);
    }

    #[test]
    fn test_part_two() {
        let ans = part_two(EXAMPLE);

        assert_eq!(ans, 145);
    }
}

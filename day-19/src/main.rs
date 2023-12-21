use std::{ time::Instant, fs, collections::HashMap, cmp::Ordering };
use lib::get_part;

// today's the biggest struct dependencies made so far
// all with lifetimes

struct Compare<'a> {
    // not sure how I could make this a key of a struct
    key: &'a str,
    cmp: Ordering,
    num: usize,
}

struct Rule<'a> {
    test: Option<Compare<'a>>,
    goto: &'a str,
}

struct Workflow<'a> {
    rules: Vec<Rule<'a>>,
}

struct System<'a> {
    workflows: HashMap<&'a str, Workflow<'a>>,
    parts: Vec<HashMap<&'a str, usize>>,
}

impl<'a> System<'a> {
    fn new(contents: &'a str) -> Self {
        let (workflow_str, parts_str) = contents
            .split_once("\n\n")
            .expect("I thought it was in two sections!");
        let mut workflows = HashMap::new();

        // TIL
        let trim: &[_] = &['{', '}'];
        let rule_symbols: &[_] = &['<', '>', ':'];

        for workflow in workflow_str.lines() {
            let mut details = workflow.split(trim).take(2);

            let name = details.next().expect("name");
            let rules = details
                .next()
                .expect("rules")
                .split(",")
                .map(|rule| {
                    let rule_parts = rule
                        .split(rule_symbols)
                        .collect::<Vec<&str>>();

                    if rule_parts.len() == 1 {
                        return Rule {
                            goto: rule_parts[0],
                            test: None,
                        };
                    }
                    let cmp = if rule.contains("<") {
                        Ordering::Less
                    } else {
                        Ordering::Greater
                    };

                    let test = Compare {
                        cmp,
                        key: rule_parts[0],
                        num: rule_parts[1].parse().expect("num"),
                    };

                    Rule {
                        goto: rule_parts.last().expect("goto"),
                        test: Some(test),
                    }
                })
                .collect();

            workflows.insert(name, Workflow {
                rules,
            });
        }

        let mut parts = vec![];

        for part_str in parts_str.lines() {
            let mut part = HashMap::new();

            part_str
                .trim_matches(trim)
                .split(",")
                .for_each(|cat| {
                    let (k, v) = cat.split_once("=").expect("xmas");

                    part.insert(k, v.parse().expect("not a number"));
                });

            parts.push(part);
        }

        Self { workflows, parts }
    }
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

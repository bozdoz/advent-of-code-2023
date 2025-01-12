use std::{ fmt, fs, time::Instant };
use lib::get_part;

// I think this is basically a nonogram
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
enum Condition {
    Operational, // "."
    Damaged, // "#"
    Unknown, // "?"
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Spring {
    list: Vec<Condition>,
    damaged_sizes: Vec<usize>,
    damaged_count: usize,
}

impl Spring {
    fn is_invalid(&self) -> bool {
        let mut consecutive = vec![];
        let mut cur = 0;
        let mut count = 0;
        let mut has_unknown = false;

        for condition in &self.list {
            match condition {
                Condition::Damaged => {
                    cur += 1;
                    count += 1;
                }
                Condition::Unknown => {
                    has_unknown = true;
                    count += 1;
                }
                Condition::Operational => {
                    if !has_unknown && cur > 0 {
                        // first time using unwrap_or
                        let damaged = self.damaged_sizes
                            .get(consecutive.len())
                            .unwrap_or(&0);

                        if cur > *damaged {
                            return true;
                        }

                        consecutive.push(cur);

                        cur = 0;
                    }
                }
            }
        }

        if count < self.damaged_count {
            return true;
        }

        if has_unknown {
            return false;
        }

        if cur > 0 {
            consecutive.push(cur);
        }

        &consecutive != &self.damaged_sizes
    }
}

impl fmt::Display for Spring {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let output = self.list
            .iter()
            .map(|x| {
                match x {
                    Condition::Operational => ".",
                    Condition::Damaged => "#",
                    Condition::Unknown => "?",
                }
            })
            .collect::<Vec<_>>()
            .join("");

        write!(f, "{}", output)
    }
}

fn parse_springs(data: &str) -> Vec<Spring> {
    // look, ma!
    data.lines()
        .map(|line| {
            // split line by spaces
            let (map, groups) = line
                .split_once(" ")
                .expect("no space to split?");

            let list = map
                .chars()
                .map(|c| {
                    match c {
                        '.' => Condition::Operational,
                        '#' => Condition::Damaged,
                        '?' => Condition::Unknown,
                        _ => panic!("what did you do!?"),
                    }
                })
                .collect();

            let damaged_sizes: Vec<usize> = groups
                .split(",")
                .map(|group| group.parse().expect("didn't get group"))
                .collect();

            let damaged_count = damaged_sizes.iter().sum();

            Spring {
                list,
                damaged_sizes,
                damaged_count,
            }
        })
        .collect()
}

// lazy duplication
fn parse_springs_five_times(data: &str) -> Vec<Spring> {
    // look, ma!
    data.lines()
        .map(|line| {
            // split line by spaces
            let (map, groups) = line
                .split_once(" ")
                .expect("no space to split?");

            let map = (0..5)
                .map(|_| map)
                .collect::<Vec<_>>()
                .join("?");

            let groups = (0..5)
                .map(|_| groups)
                .collect::<Vec<_>>()
                .join(",");

            let list = map
                .chars()
                .map(|c| {
                    match c {
                        '.' => Condition::Operational,
                        '#' => Condition::Damaged,
                        '?' => Condition::Unknown,
                        _ => panic!("what did you do!?"),
                    }
                })
                .collect();

            let damaged_sizes: Vec<usize> = groups
                .split(",")
                .map(|group| group.parse().expect("didn't get group"))
                .collect();

            let damaged_count = damaged_sizes.iter().sum();

            Spring {
                list,
                damaged_sizes,
                damaged_count,
            }
        })
        .collect()
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct State {
    spring: Spring,
    index: isize,
}

impl fmt::Display for State {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(f, "{}", self.spring)
    }
}

fn bfs(spring: &Spring) -> isize {
    let mut count = 0;
    let mut pq = vec![];

    let len = spring.list.len() as isize;
    let index = spring.list
        .iter()
        .position(|c| *c == Condition::Unknown)
        .or_else(|| Some(len as usize))
        .expect("why didn't this work?") as isize;

    pq.push(State {
        spring: spring.clone(),
        index: index,
    });

    // TODO: try depth-first recursive
    while pq.len() > 0 {
        let state = pq.pop().unwrap();

        // check if we're done
        if state.index == len {
            count += 1;
            continue;
        }

        println!("{} {}", &state, &state.index);

        let next_states = get_next_states(state);

        for next in next_states {
            println!("next {} {}", &next, &next.index);
            // check valid?
            if next.spring.is_invalid() {
                println!("invalid {}", &next);
                continue;
            }

            pq.push(next);
        }
    }

    count
}

fn dfs(spring: &Spring) -> isize {
    let index = spring.list
        .iter()
        .position(|c| *c == Condition::Unknown)
        .or_else(|| Some(spring.list.len() as usize))
        .expect("why didn't this work?") as isize;

    fn do_dfs(state: State) -> isize {
        if state.index == (state.spring.list.len() as isize) {
            1;
        } else {
            0;
        }

        let next_states = get_next_states(state);
        let mut count = 0;

        for next in next_states {
            if next.spring.is_invalid() {
                continue;
            }

            count += do_dfs(next);
        }
        count
    }

    do_dfs(State {
        spring: spring.clone(),
        index,
    })
}

// state at this given index is a question mark
fn get_next_states(state: State) -> Vec<State> {
    let index =
        (
            state.spring.list
                .iter()
                .skip(state.index as usize)
                .position(|c| *c == Condition::Unknown)
                .or_else(|| Some(state.spring.list.len()))
                .expect("thought this would work") as isize
        ) + state.index;

    match state.spring.list[state.index as usize] {
        Condition::Unknown => {
            let mut damaged_spring = state.spring.clone();
            damaged_spring.list[state.index as usize] = Condition::Damaged;

            let mut operational_spring = state.spring.clone();
            operational_spring.list[state.index as usize] =
                Condition::Operational;

            let index = damaged_spring.list
                .iter()
                .skip(state.index as usize)
                .position(|c| *c == Condition::Unknown)
                .and_then(|i| Some((i as isize) + state.index))
                .or_else(|| Some(state.spring.list.len() as isize))
                .expect("thought this would work") as isize;

            vec![
                State {
                    spring: damaged_spring,
                    index: index,
                },
                State {
                    spring: operational_spring,
                    index: index,
                }
            ]
        }
        _ => {
            vec![State {
                spring: state.spring,
                index: index,
            }]
        }
    }
}

fn part_one(springs: &Vec<Spring>) -> isize {
    springs.iter().map(bfs).sum()
}

fn part_two(input: &str) -> isize {
    let springs = parse_springs_five_times(input);

    springs.iter().map(bfs).sum()
}

fn main() {
    let (one, two) = get_part();
    let start = Instant::now();
    let contents = fs::read_to_string("./src/input.txt").unwrap();

    let springs = parse_springs(&contents);

    if one {
        let now = Instant::now();
        let ans = part_one(&springs);
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
    fn test_ten() {
        let spr = parse_springs(".###.##.#.## 3,2,1");

        assert_eq!(spr[0].is_invalid(), true);

        let springs = parse_springs("?###???????? 3,2,1");

        assert_eq!(bfs(&springs[0]), 10);
    }

    #[test]
    fn test_four() {
        let spr = parse_springs(".#...#....###. 1,1,3");

        assert_eq!(spr[0].is_invalid(), false);

        let spr = parse_springs(".....??...?##. 1,1,3");

        assert_eq!(spr[0].is_invalid(), false);

        let spr = parse_springs("......?...?##. 1,1,3");

        assert_eq!(spr[0].is_invalid(), true);

        let springs = parse_springs(".??..??...?##. 1,1,3");
        assert_eq!(bfs(&springs[0]), 4);
    }

    #[test]
    fn test_invalid() {
        let springs = parse_springs("### 1,1\n.#. 1,1\n#.# 1,1");

        assert_eq!(springs[0].is_invalid(), true);
        assert_eq!(springs[1].is_invalid(), true);
        assert_eq!(springs[2].is_invalid(), false);
    }

    #[test]
    fn test_bfs() {
        // #.#.### 1,1,3
        // ???.### 1,1,3
        let springs = parse_springs("??? 1,1");

        assert_eq!(springs.len(), 1);

        assert_eq!(bfs(&springs[0]), 1);
        // assert_eq!(dfs(&springs[0]), 1);
    }

    #[ignore = "I don't know how to do dfs"]
    #[test]
    fn test_dfs() {
        // #.#.### 1,1,3
        // ???.### 1,1,3
        let springs = parse_springs("??? 1,1");

        assert_eq!(springs.len(), 1);

        assert_eq!(dfs(&springs[0]), 1);
    }

    #[test]
    fn test_part_one() {
        let springs = parse_springs(EXAMPLE);
        let ans = part_one(&springs);

        assert_eq!(ans, 21);
    }

    #[test]
    fn test_five_times() {
        let springs = parse_springs_five_times(EXAMPLE);

        assert_eq!(dbg!(&springs[0].list).len(), 39);
        assert_eq!(dbg!(&springs[0].damaged_sizes).len(), 15);
    }

    #[test]
    fn test_example_part_two() {
        let ans = part_two("????.#...#... 4,1,1");

        assert_eq!(ans, 16);
    }

    #[ignore = "This doesn't work"]
    #[test]
    fn test_part_two() {
        let ans = part_two(EXAMPLE);

        assert_eq!(ans, 525152);
    }
}

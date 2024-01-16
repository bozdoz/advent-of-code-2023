use std::{ time::Instant, fs, collections::{HashMap, HashSet}, vec, hash::{Hasher, Hash} };
use lib::get_part;

type Components<'a> = HashMap<&'a str, Vec<&'a str>>;

fn get_components(contents: &str) -> Components {
    let mut components = HashMap::new();
    let lines = contents.lines();

    for line in lines {
        let (name, links) = line.split_once(": ").expect("a colon");
        let links = links.split_whitespace();

        for link in links {
            components
                .entry(name)
                .and_modify(|v: &mut Vec<&str>| {
                    v.push(link);
                })
                .or_insert(vec![link]);

            // also update the link
            components
                .entry(link)
                .and_modify(|v: &mut Vec<&str>| {
                    v.push(name);
                })
                .or_insert(vec![name]);
        }
    }

    components
}

#[derive(Debug)]
struct State<'a> {
    current: &'a str,
    visited: HashSet<&'a str>,
    path: Vec<(&'a str, &'a str)>,
}

fn bfs<'a>(akey: &'a str, bkey: &'a str, components: &'a Components) -> Vec<(&'a str, &'a str)> {
    let start = State{
        current: akey,
        visited: HashSet::new(),
        path: vec![]
    };

    // TODO: try binary heap
    let mut queue = vec![start];

    while let Some(state) = queue.pop() {
        let current = state.current;

        if state.visited.contains(current) {
            continue;
        }

        let links = components.get(&current).expect("current not in components");

        // ? can we check if any of these are in bkey's links?
        for next in links {
            let mut path = vec![];

            // TIL
            path.extend(state.path.iter());
            path.push((current, *next));

            if *next == bkey {
                // done
                return path;
            }

            let mut visited = HashSet::new();

            for a in state.visited.iter() {
                visited.insert(*a);
            }

            visited.insert(current);

            queue.insert(0, State{
                current: next,
                path,
                visited
            })
        }
    }

    return vec![];
}

#[derive(Debug, Eq)]
struct Edge<'a>(&'a str, &'a str);

impl<'a> Edge<'a> {
    fn from(a: &'a str, b: &'a str) -> Self {
        Self(a.min(b), a.max(b))
    }
}

impl PartialEq<Self> for Edge<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.0.min(self.1) == other.0.min(other.1) &&
        self.0.max(self.1) == other.0.max(other.1)
    }
}

impl Hash for Edge<'_> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.min(self.1).hash(state);
        self.0.max(self.1).hash(state);
    }
}

fn bfs_everywhere<'a>(components: &'a Components) -> Vec<(&'a str, &'a str)> {
    let keys: Vec<&&str> = components.keys().collect();
    let mut distances: HashMap<Edge, usize> = HashMap::new();

    // TODO: we can cache bfs from a to each val
    for (i, akey) in keys.iter().enumerate() {
        for bkey in keys.iter().skip(i+1) {
            if distances.contains_key(&Edge::from(akey, bkey)) {
                continue;
            }
            // bfs
            let path = bfs(akey, bkey, components);

            for (d, paths) in path.iter().rev().enumerate() {
                // sorting for hash
                let v = Edge::from(paths.0, paths.1);

                distances.entry(v).and_modify(|x| {
                    *x += d+1;
                }).or_insert(d+1);
            }
        }
    }

    println!("got distances: {}", &distances.len());

    let mut output: Vec<_> = distances.into_iter().collect();
    
    // return the biggest, first three
    output.sort_by(|a, b| b.1.cmp(&a.1) );

    output.iter().take(3).map(|x| {
        return (x.0.0, x.0.1);
    }).collect::<Vec<(&str, &str)>>()
}

fn remove_and_count_groups(components: &Components, cuts: Vec<(&str, &str)>) -> usize {
    let mut clone = components.clone();

    for cut in cuts.iter() {
        clone.entry(cut.0).and_modify(|x| {
            x.retain(|y| *y != cut.1);
        });

        clone.entry(cut.1).and_modify(|x| {
            x.retain(|y| *y != cut.0);
        });
    }

    // now count nodes in one group
    let (one, _) = cuts.first().expect("we had a cut");

    let mut queue = vec![one];
    let mut seen = HashSet::new();

    while let Some(current) = queue.pop() {
        if seen.contains(current) {
            continue;
        }

        seen.insert(current);

        let links = clone.get(current).expect("current not in components");

        for link in links {
            if seen.contains(link) {
                continue;
            }
            queue.push(link);
        }
    }

    let group_a = seen.len();
    let group_b = clone.len() - group_a;

    return group_a * group_b;
}

fn part_one(components: &Components) -> usize {
    let cuts = bfs_everywhere(&components);
    
    remove_and_count_groups(&components, cuts)
}

fn part_two() -> usize {
    0
}

fn main() {
    let (one, two) = get_part();
    let start = Instant::now();
    let contents = fs::read_to_string("./src/input.txt").unwrap();

    let components = get_components(&contents);

    if one {
        let now = Instant::now();
        let ans = part_one(&components);
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
    fn test_cuts() {
        let components = get_components(EXAMPLE);
        let cuts = bfs_everywhere(&components);
        
        assert!(cuts.contains(&("jqt", "nvd")));
        assert!(cuts.contains(&("bvb", "cmg")));
        assert!(cuts.contains(&("hfx", "pzl")));
    }

    #[test]
    fn test_part_one() {
        let components = get_components(EXAMPLE);
        
        let ans = part_one(&components);

        assert_eq!(ans, 54);
    }

    #[test]
    fn test_part_two() {
        let ans = part_two();

        assert_eq!(ans, 0);
    }
}

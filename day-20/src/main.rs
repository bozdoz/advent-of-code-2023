use lib::get_part;
use std::{collections::HashMap, fs, time::Instant, vec, mem};

#[derive(Debug, Clone)]
enum ModuleType {
    FlipFlop(bool),
    Conjunction,
    Broadcast
}

#[derive(Debug, Clone, PartialEq)]
enum Pulse {
    High,
    Low,
    None,
}

#[derive(Debug, Clone)]
struct Module<'a> {
    receivers: Vec<&'a str>,
    destinations: Vec<&'a str>,
    variant: ModuleType,
    last_pulse: Pulse,
}

struct Config<'a> {
    modules: HashMap<&'a str, Module<'a>>,
}

impl<'a> Config<'a> {
    fn new(contents: &'a str) -> Self {
        let mut modules = HashMap::new();
        let mut keys_destinations = HashMap::new();
        // still not sure what this type is
        let symbols: &[_] = &['%', '&'];

        contents.lines().for_each(|line| {
            let (input, output) = line.split_once(" -> ").expect("an arrow");

            let destinations: Vec<&str> = output.split(", ").collect();
            // receivers will be unknown until we loop again afterward
            let receivers = vec![];
            let last_pulse = Pulse::None;

            keys_destinations.insert(input.trim_matches(symbols), destinations.clone());

            match input {
                "broadcaster" => {
                    modules.insert(
                        input,
                        Module {
                            destinations,
                            receivers,
                            last_pulse: Pulse::Low,
                            variant: ModuleType::Broadcast,
                        },
                    );
                },
                n if n.starts_with("%") => {
                    modules.insert(
                        &n[1..],
                        Module {
                            destinations,
                            receivers,
                            last_pulse,
                            variant: ModuleType::FlipFlop(false),
                        },
                    );
                },
                n if n.starts_with("&") => {
                    modules.insert(
                        &n[1..],
                        Module {
                            destinations,
                            receivers,
                            last_pulse,
                            variant: ModuleType::Conjunction,
                        },
                    );
                },
                _ => ()
            }
        });

        // TODO: gather linked modules
        // this is a pain
        for (key, destinations) in keys_destinations {
            for dest in destinations {
                if let Some(module) = modules.get_mut(dest) {
                    module.receivers.push(key);
                }
            }
        }

        Self { modules }
    }
    fn push_button(&self, time: usize) -> usize {
        // TODO: look for full circle
        let mut modules = self.modules.clone();
        // button sends low to broadcaster
        let mut low = time;
        let mut high = 0;
        let mut destinations: Vec<(Pulse, &str)> = vec![];
        let mut next_destinations: Vec<(Pulse, &str)> = vec![];

        for _ in 0..time {
            let broadcaster = modules.get("broadcaster").expect("broadcaster to exist");
            
            // broadcaster always sends low to destinations
            low += broadcaster.destinations.len();
            broadcaster.destinations.iter().for_each(|d| {
                next_destinations.push((Pulse::Low, d));
            });

            // go through destinations
            while next_destinations.len() > 0 {
                let mut next: Vec<(Pulse, &str)> = vec![];

                for (last_pulse, dest) in next_destinations {
                    let module = modules.get_mut(dest).expect("dest to exist");

                    match (last_pulse, &module.variant) {
                        (Pulse::Low, ModuleType::FlipFlop(onoff)) => {
                            // if a flip-flop module receives a low pulse, it flips 
                            // between on and off. If it was off, it turns on and 
                            // sends a high pulse. If it was on, it turns off and 
                            // sends a low pulse.
                            let toggled = !onoff;
                            module.variant = ModuleType::FlipFlop(toggled);

                            if toggled {
                                module.destinations.iter().for_each(|d| {
                                    next.push((Pulse::High, d));
                                });
                            } else {
                                module.destinations.iter().for_each(|d| {
                                    next.push((Pulse::Low, d));
                                });
                            }
                        }
                        (_, ModuleType::Conjunction) => {
                            // Then, if it remembers high pulses for all inputs, 
                            // it sends a low pulse; otherwise, it sends a high pulse.
                            // look up receivers
                            let all_high = module.receivers.iter().any(|r| {
                                let pulse = &modules.get(r).expect("receiver").last_pulse;

                                pulse == &Pulse::Low
                            });
                        }
                        _ => ()
                    }
                }

                next_destinations = next;
            }
        }

        low * high
    }
}

fn part_one(config: &Config) -> usize {
    config.push_button(1000)
}

fn part_two() -> usize {
    0
}

fn main() {
    let (one, two) = get_part();
    let start = Instant::now();
    let contents = fs::read_to_string("./src/input.txt").unwrap();

    let config = Config::new(&contents);

    if one {
        let now = Instant::now();
        let ans = part_one(&config);
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
        let config = Config::new(EXAMPLE);
        let ans = part_one(&config);

        assert_eq!(ans, 11687500);
    }

    #[test]
    fn test_part_two() {
        let ans = part_two();

        assert_eq!(ans, 0);
    }
}

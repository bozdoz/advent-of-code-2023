use lib::get_part;
use std::{ collections::HashMap, fs, time::Instant, vec };

#[derive(Debug, Clone)]
enum ModuleType {
    FlipFlop(bool),
    Conjunction,
    Broadcast,
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

#[derive(Debug)]
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

            keys_destinations.insert(
                input.trim_matches(symbols),
                destinations.clone()
            );

            match input {
                "broadcaster" => {
                    modules.insert(input, Module {
                        destinations,
                        receivers,
                        last_pulse: Pulse::Low,
                        variant: ModuleType::Broadcast,
                    });
                }
                n if n.starts_with("%") => {
                    modules.insert(&n[1..], Module {
                        destinations,
                        receivers,
                        last_pulse,
                        variant: ModuleType::FlipFlop(false),
                    });
                }
                n if n.starts_with("&") => {
                    modules.insert(&n[1..], Module {
                        destinations,
                        receivers,
                        last_pulse,
                        variant: ModuleType::Conjunction,
                    });
                }
                _ => unreachable!(),
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
        // button sends low to broadcaster, so low starts at `time`
        let mut low = time;
        let mut high = 0;
        let mut next_destinations: Vec<(Pulse, &str)> = vec![];

        for i in 0..time {
            let broadcaster = modules
                .get("broadcaster")
                .expect("broadcaster to exist");

            // broadcaster always sends low to destinations
            low += broadcaster.destinations.len();
            broadcaster.destinations.iter().for_each(|d| {
                next_destinations.push((Pulse::Low, d));
            });

            let mut rx_count = 0;

            // go through destinations
            while next_destinations.len() > 0 {
                let mut next: Vec<(Pulse, &str)> = vec![];

                for (last_pulse, dest) in next_destinations {
                    let module = modules.get(dest);

                    if module.is_none() {
                        rx_count += 1;
                        // example data has "output" as a destination
                        continue;
                    }

                    let module = module.unwrap();

                    match (last_pulse, &module.variant) {
                        (Pulse::Low, ModuleType::FlipFlop(onoff)) => {
                            // if a flip-flop module receives a low pulse, it flips
                            // between on and off. If it was off, it turns on and
                            // sends a high pulse. If it was on, it turns off and
                            // sends a low pulse.
                            let toggled = !onoff;

                            // ! TIL: I can get_mut when I need it, though this seems incredibly wasteful

                            let module = modules.get_mut(dest).unwrap();
                            module.variant = ModuleType::FlipFlop(toggled);
                            module.last_pulse = if toggled {
                                Pulse::High
                            } else {
                                Pulse::Low
                            };

                            if toggled {
                                high += module.destinations.len();
                                module.destinations.iter().for_each(|d| {
                                    next.push((Pulse::High, d));
                                });
                            } else {
                                low += module.destinations.len();
                                module.destinations.iter().for_each(|d| {
                                    next.push((Pulse::Low, d));
                                });
                            }
                        }
                        (_, ModuleType::Conjunction) => {
                            // they initially default to remembering a low pulse for each input
                            // Then, if it remembers high pulses for all inputs,
                            // it sends a low pulse; otherwise, it sends a high pulse.
                            // look up receivers
                            let all_high = &module.receivers.iter().all(|r| {
                                let pulse = &modules
                                    .get(r)
                                    .expect("receiver").last_pulse;

                                // None is equivalent to Low
                                pulse == &Pulse::High
                            });

                            let module = modules.get_mut(dest).unwrap();

                            if *all_high {
                                low += module.destinations.len();
                                module.last_pulse = Pulse::Low;
                                module.destinations.iter().for_each(|d| {
                                    next.push((Pulse::Low, d));
                                });
                            } else {
                                high += module.destinations.len();
                                module.last_pulse = Pulse::High;
                                module.destinations.iter().for_each(|d| {
                                    next.push((Pulse::High, d));
                                });
                            }
                        }
                        _ => {}
                    }
                }

                next_destinations = next;
            }

            if rx_count == 1 {
                return i;
            }
        }

        println!("LOW * HIGH");

        low * high
    }
}

fn part_one(config: &Config) -> usize {
    config.push_button(1000)
}

fn part_two(config: &Config) -> usize {
    config.push_button(1_000_000_000_000)
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
        let ans = part_two(&config);
        println!("Part two: {:?} {:?}", ans, now.elapsed());
    }

    println!("Time: {:?}", start.elapsed())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("./example.txt");

    #[test]
    fn test_single_press() {
        let config = Config::new(EXAMPLE);

        assert_eq!(config.push_button(1), 16);
    }

    #[test]
    fn test_part_one() {
        let config = Config::new(EXAMPLE);

        let ans = part_one(&config);

        assert_eq!(ans, 11687500);
    }

    const OTHER: &str =
        "broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a
";

    #[test]
    fn test_alt_example() {
        let config = Config::new(OTHER);

        assert_eq!(config.push_button(1), 8 * 4);
        assert_eq!(config.push_button(1000), 32000000);
    }
}

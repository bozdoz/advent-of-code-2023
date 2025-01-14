use lib::get_part;
use std::{ collections::HashMap, fs, time::Instant, usize, vec };

struct Stamp {
    map: HashMap<String, usize>,
    next: usize,
}

// from chatgpt
impl Stamp {
    fn new() -> Self {
        Self { map: HashMap::new(), next: 0 }
    }
    fn get(&self, input: &str) -> Option<usize> {
        self.map.get(input).and_then(|&x| Some(x))
    }

    fn stamp(&mut self, input: &str) -> usize {
        if let Some(&value) = self.map.get(input) {
            value
        } else {
            let value = self.next;
            self.map.insert(input.to_string(), value);
            self.next += 1;
            value
        }
    }
}

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
struct Module {
    receivers: Vec<usize>,
    destinations: Vec<usize>,
    variant: ModuleType,
    last_pulse: Pulse,
}

#[derive(Debug)]
struct Config {
    modules: Vec<Module>,
    output: usize,
    broadcaster: usize,
}

impl Config {
    fn new(contents: &str) -> Self {
        // convert str to u8
        let stamp = &mut Stamp::new();

        // rx never turns into a module, so we don't want the usize messing with the vec index
        stamp.map.insert("rx".to_string(), usize::MAX);

        let mut module_map = HashMap::new();
        let mut keys_destinations = HashMap::new();

        // still not sure what this type is
        let symbols: &[_] = &['%', '&'];

        contents.lines().for_each(|line| {
            let (input, output) = line.split_once(" -> ").expect("an arrow");

            let destinations: Vec<_> = output
                .split(", ")
                .map(|x| stamp.stamp(x))
                .collect();
            // receivers will be unknown until we loop again afterward
            let receivers = vec![];
            let last_pulse = Pulse::None;

            let key = stamp.stamp(input.trim_matches(symbols));

            keys_destinations.insert(key, destinations.clone());

            match input {
                "broadcaster" => {
                    module_map.insert(key, Module {
                        destinations,
                        receivers,
                        last_pulse: Pulse::Low,
                        variant: ModuleType::Broadcast,
                    });
                }
                n if n.starts_with("%") => {
                    module_map.insert(key, Module {
                        destinations,
                        receivers,
                        last_pulse,
                        variant: ModuleType::FlipFlop(false),
                    });
                }
                n if n.starts_with("&") => {
                    module_map.insert(key, Module {
                        destinations,
                        receivers,
                        last_pulse,
                        variant: ModuleType::Conjunction,
                    });
                }
                _ => unreachable!(),
            }
        });

        // this is a pain
        for (key, destinations) in keys_destinations {
            for dest in destinations {
                if let Some(module) = module_map.get_mut(&dest) {
                    module.receivers.push(key);
                }
            }
        }

        let modules = (0..)
            .map_while(|i| {
                if let Some(module) = module_map.get(&i) {
                    // crazy
                    Some(module.clone())
                } else {
                    None
                }
            })
            .collect();

        // rx might not exist
        let unknown = stamp.stamp("UNKNOWN");
        let output = *stamp.map
            .get("output")
            .or(stamp.map.get("rx"))
            .unwrap_or(&unknown);

        Self {
            modules,
            output,
            broadcaster: stamp.get("broadcaster").expect("broadcaster"),
        }
    }

    fn push_button(&self, time: usize) -> usize {
        let mut modules = self.modules.clone();

        let broadcaster = &modules[self.broadcaster];

        // button sends low to broadcaster, so low starts at `time`, plus all the
        // pulses broadcaster *will* send
        let mut low = time + broadcaster.destinations.len() * time;
        let mut high = 0;
        // broadcaster always sends low to destinations
        let broadcasts = broadcaster.destinations
            .iter()
            .map(|&d| { (Pulse::Low, d) })
            .collect::<Vec<_>>();

        for _ in 0..time {
            let mut next_destinations = broadcasts.clone();

            // go through destinations
            while next_destinations.len() > 0 {
                let mut next: Vec<(Pulse, usize)> = vec![];

                for (last_pulse, dest) in next_destinations.iter() {
                    if *dest == self.output {
                        // example data has "output" as a destination
                        continue;
                    }

                    let module = &modules[*dest];

                    match (last_pulse, &module.variant) {
                        (Pulse::Low, ModuleType::FlipFlop(onoff)) => {
                            // if a flip-flop module receives a low pulse, it flips
                            // between on and off. If it was off, it turns on and
                            // sends a high pulse. If it was on, it turns off and
                            // sends a low pulse.
                            let toggled = !onoff;

                            // ! TIL: I can get_mut when I need it, though this seems incredibly wasteful

                            let module = modules.get_mut(*dest).unwrap();
                            module.variant = ModuleType::FlipFlop(toggled);
                            module.last_pulse = if toggled {
                                Pulse::High
                            } else {
                                Pulse::Low
                            };

                            if toggled {
                                high += module.destinations.len();
                                next.extend(
                                    module.destinations
                                        .iter()
                                        .map(|&d| { (Pulse::High, d) })
                                );
                            } else {
                                low += module.destinations.len();
                                next.extend(
                                    module.destinations
                                        .iter()
                                        .map(|&d| { (Pulse::Low, d) })
                                );
                            }
                        }
                        (_, ModuleType::Conjunction) => {
                            // they initially default to remembering a low pulse for each input
                            // Then, if it remembers high pulses for all inputs,
                            // it sends a low pulse; otherwise, it sends a high pulse.
                            // look up receivers
                            let all_high = &module.receivers.iter().all(|&r| {
                                // None is equivalent to Low
                                &modules[r].last_pulse == &Pulse::High
                            });

                            let module = modules.get_mut(*dest).unwrap();

                            if *all_high {
                                low += module.destinations.len();
                                module.last_pulse = Pulse::Low;
                                next.extend(
                                    module.destinations
                                        .iter()
                                        .map(|&d| { (Pulse::Low, d) })
                                );
                            } else {
                                high += module.destinations.len();
                                module.last_pulse = Pulse::High;
                                next.extend(
                                    module.destinations
                                        .iter()
                                        .map(|&d| { (Pulse::High, d) })
                                );
                            }
                        }
                        _ => {}
                    }
                }

                // is mem swap faster?
                std::mem::swap(&mut next_destinations, &mut next);
            }
        }

        low * high
    }

    fn get_cycles_for_rx(&self) -> [usize; 4] {
        let mut modules = self.modules.clone();

        let broadcaster = &modules[self.broadcaster];

        // broadcaster always sends low to destinations
        let broadcasts = broadcaster.destinations
            .iter()
            .map(|&d| { (Pulse::Low, d) })
            .collect::<Vec<_>>();

        for _ in 0.. {
            let mut next_destinations = broadcasts.clone();

            // go through destinations
            while next_destinations.len() > 0 {
                let mut next: Vec<(Pulse, usize)> = vec![];

                for (last_pulse, dest) in next_destinations.iter() {
                    if *dest == self.output {
                        // example data has "output" as a destination
                        continue;
                    }

                    let module = &modules[*dest];

                    match (last_pulse, &module.variant) {
                        (Pulse::Low, ModuleType::FlipFlop(onoff)) => {
                            // if a flip-flop module receives a low pulse, it flips
                            // between on and off. If it was off, it turns on and
                            // sends a high pulse. If it was on, it turns off and
                            // sends a low pulse.
                            let toggled = !onoff;

                            // ! TIL: I can get_mut when I need it, though this seems incredibly wasteful

                            let module = modules.get_mut(*dest).unwrap();
                            module.variant = ModuleType::FlipFlop(toggled);
                            module.last_pulse = if toggled {
                                Pulse::High
                            } else {
                                Pulse::Low
                            };

                            if toggled {
                                next.extend(
                                    module.destinations
                                        .iter()
                                        .map(|&d| { (Pulse::High, d) })
                                );
                            } else {
                                next.extend(
                                    module.destinations
                                        .iter()
                                        .map(|&d| { (Pulse::Low, d) })
                                );
                            }
                        }
                        (_, ModuleType::Conjunction) => {
                            // they initially default to remembering a low pulse for each input
                            // Then, if it remembers high pulses for all inputs,
                            // it sends a low pulse; otherwise, it sends a high pulse.
                            // look up receivers
                            let all_high = &module.receivers.iter().all(|&r| {
                                // None is equivalent to Low
                                &modules[r].last_pulse == &Pulse::High
                            });

                            let module = modules.get_mut(*dest).unwrap();

                            if *all_high {
                                module.last_pulse = Pulse::Low;
                                next.extend(
                                    module.destinations
                                        .iter()
                                        .map(|&d| { (Pulse::Low, d) })
                                );
                            } else {
                                module.last_pulse = Pulse::High;
                                next.extend(
                                    module.destinations
                                        .iter()
                                        .map(|&d| { (Pulse::High, d) })
                                );
                            }
                        }
                        _ => {}
                    }
                }

                // is mem swap faster?
                std::mem::swap(&mut next_destinations, &mut next);
            }
        }

        [1, 1, 1, 1]
    }
}

fn part_one(config: &Config) -> usize {
    config.push_button(1000)
}

fn part_two(config: &Config) -> usize {
    let vals = config.get_cycles_for_rx();

    vals.iter().product()
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
    fn test_alt_once() {
        let config = Config::new(OTHER);

        assert_eq!(config.push_button(1), 8 * 4);
    }

    #[test]
    fn test_alt_one_thousand() {
        let config = Config::new(OTHER);

        assert_eq!(config.push_button(1000), 32000000);
    }
}

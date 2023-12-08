use std::{ time::Instant, fs, collections::HashMap, cmp::Ordering };
use lib::get_part;

// reverse order, because that's how I can compare via FullHouse > Three
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum CamelType {
    HighCard(Vec<u8>),
    Pair(Vec<u8>),
    TwoPair(Vec<u8>),
    Three(Vec<u8>),
    FullHouse(Vec<u8>),
    Four(Vec<u8>),
    Five(Vec<u8>),
}

#[derive(Debug, PartialEq, Eq, PartialOrd)]
struct CamelHand {
    hand: CamelType,
    bid: usize,
}

impl Ord for CamelHand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // I can't believe I have to do this
        match (&self.hand, &other.hand) {
            | (CamelType::HighCard(a), CamelType::HighCard(b))
            | (CamelType::Pair(a), CamelType::Pair(b))
            | (CamelType::TwoPair(a), CamelType::TwoPair(b))
            | (CamelType::Three(a), CamelType::Three(b))
            | (CamelType::FullHouse(a), CamelType::FullHouse(b))
            | (CamelType::Four(a), CamelType::Four(b))
            | (CamelType::Five(a), CamelType::Five(b)) => {
                // check the cards (vec<u8> has `.cmp()`)
                return a.cmp(&b);
            }
            _ => {
                if self.hand < other.hand {
                    return Ordering::Less;
                }
                
                if self.hand > other.hand {
                    return Ordering::Greater;
                }
                return Ordering::Equal;
            }
        }
    }
}

impl CamelHand {
    fn new(data: (&str, &str), part: u8) -> Self {
        CamelHand {
            hand: Self::hand_to_category(data.0, part),
            bid: data.1.parse().unwrap(),
        }
    }

    fn hand_to_category(hand: &str, part: u8) -> CamelType {
        Self::categorize_digits(Self::hand_to_digits(hand, part), part)
    }

    fn categorize_digits(hand: Vec<u8>, part: u8) -> CamelType {
        let mut counts: HashMap<&u8, i32> = HashMap::new();

        for card in hand.iter() {
            if let Some(x) = counts.get_mut(card) {
                *x += 1;
            } else {
                counts.insert(card, 1);
            }
        }

        // need to count 'J' in part 2, before categorizing
        if part == 2 {
            Self::alter_counts(&mut counts);
        }

        // deduce the hand from the count groups
        match counts.len() {
            1 => { CamelType::Five(hand) }
            2 => {
                if counts.values().any(|x| *x == 2) {
                    return CamelType::FullHouse(hand);
                }
                CamelType::Four(hand)
            }
            3 => {
                if counts.values().any(|x| *x == 2) {
                    return CamelType::TwoPair(hand);
                }
                CamelType::Three(hand)
            }
            4 => { CamelType::Pair(hand) }
            _ => { CamelType::HighCard(hand) }
        }
    }

    fn alter_counts(counts: &mut HashMap<&u8, i32>) {
        if let Some(joker) = counts.get(&1) {
            // found a joker
            // these are crazy types
            let mut max: (&&u8, &i32) = (&&0u8, &0i32);
            for count in counts.iter() {
                if count.0 == &&1u8 {
                    // don't count jokers 
                    continue;
                }
                if count.1 > &max.1 {
                    max = count;
                }
            }

            // give all jokers to the max
            counts.insert(max.0, max.1 + joker);
            counts.remove(&1);
        }
    }

    fn hand_to_digits(hand: &str, part: u8) -> Vec<u8> {
        hand.chars()
            .map(|c| {
                match c {
                    'A' => { 14 }
                    'K' => { 13 }
                    'Q' => { 12 }
                    'J' => { if part == 1 { 11 } else { 1 } }
                    'T' => { 10 }
                    n => n.to_digit(10).unwrap() as u8,
                }
            })
            .collect()
    }

    fn hands(contents: &str, part: u8) -> Vec<Self> {
        contents
            .lines()
            .map(|l| { Self::new(l.split_once(" ").unwrap(), part) })
            .collect()
    }
}

fn part_one(hands: &mut Vec<CamelHand>) -> usize {
    // this really is the magic?
    hands.sort();

    let mut winnings = 0;

    for (i, hand) in hands.iter().enumerate() {
        winnings += hand.bid * (i+1);
    }

    winnings
}

fn part_two(hands: &mut Vec<CamelHand>) -> usize {
    // this feels like a code smell
    part_one(hands)
}

fn main() {
    let (one, two) = get_part();
    let start = Instant::now();
    let contents = fs::read_to_string("./src/input.txt").unwrap();

    
    if one {
        let now = Instant::now();
    
        let mut hands = CamelHand::hands(contents.as_str(), 1);
        
        let ans = part_one(&mut hands);    
        
        println!("Part one: {:?} {:?}", ans, now.elapsed());
    }
    
    if two {
        let now = Instant::now();
        let mut hands = CamelHand::hands(contents.as_str(), 2);
        let ans = part_two(&mut hands);
        println!("Part two: {:?} {:?}", ans, now.elapsed());
    }

    println!("Time: {:?}", start.elapsed())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("./example.txt");

    #[test]
    fn test_hand_digits() {
        assert_eq!(CamelHand::hand_to_digits("32T3K", 1), [3, 2, 10, 3, 13]);
    }

    #[test]
    fn test_hand_category() {
        assert!(
            matches!(CamelHand::hand_to_category("32T3K", 1), CamelType::Pair(_))
        );
        assert!(
            matches!(CamelHand::hand_to_category("11111", 1), CamelType::Five(_))
        );
        assert!(
            matches!(CamelHand::hand_to_category("21111", 1), CamelType::Four(_))
        );
        assert!(
            matches!(CamelHand::hand_to_category("23111", 1), CamelType::Three(_))
        );
        assert!(
            matches!(
                CamelHand::hand_to_category("22111", 1),
                CamelType::FullHouse(_)
            )
        );
        assert!(
            matches!(
                CamelHand::hand_to_category("32211", 1),
                CamelType::TwoPair(_)
            )
        );
        assert!(
            matches!(CamelHand::hand_to_category("23411", 1), CamelType::Pair(_))
        );
        assert!(
            matches!(
                CamelHand::hand_to_category("12345", 1),
                CamelType::HighCard(_)
            )
        );
    }

    #[test]
    fn test_cmp() {
        let hand1: CamelHand = CamelHand::new(("KK111", "0"), 1);
        let hand2: CamelHand = CamelHand::new(("KKK21", "0"), 1);

        // not sure why we have so many gt functions
        assert!(hand1 > hand2);
        assert!(hand1.gt(&hand2));
        assert!(hand1.cmp(&hand2) == Ordering::Greater);
    }

    #[test]
    fn test_sort() {
        let hand1 = CamelHand::new(("12345", "0"), 1);
        let hand2 = CamelHand::new(("11234", "0"), 1);
        let hand3 = CamelHand::new(("11112", "0"), 1);
        let hand4 = CamelHand::new(("11111", "0"), 1);

        let mut hands = [&hand4, &hand2, &hand1, &hand3];

        hands.sort();

        assert!(hands[0] == &hand1);
    }

    #[test]
    fn test_first_best() {
        let hand1 = CamelHand::new(("11633", "0"), 1);
        let hand2 = CamelHand::new(("11522", "0"), 1);

        let mut hands = [&hand1, &hand2];

        hands.sort();

        assert!(hands[0] == &hand2);
    }

    #[test]
    fn test_part_one() {
        let mut hands = CamelHand::hands(EXAMPLE, 1);

        let ans = part_one(&mut hands);

        assert_eq!(ans, 6440);
    }
    
    #[test]
    fn test_alter_counts() {
        assert!(
            matches!(CamelHand::hand_to_category("33J22", 2), CamelType::FullHouse(_))
        );
        assert!(
            matches!(CamelHand::hand_to_category("KTJJT", 2), CamelType::Four(_))
        );
    }

    #[test]
    fn test_part_two() {
        let mut hands = CamelHand::hands(EXAMPLE, 2);

        let ans = part_two(&mut hands);

        assert_eq!(ans, 5905);
    }
}

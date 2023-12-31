use std::collections::HashMap;

const fn decode_char_joker(x: char) -> u8 {
    match x {
        'A' => 15,
        'K' => 14,
        'Q' => 13,
        'J' => 0,
        'T' => 11,
        '9' => 9,
        '8' => 8,
        '7' => 7,
        '6' => 6,
        '5' => 5,
        '4' => 4,
        '3' => 3,
        _ => 2,
    }
}
const fn decode_char(x: char) -> u8 {
    match x {
        'A' => 15,
        'K' => 14,
        'Q' => 13,
        'J' => 12,
        'T' => 11,
        '9' => 9,
        '8' => 8,
        '7' => 7,
        '6' => 6,
        '5' => 5,
        '4' => 4,
        '3' => 3,
        _ => 2,
    }
}

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord, Copy)]
enum HandKind {
    FiveOfAKind = 6,
    FourOfAKind = 5,
    FullHouse = 4,
    ThreeOfAKind = 3,
    TwoPair = 2,
    OnePair = 1,
    HighCard = 0,
}
struct Results {
    hand: String,
    bid: i64,
    kind: HandKind,
}

impl Ord for Results {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.kind == other.kind {
            println!("{:?} == {:?}", self.kind, other.kind);
        }
        return self.kind.cmp(&other.kind);
    }
}
impl Eq for Results {}
impl PartialEq for Results {
    fn eq(&self, other: &Self) -> bool {
        self.kind.eq(&other.kind)
    }
}

impl PartialOrd for Results {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.kind == other.kind {
            let chars = self.hand.chars();
            let mut other_chars = other.hand.chars();
            for c in chars {
                let next_char = other_chars.next().unwrap();
                if next_char != c {
                    let v = decode_char(c);
                    let v2 = decode_char(next_char);
                    return Some(v.cmp(&v2));
                }
            }
            return Some(core::cmp::Ordering::Equal);
        }
        Some(self.kind.cmp(&other.kind))
    }
}

struct ResultsPart2 {
    hand: String,
    bid: i64,
    kind: HandKind,
}

impl Ord for ResultsPart2 {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.kind == other.kind {
            println!("{:?} == {:?}", self.kind, other.kind);
        }
        return self.kind.cmp(&other.kind);
    }
}
impl Eq for ResultsPart2 {}
impl PartialEq for ResultsPart2 {
    fn eq(&self, other: &Self) -> bool {
        self.kind.eq(&other.kind)
    }
}

impl PartialOrd for ResultsPart2 {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.kind == other.kind {
            let chars = self.hand.chars();
            let mut other_chars = other.hand.chars();
            for c in chars {
                let next_char = other_chars.next().unwrap();
                if next_char != c {
                    let v = decode_char_joker(c);
                    let v2 = decode_char_joker(next_char);
                    return Some(v.cmp(&v2));
                }
            }
            return Some(core::cmp::Ordering::Equal);
        }
        Some(self.kind.cmp(&other.kind))
    }
}

#[derive(Debug, Clone)]
struct Game {
    hand: String,
    bid: i64,
}
fn main() {
    let input = include_str!("./day_07_input.txt");
    println!("Day 07 Part 01 {:?}", day_07_01(input));
    println!("Day 07 Part 02 {:?}", day_07_02(input));
}

fn parse_input_part_01(input: &str) -> Vec<Game> {
    input
        .lines()
        .map(|line| {
            let values = line.split_whitespace().collect::<Vec<&str>>();

            Game {
                hand: values.get(0).unwrap().to_string(),
                bid: values.get(1).unwrap().parse::<i64>().unwrap(),
            }
        })
        .collect()
}

fn get_hand_type(hand: HashMap<i64, Vec<char>>) -> HandKind {
    let kind: HandKind;

    if hand.get(&5).is_some() {
        kind = HandKind::FiveOfAKind;
    } else if hand.get(&4).is_some() {
        kind = HandKind::FourOfAKind;
    } else if hand.get(&3).is_some() && hand.get(&2).is_some() {
        kind = HandKind::FullHouse;
    } else if hand.get(&3).is_some() {
        kind = HandKind::ThreeOfAKind;
    } else if let Some(pair) = hand.get(&2) {
        if pair.len() > 1 {
            kind = HandKind::TwoPair;
        } else {
            kind = HandKind::OnePair;
        }
    } else {
        kind = HandKind::HighCard;
    }

    kind
}
fn group_hand(game: &Game) -> HashMap<char, i64> {
    game.hand.chars().fold(HashMap::new(), |mut acc, c| {
        if !acc.contains_key(&c) {
            acc.insert(c, 1);
        } else {
            let a = acc.get_mut(&c).unwrap();
            *a += 1;
        }
        acc
    })
}
fn inverse_map<T, D>(hand: &HashMap<T, D>) -> HashMap<D, Vec<T>>
where
    D: Eq + std::hash::Hash + Clone,
    T: Clone,
{
    let mut g: HashMap<D, Vec<T>> = HashMap::new();

    for (k, v) in hand {
        g.entry(v.clone()).or_insert_with(Vec::new).push(k.clone())
    }
    g
}

fn get_total_winnings_part2(mut results: Vec<ResultsPart2>) -> i64 {
    results.sort();
    let mut t: i64 = 0;
    for (idx, a) in results.iter().enumerate() {
        t += (idx + 1) as i64 * a.bid;
    }
    t
}

fn get_total_winnings(mut results: Vec<Results>) -> i64 {
    results.sort();
    let mut t: i64 = 0;
    for (idx, a) in results.iter().enumerate() {
        t += (idx + 1) as i64 * a.bid;
    }
    t
}

pub fn day_07_01(input: &str) -> String {
    let games = parse_input_part_01(input);
    let mut results: Vec<Results> = Vec::new();
    for game in games {
        let hand = group_hand(&game);
        let inversed_hand = inverse_map(&hand);
        results.push(Results {
            hand: game.hand,
            bid: game.bid,
            kind: get_hand_type(inversed_hand),
        });
    }

    format!("{}", get_total_winnings(results))
}

fn try_jokers(
    hand_without_jokers: &HashMap<i64, Vec<char>>,
    number_of_jokers: i64,
) -> HashMap<i64, Vec<char>> {
    let mut result = hand_without_jokers.clone();
    let mut remainning_jokers = number_of_jokers;
    while remainning_jokers > 0 {
        let hand_kind = get_hand_type(result.clone());
        match hand_kind {
            HandKind::FiveOfAKind => {}
            HandKind::FourOfAKind => {
                if let Some(four_of_a_kind) = result.get(&4) {
                    //we can have only 1 so we increase it
                    result.insert(5, four_of_a_kind.clone());
                    result.remove(&4);
                }
            }
            HandKind::FullHouse | HandKind::ThreeOfAKind => {
                // convert to Four of Kind
                let three = result.get(&3).unwrap();
                result.insert(4, three.clone());
                result.remove(&3);
            }
            HandKind::TwoPair => {
                //convert to full house
                let pairs = result.get_mut(&2).unwrap();
                let val = pairs.get(0).unwrap().clone();
                pairs.remove(1);
                result.insert(3, vec![val]);
            }
            HandKind::OnePair => {
                // convert to Three of a kind
                let pair = result.get(&2).unwrap();
                result.insert(3, pair.clone());
                result.remove(&2);
            }
            HandKind::HighCard => {
                // convert one to pair
                if let Some(h) = result.get_mut(&1) {
                    let val = h.get(0).unwrap().clone();
                    if h.len() > 1 {
                        h.remove(1);
                    }
                    result.insert(2, vec![val]);
                } else {
                    result.insert(1, vec!['2']);
                }
            }
        }
        remainning_jokers -= 1;
    }
    result
}
pub fn day_07_02(input: &str) -> String {
    let games = parse_input_part_01(input);
    let mut results: Vec<ResultsPart2> = Vec::new();
    for game in games {
        let mut hand = group_hand(&game);
        let jokers = hand.get(&'J');
        let mut inversed_hand: HashMap<i64, Vec<char>>;

        if jokers.is_some() {
            let number_of_jokers = jokers.unwrap().clone();
            hand.remove(&'J');
            inversed_hand = inverse_map(&hand);

            inversed_hand = try_jokers(&inversed_hand, number_of_jokers);
        } else {
            inversed_hand = inverse_map(&hand);
        }

        results.push(ResultsPart2 {
            hand: game.hand,
            bid: game.bid,
            kind: get_hand_type(inversed_hand),
        });
    }

    format!("{}", get_total_winnings_part2(results))
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day_07_part_01() {
        let input = include_str!("./day_07_sample.txt");
        assert_eq!(String::from("6440"), day_07_01(input));
    }
    #[test]
    fn day_07_part_02() {
        let input = include_str!("./day_07_sample.txt");
        assert_eq!(String::from("5905"), day_07_02(input));
    }
}

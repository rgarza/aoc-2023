use std::collections::HashMap;

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

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
struct Game {
    hand: String,
    bid: i64,
}
fn main() {
    let input = include_str!("./day_07_input.txt");
    println!("Day 07 Part 01 {:?}", day_07_01(input));
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

pub fn day_07_01(input: &str) -> String {
    let games = parse_input_part_01(input);
    let mut results: Vec<Results> = Vec::new();
    for game in games {
        let grouped_hand = game.hand.chars().fold(HashMap::new(), |mut acc, c| {
            if !acc.contains_key(&c) {
                acc.insert(c, 1);
            } else {
                let a = acc.get_mut(&c).unwrap();
                *a += 1;
            }

            acc
        });
        let mut g = HashMap::new();

        for (k, v) in grouped_hand {
            g.entry(v).or_insert_with(Vec::new).push(k)
        }
        let kind: HandKind;
        if g.get(&5).is_some() {
            kind = HandKind::FiveOfAKind;
        } else if g.get(&4).is_some() {
            kind = HandKind::FourOfAKind;
        } else if g.get(&3).is_some() && g.get(&2).is_some() {
            kind = HandKind::FullHouse;
        } else if g.get(&3).is_some() {
            kind = HandKind::ThreeOfAKind;
        } else if let Some(pair) = g.get(&2) {
            if pair.len() > 1 {
                kind = HandKind::TwoPair;
            } else {
                kind = HandKind::OnePair;
            }
        } else {
            kind = HandKind::HighCard;
        }
        results.push(Results {
            hand: game.hand,
            bid: game.bid,
            kind,
        });
    }
    results.sort();
    let mut t: i64 = 0;
    for (idx, a) in results.iter().enumerate() {
        println!(
            "{:?} * {:?} = {:?}",
            a.bid,
            idx + 1,
            a.bid * (idx + 1) as i64
        );
        t += (idx + 1) as i64 * a.bid;
    }
    format!("{}", t)
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day_07_part_01() {
        let input = include_str!("./day_07_sample.txt");
        assert_eq!(String::from("6440"), day_07_01(input));
    }
}

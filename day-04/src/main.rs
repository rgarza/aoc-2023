use std::collections::{HashMap, HashSet};

fn main() {
    let input = include_str!("./day_04_part_01.txt");
    println!("{}", day_04_01(input));

    println!("{}", day_04_02(input));
}

fn numbers_to_hashset(input: &str) -> HashSet<&str> {
    let mut set: HashSet<&str> = HashSet::new();

    input.trim().split(" ").filter(|f| f != &"").for_each(|e| {
        set.insert(e.trim());
    });
    set
}

pub fn day_04_01(input: &str) -> String {
    let mut sum: i64 = 0;
    input.lines().for_each(|line| {
        let parts = line.split(":").collect::<Vec<&str>>();
        println!("{:?}", parts.get(0).unwrap());
        let numbers = parts.get(1).unwrap().split("|").collect::<Vec<&str>>();

        let winners = numbers_to_hashset(numbers.get(0).unwrap());
        println!("Winner numbers {:?}", winners);

        let my_numbers = numbers_to_hashset(numbers.get(1).unwrap());
        println!("My numbers {:?}", my_numbers);
        let mut card_total: i64 = 0;
        for ele in my_numbers.iter() {
            if winners.contains(ele) {
                if card_total == 0 {
                    card_total = 1;
                } else {
                    card_total += card_total;
                }
            }
        }
        println!("Card total {:?}", card_total);

        sum += card_total;
    });
    format!("{:?}", sum).to_string()
}

pub fn day_04_02(input: &str) -> String {
    let mut sum: i64 = 0;
    let mut number_of_cards: HashMap<i64, i64> = HashMap::new();

    input.lines().for_each(|line| {
        let parts = line.split(":").collect::<Vec<&str>>();
        let card_number: &str = parts
            .get(0)
            .unwrap()
            .split(" ")
            .filter(|f| f != &"")
            .collect::<Vec<&str>>()
            .get(1)
            .unwrap();

        let card_as_number: i64 = card_number.trim().parse::<i64>().unwrap();
        println!("card_number {:?}", card_number);

        let numbers = parts.get(1).unwrap().split("|").collect::<Vec<&str>>();

        let winners = numbers_to_hashset(numbers.get(0).unwrap());
        println!("Winner numbers {:?}", winners);

        let my_numbers = numbers_to_hashset(numbers.get(1).unwrap());

        println!("My numbers {:?}", my_numbers);
        let mut card_total: i64 = 0;
        for ele in my_numbers.iter() {
            if winners.contains(ele) {
                card_total += 1;
            }
        }

        if !number_of_cards.contains_key(&card_as_number) {
            number_of_cards.insert(card_as_number, 1);
        } else {
            let card = number_of_cards.get_mut(&card_as_number).unwrap();
            *card += 1;
        }
        let n = number_of_cards.get(&card_as_number).unwrap().clone();
        for i in (card_as_number + 1)..(card_as_number + 1 + card_total) {
            println!("Card {}, gives another to {:?}", card_number, i);
            if !number_of_cards.contains_key(&i) {
                number_of_cards.insert(i, 1 * n);
            } else {
                let card = number_of_cards.get_mut(&i).unwrap();
                *card += n;
            }
        }
    });
    println!("number of cards {:?}", number_of_cards);
    for (_k, v) in number_of_cards.iter() {
        sum += v;
    }
    format!("{:?}", sum).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day_04_part_01() {
        let input = include_str!("./day_01_sample.txt");
        let expected: String = "13".to_string();
        assert_eq!(expected, day_04_01(input));
    }

    #[test]
    fn day_04_part_02() {
        let input = include_str!("./day_01_sample.txt");
        let expected: String = "30".to_string();
        assert_eq!(expected, day_04_02(input));
    }
}

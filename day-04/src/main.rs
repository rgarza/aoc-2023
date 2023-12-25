use std::collections::HashSet;

fn main() {
    let input = include_str!("./day_04_part_01.txt");
    println!("{}", day_04_01(input));
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day_04_part_01() {
        let input = include_str!("./day_01_sample.txt");
        let expected: String = "13".to_string();
        assert_eq!(expected, day_04_01(input));
    }
}

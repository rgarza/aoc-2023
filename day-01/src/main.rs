pub fn day01(input: &str) -> String {
    let sum: u64 = input
        .lines()
        .map(|line| {
            let l = Some(line);
            let mut first: Option<char> = None;
            let mut last: Option<char> = None;
            l.unwrap().chars().for_each(|ele| {
                if ele.is_digit(10) {
                    if first.is_none() {
                        first = Some(ele);
                    }
                    last = Some(ele);
                }
            });
            format!("{}{}", first.unwrap(), last.unwrap())
                .parse::<u64>()
                .unwrap()
        })
        .sum();
    sum.to_string()
}

pub fn day01_02(input: &str) -> String {
    let vec_words = vec![
        "one".to_string(),
        "two".to_string(),
        "three".to_string(),
        "four".to_string(),
        "five".to_string(),
        "six".to_string(),
        "seven".to_string(),
        "eight".to_string(),
        "nine".to_string(),
    ];
    let sum: u64 = input
        .lines()
        .map(|line| {
            let l = Some(line);
            let mut first: Option<char> = None;
            let mut last: Option<char> = None;
            let mut current_words: Vec<String> = vec![];
            l.unwrap().chars().for_each(|ele| {
                if ele.is_digit(10) {
                    current_words.clear();
                    if first.is_none() {
                        first = Some(ele);
                    }
                    last = Some(ele);
                } else {
                    for i in (0..current_words.len()).rev() {
                        current_words[i].push_str(&ele.to_string());

                        let found = vec_words.contains(&current_words[i]);

                        if found {
                            let index = vec_words
                                .iter()
                                .position(|value| value == &current_words[i])
                                .unwrap() as u32;
                            current_words.remove(i);
                            let c = char::from_digit(index + 1, 10);
                            if first.is_none() {
                                first = c;
                            }
                            last = c;
                        } else {
                            // check if word starts with that
                            // if not remove
                            let index = vec_words
                                .iter()
                                .position(|value| value.starts_with(&current_words[i]));

                            if index.is_none() {
                                // nothing starts with this remove
                                current_words.remove(i);
                            }
                        }
                    }
                    current_words.push(ele.to_string());
                }
            });
            let v = format!("{}{}", first.unwrap(), last.unwrap())
                .parse::<u64>()
                .unwrap();
            v
        })
        .sum();
    sum.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn day_01() {
        let input = include_str!("./input_day_01_sample.txt");
        let expected = String::from("142");
        assert_eq!(expected, day01(input));
    }
    #[test]
    fn day_01_02_test() {
        let input = include_str!("./input_day_01_part_02_sample.txt");
        let expected = String::from("281");
        assert_eq!(expected, day01_02(input));
    }
    #[test]
    fn day_01_02_real_test() {
        let input = include_str!("./input_day_01_02.txt");
        let expected = String::from("54087");
        assert_eq!(expected, day01_02(input));
    }
}

fn main() {
    let input = include_str!("./input_day_01.txt");
    println!("day 01 part 01 Answer {:?}", day01(input));
    let input_02 = include_str!("./input_day_01_02.txt");
    println!("day 01 part 02 Answer {:?}", day01_02(input_02));
}

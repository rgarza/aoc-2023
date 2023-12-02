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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day_01() {
        let input = include_str!("./input_day_01_sample.txt");
        let expected = String::from("142");
        assert_eq!(expected, day01(input));
    }
}

fn main() {
    let input = include_str!("./input_day_01.txt");
    println!("{:?}", day01(input));
}

fn main() {
    let input = include_str!("./day_09_input.txt");
    println!("Day 09 part 01 {}", day_09_01(input));
}

fn process_line(values: &Vec<i64>, zeroes: i64) -> i64 {
    let mut new_vec: Vec<i64> = Vec::new();

    if zeroes == values.len() as i64 {
        return 0;
    }
    let mut number_of_zeros: i64 = 0;
    for i in 1..values.len() {
        let current = values.get(i).unwrap();
        let prev = values.get(i - 1).unwrap();
        if (current - prev) == 0 {
            number_of_zeros += 1;
        }
        new_vec.push(current - prev);
    }

    process_line(&new_vec, number_of_zeros) + values.last().unwrap()
}
pub fn day_09_01(input: &str) -> String {
    let sum: i64 = input
        .lines()
        .map(|line| {
            let values: Vec<i64> = line
                .split_whitespace()
                .map(|m| m.parse::<i64>().unwrap())
                .collect::<Vec<i64>>();
            process_line(&values, 0)
        })
        .sum();
    format!("{}", sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day_09_part_01() {
        let input = include_str!("./day_09_sample.txt");
        assert_eq!(String::from("114"), day_09_01(input));
    }
}

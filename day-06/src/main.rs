fn main() {
    let input = include_str!("./day_06_input.txt");
    println!("{}", day_06_01(input));
}
#[derive(Debug, Clone)]
struct Race {
    time: i64,
    record: i64,
}
fn parse_input(input: &str) -> Vec<Race> {
    let mut values: Vec<Vec<i64>> = Vec::new();
    let mut result: Vec<Race> = Vec::new();

    input.lines().for_each(|line| {
        let numbers = line.split(":").collect::<Vec<&str>>();
        let a: Vec<i64> = numbers
            .get(1)
            .unwrap()
            .split_whitespace()
            .filter(|f| f != &"")
            .map(|m| m.parse().unwrap())
            .collect();
        values.push(a);
    });
    for i in 0..values.get(0).unwrap().len() {
        let time = values.get(0).unwrap();

        let record = values.get(1).unwrap();
        result.push(Race {
            time: *time.get(i).unwrap(),
            record: *record.get(i).unwrap(),
        });
    }

    println!("Races {:?}", result);
    result
}
pub fn day_06_01(input: &str) -> String {
    let races = parse_input(input);
    let r: i64 = races
        .iter()
        .map(|race| {
            let mut c: i64 = 0;
            for i in 0..race.time {
                let remaining_time = race.time - i;
                let distance = remaining_time * i;
                if distance > race.record {
                    c += 1;
                }
            }
            c
        })
        .product();
    format!("{}", r)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn day_06_part_01() {
        let input = include_str!("./day_06_sample.txt");
        assert_eq!(String::from("288"), day_06_01(input));
    }
}

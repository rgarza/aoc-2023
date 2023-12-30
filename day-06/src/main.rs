fn main() {
    let input = include_str!("./day_06_input.txt");
    println!("{}", day_06_01(input));

    println!("{}", day_06_02(input));
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

fn parse_input_day_02(input: &str) -> Race {
    let mut values: Vec<i64> = Vec::new();
    let mut race: Race = Race { time: 0, record: 0 };

    input.lines().for_each(|line| {
        let numbers = line.split(":").collect::<Vec<&str>>();
        let v = numbers
            .get(1)
            .unwrap()
            .split_whitespace()
            .filter(|f| f != &"")
            .collect::<Vec<&str>>()
            .join("");
        values.push(v.parse::<i64>().unwrap());
    });
    println!("{:?}", values);
    race.time = *values.get(0).unwrap();

    race.record = *values.get(1).unwrap();
    race
}
fn race(r: &Race) -> i64 {
    let mut c: i64 = 0;
    for i in 0..r.time {
        let remaining_time = r.time - i;
        let distance = remaining_time * i;
        if distance > r.record {
            c += 1;
        }
    }
    c
}
pub fn day_06_01(input: &str) -> String {
    let races = parse_input(input);
    let r: i64 = races.iter().map(|r| race(r)).product();
    format!("{}", r)
}

pub fn day_06_02(input: &str) -> String {
    let r = parse_input_day_02(input);
    println!("{:?}", r);
    format!("{}", race(&r))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn day_06_part_01() {
        let input = include_str!("./day_06_sample.txt");
        assert_eq!(String::from("288"), day_06_01(input));
    }
    #[test]
    pub fn day_06_part_02() {
        let input = include_str!("./day_06_sample.txt");
        assert_eq!(String::from("71503"), day_06_02(input));
    }
}

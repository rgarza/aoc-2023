use std::collections::HashSet;

pub fn day_05_01(input: &str) -> String {
    let mut seeds_needed: Vec<i64> = vec![];
    let mut seeds_initialized: bool = false;
    let mut changed: HashSet<usize> = HashSet::new();
    input.lines().for_each(|line| {
        if !seeds_initialized {
            let seeds_line = line.split(":").collect::<Vec<&str>>();
            seeds_needed = seeds_line
                .last()
                .unwrap()
                .split_whitespace()
                .filter(|f| f != &"")
                .map(|seed: &str| seed.parse::<i64>().unwrap())
                .collect::<Vec<i64>>();
            println!("seeds needed {:?}", seeds_needed);
            seeds_initialized = true;
        } else {
            if line.len() == 0 {
                changed.clear();
                //skip empty line
            } else if !line.contains(":") {
                // only if it does not contains :
                let values: Vec<i64> = line
                    .split_whitespace()
                    .filter(|f| f != &"")
                    .map(|v: &str| v.trim().parse::<i64>().unwrap())
                    .collect::<Vec<i64>>();
                let destination: i64 = values.get(0).unwrap().clone();
                let source: i64 = values.get(1).unwrap().clone();
                let length = values.get(2).unwrap();
                let max_source = source + length - 1;
                let cloned_seeds = seeds_needed.clone();
                for (idx, s) in cloned_seeds.iter().enumerate() {
                    if !changed.contains(&idx) {
                        println!(
                            "looking for {}, min source {}, max source {}",
                            *s, source, max_source
                        );
                        if *s >= source && *s <= max_source {
                            let dif = *s - source;
                            seeds_needed[idx] = dif + destination;
                            println!("{:?} ==> {:?}", *s, destination + dif);
                            changed.insert(idx);
                        }
                    }
                }
                println!("{:?}", seeds_needed);
            }
        }
    });
    format!("{}", seeds_needed.iter().min().unwrap()).to_string()
}

fn main() {
    let input = include_str!("./day_05_input.txt");
    println!("day 05 01 {}", day_05_01(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day_05_part_01() {
        let input = include_str!("./day_05_sample.txt");
        assert_eq!(String::from("35"), day_05_01(input));
    }
}

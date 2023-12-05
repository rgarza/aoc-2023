use std::collections::HashMap;

pub fn day_02_01(input: &str) -> String {
    let total: u32 = input
        .lines()
        .map(|line| {
            let parts = line.split(":").collect::<Vec<&str>>();
            let game_id = parts[0].split(" ").collect::<Vec<&str>>()[1];
            let mut hash_cubes: HashMap<&str, u32> = HashMap::new();

            hash_cubes.insert("red", 12);
            hash_cubes.insert("green", 13);
            hash_cubes.insert("blue", 14);
            let mut allow_game = true;
            parts[1]
                .split(";")
                .collect::<Vec<&str>>()
                .iter()
                .for_each(|set| {
                    set.split(",")
                        .collect::<Vec<&str>>()
                        .iter()
                        .for_each(|cubes| {
                            let p = cubes.trim().split(" ").collect::<Vec<&str>>();
                            let number_of_cubes = p[0].parse::<u32>().unwrap();
                            if hash_cubes[p[1]] < number_of_cubes {
                                allow_game = false;
                            }
                        })
                });
            if allow_game {
                return game_id.parse::<u32>().unwrap();
            }
            return 0;
        })
        .sum();
    total.to_string()
}

fn main() {
    let input = include_str!("./input_day02_01.txt");
    println!("{:?}", day_02_01(input));
}

#[cfg(test)]
mod tests {
    use crate::day_02_01;

    #[test]
    fn test_day_02_01() {
        let input = include_str!("./sample_day_02_01.txt");
        let expected = String::from("8");
        assert_eq!(expected, day_02_01(input));
    }
}

use std::{collections::HashMap, time::Instant};

fn main() {
    let input = include_str!("./day_08_input.txt");
    println!("Day 08 01 {}", day_08_01(input));
    println!("Day 08 02 {}", day_08_02(input));
}
#[derive(Debug, Clone)]
struct Puzzle {
    instructions: Vec<u8>,
    nodes: HashMap<String, (String, String)>,
}
fn parse_input_01(input: &str) -> Puzzle {
    let mut instructions_loaded: bool = false;
    let mut puzzle: Puzzle = Puzzle {
        instructions: Vec::new(),
        nodes: HashMap::new(),
    };
    input.lines().for_each(|line| {
        if !instructions_loaded {
            instructions_loaded = true;
            for c in line.chars() {
                puzzle.instructions.push(match c {
                    'R' => 1,
                    _ => 0,
                });
            }
        } else if line.len() > 0 {
            let nodes_data = line.split(" = ").collect::<Vec<&str>>();
            let node_value = nodes_data.get(0).unwrap();
            let possible_destinations = nodes_data.get(1).unwrap();
            let destinations_line = possible_destinations.replace("(", "").replace(")", "");
            let destinations_vec = destinations_line.split(",").collect::<Vec<&str>>();
            puzzle.nodes.insert(
                node_value.to_string(),
                (
                    destinations_vec.get(0).unwrap().trim().to_string(),
                    destinations_vec.get(1).unwrap().trim().to_string(),
                ),
            );
        }
    });
    puzzle
}

pub fn day_08_01(input: &str) -> String {
    let instant = Instant::now();
    let puzzle = parse_input_01(input);
    let mut steps: i64 = 0;
    let mut current_instruction: usize = 0;
    let mut current_node_value: String = "AAA".to_string();
    while current_node_value != "ZZZ" {
        let current_node = puzzle.nodes.get(&current_node_value).unwrap();
        let use_node = puzzle.instructions.get(current_instruction).unwrap();
        current_node_value = match use_node {
            0 => current_node.0.clone(),
            _ => current_node.1.clone(),
        };
        steps += 1;
        current_instruction += 1;
        if current_instruction >= puzzle.instructions.len() {
            current_instruction = 0;
        }
    }
    let elapsed = instant.elapsed();
    println!("time elapsed {:.2?}", elapsed);
    format!("{}", steps)
}

#[derive(Debug, Clone)]
struct CurrentNode {
    current_node_value: String,
}

fn get_nodes_ending_with_letter(puzzle: &Puzzle) -> Vec<CurrentNode> {
    let m = puzzle.nodes.keys().clone();
    m.filter(|f| f.ends_with("A"))
        .map(|n| CurrentNode {
            current_node_value: n.clone(),
        })
        .collect::<Vec<CurrentNode>>()
}

pub fn day_08_02(input: &str) -> String {
    let instant = Instant::now();
    let puzzle = parse_input_01(input);
    let queue = get_nodes_ending_with_letter(&puzzle);

    let mut steps: Vec<i64> = Vec::new();
    for c in queue {
        let mut instructions = puzzle.instructions.iter().cycle();
        let mut current_node_value = c.current_node_value.clone();
        let mut number_of_steps: i64 = 0;
        while !current_node_value.ends_with("Z") {
            let current_instruction = instructions.next().unwrap();
            let node = puzzle.nodes.get(&current_node_value).unwrap();
            current_node_value = match current_instruction {
                0 => node.0.clone(),
                _ => node.1.clone(),
            };

            number_of_steps += 1;
        }
        steps.push(number_of_steps);
    }

    let mut iter = steps.iter();

    let f = *iter.next().unwrap();
    let s = *iter.next().unwrap();
    let mut ans = lcm(f, s);
    while let Some(n) = iter.next() {
        ans = lcm(ans, *n);
    }
    let elapsed = instant.elapsed();
    println!("time elapsed: {:.2?}", elapsed);
    format!("{}", ans)
}

// lcm = (a*b)/ gcd(a, b)
// gcm = if b = 0 then a else gcd(b, a%b)

fn gcm(a: i64, b: i64) -> i64 {
    if b == 0 {
        return a;
    }

    gcm(b, a % b)
}

fn lcm(a: i64, b: i64) -> i64 {
    (a * b) / gcm(a, b)
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn test_day_08_part_01() {
        let input = include_str!("./day_08_sample.txt");
        assert_eq!(String::from("2"), day_08_01(input))
    }

    #[test]
    fn test_day_08_part_02() {
        let input = include_str!("./day_08_02_sample.txt");
        assert_eq!(String::from("6"), day_08_02(input))
    }
}

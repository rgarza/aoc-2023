use std::collections::HashMap;

fn main() {
    let input = include_str!("./day_08_input.txt");
    println!("Day 08 01 {}", day_08_01(input));
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
    let puzzle = parse_input_01(input);
    let mut steps: i64 = 0;
    let mut current_instruction: usize = 0;
    let mut current_node_value: String = "AAA".to_string();
    while current_node_value != "ZZZ" {
        println!("{}", current_node_value);
        let current_node = puzzle.nodes.get(&current_node_value).unwrap();
        let use_node = puzzle.instructions.get(current_instruction).unwrap();
        current_node_value = match use_node {
            0 => current_node.0.clone(),
            _ => current_node.1.clone(),
        };
        steps += 1;
        println!("{}", current_node_value);
        current_instruction += 1;
        if current_instruction >= puzzle.instructions.len() {
            current_instruction = 0;
        }
    }
    format!("{}", steps)
}
#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn test_day_08_part_01() {
        let input = include_str!("./day_08_sample.txt");
        assert_eq!(String::from("2"), day_08_01(input))
    }
}

use std::char;

pub fn day_03_01(input: &str) -> String {
    let mut data: Vec<Vec<char>> = Vec::new();
    let mut height: i32 = 0;
    let mut width: i32 = 0;
    input.lines().for_each(|f| {
        height += 1;
        data.push(f.chars().collect());
    });
    width = data.get(0).unwrap().len() as i32;

    let adjacents: Vec<(i32, i32)> = vec![(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)];

    let mut current_sum: i64 = 0;
    for (i, line) in data.iter().enumerate() {
        let mut has_adjacent: bool = false;
        let mut current_number: String = "".to_string();
        for (j, c) in line.iter().enumerate() {
            if c.is_digit(10) {
                current_number = format!("{}{}", current_number, c);
                if !has_adjacent {
                    for (y, x) in adjacents.iter() {
                        let new_x = j as i32 + x;
                        let new_y = i as i32 + y;

                        if new_x < 0 || new_y < 0 || new_x >= width || new_y >= height{
                            continue;
                        }
                        let d = data.get(new_y as usize).unwrap().get(new_x as usize).unwrap();
                        if d != &'.' && !d.is_digit(10) {
                            println!("position y={}, x={}, adjacent ({}, {}) adjacent is {:?}", i, j, x, y, data.get(new_y as usize).unwrap().get(new_x as usize).unwrap());
                            has_adjacent = true;
                            break;
                        }
                    }
                }
            } else { //last of sequence
                println!("Number {}, has Adjacent {}", current_number, has_adjacent);
                if has_adjacent {
                    current_sum += current_number.parse::<i64>().unwrap();
                }
                println!("current sum {:?}", current_sum);
                current_number = "".to_string();
                has_adjacent = false;
            }
        }
        if has_adjacent && current_number.len() > 0 {
            current_sum += current_number.parse::<i64>().unwrap();
        }
    }

    format!("{:?}", current_sum)
}
fn main() {
    let input = include_str!("./day_03_01_input.txt");
    println!("Day 03 part 01, {}", day_03_01(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day_03_part_01() {
        let input = include_str!("./day_03_01_sample.txt");
        let expected = String::from("4361");
        assert_eq!(expected, day_03_01(input));
    }

}

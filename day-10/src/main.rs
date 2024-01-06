pub(crate) use std::cmp::Ordering;
fn main() {
    let input = include_str!("./day_10_input.txt");
    println!("{:?}", day_10_01(input));
}

fn parse_input_01(input: &str) -> (Vec<Vec<char>>, (i64, i64)) {
    let mut res: Vec<Vec<char>> = Vec::new();
    let mut start_position: (i64, i64) = (0, 0);
    let mut idx: i64 = 0;
    input.lines().for_each(|f| {
        res.push(f.chars().collect::<Vec<char>>());
        if let Some(position) = f.find("S") {
            start_position = (idx.clone(), position as i64);
            println!("Starting position {start_position:?}");
        }
        idx += 1;
    });

    (res, start_position)
}

#[derive(PartialEq, Debug, Clone)]
enum Direction {
    NORTH,
    SOUTH,
    EAST,
    WEST,
}

impl Direction {
    fn is_oposite(&self, other: &Direction) -> bool {
        match other {
            Direction::NORTH => *self == Direction::SOUTH,
            Direction::SOUTH => *self == Direction::NORTH,
            Direction::EAST => *self == Direction::WEST,
            Direction::WEST => *self == Direction::EAST,
        }
    }
    fn value(&self) -> (i64, i64) {
        match *self {
            Direction::EAST => (0, 1),
            Direction::WEST => (0, -1),
            Direction::NORTH => (-1, 0),
            Direction::SOUTH => (1, 0),
        }
    }
}
fn next_posible_moves(tile: char) -> Vec<Direction> {
    match tile {
        '|' => vec![Direction::NORTH, Direction::SOUTH],
        '-' => vec![Direction::EAST, Direction::WEST],
        'F' => vec![Direction::EAST, Direction::SOUTH],
        'L' => vec![Direction::EAST, Direction::NORTH],
        'J' => vec![Direction::WEST, Direction::NORTH],
        '7' => vec![Direction::WEST, Direction::SOUTH],
        'S' => vec![
            Direction::SOUTH,
            Direction::WEST,
            Direction::EAST,
            Direction::NORTH,
        ],
        _ => vec![],
    }
}

fn can_reach(tile: char, to: &Direction) -> bool {
    match tile {
        '|' => *to == Direction::NORTH || *to == Direction::SOUTH,
        '-' => *to == Direction::WEST || *to == Direction::EAST,
        'L' => *to == Direction::SOUTH || *to == Direction::WEST,
        'J' => *to == Direction::SOUTH || *to == Direction::EAST,
        '7' => *to == Direction::NORTH || *to == Direction::EAST,
        'F' => *to == Direction::NORTH || *to == Direction::WEST,
        _ => false,
    }
}
#[derive(Debug, Clone)]
struct Path {
    current_position: (i64, i64),
    coming_from: Direction,
    current_tile: char,
    steps: i64,
}

fn solve(field: &Vec<Vec<char>>, start_position: (i64, i64)) -> String {
    let mut possible_paths: Vec<Path> = Vec::new();
    // create all possible paths
    for next_position in next_posible_moves('S') {
        let next_x = start_position.0 + next_position.value().0;
        let next_y = start_position.1 + next_position.value().1;

        if next_x < 0 || next_y < 0 {
            continue;
        }
        let next_tile = field[next_x as usize][next_y as usize];

        if can_reach(next_tile, &next_position) {
            possible_paths.push(Path {
                coming_from: next_position,
                current_position: (next_x, next_y),
                current_tile: next_tile,
                steps: 1,
            });
        }
    }
    loop {
        for i in 0..possible_paths.len() {
            let path = possible_paths.get_mut(i).unwrap();

            for next_position in next_posible_moves(path.current_tile) {
                let next_x = path.current_position.0 + next_position.value().0;
                let next_y = path.current_position.1 + next_position.value().1;

                if next_x < 0 || next_y < 0 || next_position.is_oposite(&path.coming_from) {
                    continue;
                }
                let next_tile = field[next_x as usize][next_y as usize];
                if can_reach(next_tile, &next_position) {
                    path.current_tile = next_tile;
                    path.current_position = (next_x, next_y);
                    path.coming_from = next_position;
                    path.steps += 1;
                    break;
                }
            }
        }
        let equal = possible_paths[0]
            .current_position
            .partial_cmp(&possible_paths[1].current_position);
        if equal.unwrap() == Ordering::Equal {
            break;
        }
    }
    format!("{}", possible_paths[0].steps)
}
pub fn day_10_01(input: &str) -> String {
    let (field, start_position) = parse_input_01(input);
    solve(&field, start_position)
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_10_01() {
        let input = include_str!("./day_10_sample.txt");
        assert_eq!(String::from("8"), day_10_01(input));
    }
}

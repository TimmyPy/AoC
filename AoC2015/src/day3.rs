use std::fs;
use std::collections::HashMap;

#[derive(Hash, Clone)]
struct Position {
    x: i16,
    y: i16 

}

impl PartialEq for Position {
    fn eq(&self, other: &Self) -> bool {
        if self.x == other.x && self.y == other.y {
            return true
        }
        false
    }
}

impl Eq for Position {}

fn read_input() -> String {
    fs::read_to_string("src/inputs/day3.txt").unwrap()
}

pub fn solution_part1() -> u16 {
    let mut positions = HashMap::new();
    let mut current_position = Position {x: 0, y: 0};
    positions.insert(current_position.clone(), 1);

    let input: String = read_input();

    for direction in input.chars() {
        match direction {
            '^' => {
                current_position.y += 1;
            },
            '>' => {
                current_position.x += 1;
            },
            '<' => {
                current_position.x -= 1;
            },
            'v' => {
                current_position.y -= 1;
            },
            '\n' => {
                break;
            }
            _ => {
                panic!("Unexpected input has been passed: {}", direction as u32);
            }
        }
        if !positions.contains_key(&current_position) {
            positions.insert(current_position.clone(), 1);
        }
    }
    match u16::try_from(positions.len()) {
        Ok(val) => val,
        Err(e) => {
            panic!("Imposibble to convert answer to u16: {e}");
        }
    }
}

fn update_position(positions_map: &mut HashMap<Position, i32>, mut position: Position, direction: char) -> Position {
    match direction {
        '^' => {
            position.y += 1;
        },
        '>' => {
            position.x += 1;
        },
        '<' => {
            position.x -= 1;
        },
        'v' => {
            position.y -= 1;
        },
        '\n' => {}
        _ => {
            panic!("Unexpected input has been passed: {}", direction as u32);
        }
    }
    if !positions_map.contains_key(&position) {
        positions_map.insert(position.clone(), 1);
    }

    position

}

pub fn solution_part2() -> u16 {
    let mut positions = HashMap::new();
    let mut current_position_s = Position {x: 0, y: 0};
    let mut current_position_rs = Position {x: 0, y: 0};
    positions.insert(current_position_s.clone(), 1);
    positions.insert(current_position_rs.clone(), 1);

    let input: String = read_input();

    for (i, direction) in input.chars().enumerate() {
        if i % 2 == 0 {
            current_position_rs = update_position(&mut positions, current_position_rs, direction);
        } else {
            current_position_s = update_position(&mut positions, current_position_s, direction);
        }
    }
    match u16::try_from(positions.len()) {
        Ok(val) => {
            val
        },
        _ => {
            panic!("Imposibble to convert answer to u16");
        }
    }
}

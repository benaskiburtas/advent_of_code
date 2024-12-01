use crate::utility::file::read_lines;

const PUZZLE_INPUT_FILE_PATH: &str = "src/year_2024/day_01/puzzle_input.txt";

pub fn solution() {
    let input: Vec<String> = read_lines(PUZZLE_INPUT_FILE_PATH);

    let left_list = get_locations(&input, true);
    let right_list = get_locations(&input, false);

    let total_distance: i32 = left_list
        .iter()
        .zip(right_list.iter())
        .map(|(a, b)| (a - b).abs())
        .sum();

    println!("Total distance: {}", total_distance);
}

fn get_locations(input: &Vec<String>, left_direction: bool) -> Vec<i32> {
    let mut result = Vec::new();

    for line in input {
        let parts: Vec<&str> = line.split_whitespace().collect();

        if parts.len() != 2 {
            panic!("Invalid line format: '{}'. Expected two elements per line.", line);
        }

        let left_location_id: i32 = match parts[0].parse() {
            Ok(n) => n,
            Err(_) => panic!("Failed to parse left location ID from '{}'.", parts[0]),
        };

        let right_location_id: i32 = match parts[1].parse() {
            Ok(n) => n,
            Err(_) => panic!("Failed to parse right location ID from '{}'.", parts[1]),
        };

        if left_direction {
            result.push(left_location_id);
        } else {
            result.push(right_location_id);
        }
    }

    result.sort_unstable();
    result
}
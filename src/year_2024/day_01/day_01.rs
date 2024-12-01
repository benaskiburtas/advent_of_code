use crate::utility::file::read_lines;
use std::collections::HashMap;

const PUZZLE_INPUT_FILE_PATH: &str = "src/year_2024/day_01/puzzle_input.txt";

struct LocationsEntry {
    location_ids: Vec<i32>,
    similarity_map: HashMap<i32, i32>,
}

pub fn solution() {
    let input: Vec<String> = read_lines(PUZZLE_INPUT_FILE_PATH);

    let locations_left = get_locations(&input, true);
    let locations_right = get_locations(&input, false);

    let total_distance: i32 = locations_left
        .location_ids
        .iter()
        .map(|left| {
            let similarity = locations_right.similarity_map.get(left).unwrap_or(&0);
            left * similarity
        })
        .sum();

    println!("Total distance: {}", total_distance);
}
fn get_locations(input: &Vec<String>, left_direction: bool) -> LocationsEntry {
    let mut location_ids = Vec::new();
    let mut similarity_map: HashMap<i32, i32> = HashMap::new();

    for line in input {
        let parts: Vec<&str> = line.split_whitespace().collect();

        if parts.len() != 2 {
            panic!(
                "Invalid line format: '{}'. Expected two elements per line.",
                line
            );
        }

        let left_location_id: i32 = match parts[0].parse() {
            Ok(n) => n,
            Err(_) => panic!("Failed to parse left location ID from '{}'.", parts[0]),
        };

        let right_location_id: i32 = match parts[1].parse() {
            Ok(n) => n,
            Err(_) => panic!("Failed to parse right location ID from '{}'.", parts[1]),
        };

        match left_direction {
            true => {
                location_ids.push(left_location_id);
                *similarity_map.entry(left_location_id).or_insert(0) += 1;
            }
            false => {
                location_ids.push(right_location_id);
                *similarity_map.entry(right_location_id).or_insert(0) += 1;
            }
        }
    }

    location_ids.sort();

    LocationsEntry {
        location_ids,
        similarity_map,
    }
}

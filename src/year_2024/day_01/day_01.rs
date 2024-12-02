use crate::utility::file::read_puzzle_input;
use std::collections::HashMap;

struct LocationsEntry {
    location_ids: Vec<i32>,
    similarity_map: HashMap<i32, i32>,
}

pub fn solution() {
    let input: Vec<String> = read_puzzle_input(file!());
    let (locations_left, locations_right) = get_locations(&input);

    let total_distance: i32 = locations_left
        .location_ids
        .iter()
        .map(|left| {
            let similarity = locations_right.similarity_map.get(left).unwrap_or(&0);
            left * similarity
        })
        .sum();

    println!("Total location distance: {}", total_distance);
}
fn get_locations(input: &Vec<String>) -> (LocationsEntry, LocationsEntry) {
    let mut locations_left = LocationsEntry {
        location_ids: Vec::new(),
        similarity_map: HashMap::new(),
    };

    let mut locations_right = LocationsEntry {
        location_ids: Vec::new(),
        similarity_map: HashMap::new(),
    };

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

        locations_left.location_ids.push(left_location_id);
        *locations_left
            .similarity_map
            .entry(left_location_id)
            .or_insert(0) += 1;

        locations_right.location_ids.push(right_location_id);
        *locations_right
            .similarity_map
            .entry(right_location_id)
            .or_insert(0) += 1;
    }

    locations_left.location_ids.sort();
    locations_right.location_ids.sort();
    (locations_left, locations_right)
}

use advent_of_code::year_2024;
use std::time::Instant;

fn main() {
    let start_time = Instant::now();
    let result = year_2024::day_02_solution();
    let duration = start_time.elapsed();

    println!("Result: {:?}", result);
    println!("Time taken: {:?}", duration);
}

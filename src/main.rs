use std::time::Instant;
use advent_of_code::year_2024;
fn main() {
    let start_time = Instant::now();
    let result = year_2024::day_02::day_02::solution();
    let duration = start_time.elapsed();

    println!("Result: {}", result);
    println!("Time taken: {:?}", duration);
}
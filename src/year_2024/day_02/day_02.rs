use crate::utility::file::read_puzzle_input;
use rayon::prelude::*;
use std::cmp::Ordering;

pub fn solution() -> i32 {
    let reports: Vec<String> = read_puzzle_input(file!());
    let report_count = reports.len();

    let safe_report_count = reports
        .par_iter()
        .filter(|report| is_report_safe_dampened(report))
        .count();

    let unsafe_report_count = reports.len() - safe_report_count;

    println!(
        "Out of {} reports, {} were safe and {} were unsafe",
        report_count, safe_report_count, unsafe_report_count
    );

    safe_report_count as i32
}

fn is_report_safe_dampened(report: &String) -> bool {
    let levels: Vec<i32> = match report
        .split_whitespace()
        .collect::<Vec<&str>>()
        .iter()
        .map(|&level| level.parse::<i32>())
        .collect()
    {
        Ok(level) => level,
        Err(_) => return false,
    };

    if is_report_safe(&levels) {
        true
    } else {
        for i in 0..levels.len() {
            let left = &levels[..i];
            let right = &levels[i + 1..];
            let dampened: Vec<_> = left.iter().chain(right.iter()).cloned().collect();
            if is_report_safe(&dampened) {
                return true;
            }
        }
        false
    }
}

fn is_report_safe(levels: &Vec<i32>) -> bool {
    let mut increasing = true;
    let mut decreasing = true;

    for i in 0..levels.len() - 1 {
        let difference = levels[i + 1] - levels[i];
        if difference.abs() > 3 {
            return false;
        }
        match difference.cmp(&0) {
            Ordering::Less => {
                increasing = false;
                if !decreasing {
                    return false;
                }
            }
            Ordering::Greater => {
                decreasing = false;
                if !increasing {
                    return false;
                }
            }
            Ordering::Equal => return false,
        }
    }
    true
}

use crate::utility::file::read_puzzle_input;
use std::cmp::Ordering;

pub fn solution() {
    let reports: Vec<String> = read_puzzle_input(file!());
    let report_count = reports.len();

    let safe_report_count = reports
        .iter()
        .filter(|report| is_report_safe(report))
        .count();

    let unsafe_report_count = reports.len() - safe_report_count;

    println!(
        "Out of {} reports, {} were safe and {} were unsafe",
        report_count, safe_report_count, unsafe_report_count
    );
}

fn is_report_safe(report: &str) -> bool {
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
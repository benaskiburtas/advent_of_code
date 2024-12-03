use crate::utility::file::read_puzzle_input;
use regex::Regex;

enum Instruction {
    Multiply,
    Dont,
    Do,
}

pub fn solution() -> i32 {
    let memory: Vec<String> = read_puzzle_input(file!());

    let number_pattern: &str = r"\d+";
    let multiply_pattern: &str = r"mul\(\d+,\d+\)";
    let dont_pattern: &str = r"don't\(\)";
    let do_pattern: &str = r"do\(\)";

    let number_regex = Regex::new(number_pattern).unwrap();
    let multiply_regex = Regex::new(multiply_pattern).unwrap();
    let dont_regex = Regex::new(dont_pattern).unwrap();
    let do_regex = Regex::new(do_pattern).unwrap();

    let pattern: Regex =
        Regex::new(format!("{multiply_pattern}|{dont_pattern}|{do_pattern}").as_str()).unwrap();

    let mut sum: i32 = 0;
    let mut is_active: bool = true;

    memory.iter().for_each(|chunk| {
        for found in pattern.find_iter(&chunk) {
            let match_string = found.as_str();

            if let Some(pattern_variant) =
                match_pattern(match_string, &multiply_regex, &dont_regex, &do_regex)
            {
                match pattern_variant {
                    Instruction::Multiply => {
                        if !is_active {
                            continue;
                        }
                        sum += number_regex
                            .find_iter(match_string)
                            .map(|m| m.as_str())
                            .map(|num| num.parse::<i32>().unwrap_or(0))
                            .reduce(|num1, num2| num1 * num2)
                            .unwrap();
                    }
                    Instruction::Dont => {
                        is_active = false;
                    }
                    Instruction::Do => {
                        is_active = true;
                    }
                }
            }
        }
    });
    sum
}

fn match_pattern(
    pattern_str: &str,
    multiply_pattern: &Regex,
    dont_pattern: &Regex,
    do_pattern: &Regex,
) -> Option<Instruction> {
    if multiply_pattern.is_match(pattern_str) {
        Some(Instruction::Multiply)
    } else if dont_pattern.is_match(pattern_str) {
        Some(Instruction::Dont)
    } else if do_pattern.is_match(pattern_str) {
        Some(Instruction::Do)
    } else {
        None
    }
}

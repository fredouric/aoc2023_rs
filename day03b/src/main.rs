use regex::Regex;
use std::fs;

fn find_numbers(input: &str) -> Vec<(&str, i32, i32)> {
    let mut numbers: Vec<(&str, i32, i32)> = Vec::new();
    let re = Regex::new(r"\d+").unwrap();

    for (row, line) in input.lines().enumerate() {
        for num_match in re.find_iter(line) {
            numbers.push((num_match.as_str(), row as i32, num_match.start() as i32));
        }
    }
    numbers
}

fn find_gears(input: &str) -> Vec<(char, i32, i32)> {
    let mut gears: Vec<(char, i32, i32)> = Vec::new();
    for (row, line) in input.lines().enumerate() {
        for (index, char) in line.chars().enumerate() {
            if char == '*' {
                gears.push((char, row as i32, index as i32))
            }
        }
    }
    gears
}

fn find_gear_ratios(
    rows: u32,
    cols: u32,
    numbers: Vec<(&str, i32, i32)>,
    gears: Vec<(char, i32, i32)>,
) -> Vec<u32> {
    let mut gear_ratios: Vec<u32> = Vec::new();

    for (_, gear_x, gear_y) in gears.iter() {
        let adjacent_symbols = [
            (gear_x - 1, gear_y - 1),
            (gear_x - 1, *gear_y),
            (gear_x - 1, gear_y + 1),
            (*gear_x, gear_y - 1),
            (*gear_x, gear_y + 1),
            (gear_x + 1, gear_y - 1),
            (gear_x + 1, *gear_y),
            (gear_x + 1, gear_y + 1),
        ]
        .into_iter()
        .filter(|s| s.0 >= 0 && s.1 >= 0 && s.0 < rows as i32 && s.1 < cols as i32)
        .collect::<Vec<(i32, i32)>>();

        let mut adjacent_numbers: Vec<u32> = Vec::new();
        for (num_str, num_x, num_y) in numbers.iter() {
            let num_end_y = num_y + num_str.len() as i32 - 1;
            for adjacents in adjacent_symbols.iter() {
                if adjacents.0 == *num_x && adjacents.1 >= *num_y && adjacents.1 <= num_end_y {
                    adjacent_numbers.push(num_str.parse::<u32>().unwrap());
                    break;
                }
            }
        }
        if adjacent_numbers.len() == 2 {
            gear_ratios.push(adjacent_numbers.iter().product());
        }
    }
    gear_ratios
}

fn compute(input: &str) -> u32 {
    let numbers = find_numbers(&input);
    let gears = find_gears(&input);
    let gear_ratios = find_gear_ratios(
        input.lines().count() as u32,
        input.lines().next().unwrap().chars().count() as u32,
        numbers,
        gears,
    );
    gear_ratios.iter().sum()
}

fn main() {
    if let Ok(input) = fs::read_to_string("./input.txt") {
        let res = compute(input.as_str());
        println!("{}", res);
    }
}

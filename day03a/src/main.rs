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

fn find_symbols(input: &str) -> Vec<(char, i32, i32)> {
    let mut symbols: Vec<(char, i32, i32)> = Vec::new();
    for (row, line) in input.lines().enumerate() {
        for (index, char) in line.chars().enumerate() {
            if !char.is_ascii_digit() && char != '.' {
                symbols.push((char, row as i32, index as i32))
            }
        }
    }

    symbols
}

fn find_part_numbers(
    rows: u32,
    cols: u32,
    numbers: Vec<(&str, i32, i32)>,
    symbols: Vec<(char, i32, i32)>,
) -> Vec<u32> {
    let mut part_numbers: Vec<u32> = Vec::new();
    for (num_str, num_x, num_y) in numbers.iter() {
        let num_y_end = num_y + num_str.len() as i32 - 1;

        for (_, sym_x, sym_y) in symbols.iter() {
            let adjacent_symbols = [
                (sym_x - 1, sym_y - 1),
                (sym_x - 1, *sym_y),
                (sym_x - 1, sym_y + 1),
                (*sym_x, sym_y - 1),
                (*sym_x, sym_y + 1),
                (sym_x + 1, sym_y - 1),
                (sym_x + 1, *sym_y),
                (sym_x + 1, sym_y + 1),
            ]
            .into_iter()
            .filter(|s| s.0 >= 0 && s.1 >= 0 && s.0 < rows as i32 && s.1 < cols as i32)
            .collect::<Vec<(i32, i32)>>();

            for symbols in adjacent_symbols.iter() {
                if symbols.0 == *num_x && symbols.1 >= *num_y && symbols.1 <= num_y_end {
                    println!("{}", num_str.parse::<u32>().unwrap());
                    part_numbers.push(num_str.parse::<u32>().unwrap());
                    break;
                }
            }
        }
    }
    part_numbers
}

fn compute(input: &str) -> u32 {
    let numbers = find_numbers(&input);
    let symbols = find_symbols(&input);
    let part_numbers = find_part_numbers(
        input.lines().count() as u32,
        input.lines().next().unwrap().chars().count() as u32,
        numbers,
        symbols,
    );
    part_numbers.iter().sum()
}

fn main() {
    if let Ok(input) = fs::read_to_string("./input.txt") {
        let res = compute(input.as_str());
        println!("{}", res);
    }
}

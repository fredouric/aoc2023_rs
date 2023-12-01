use std::fs;

fn main() {
    if let Ok(input) = fs::read_to_string("./input.txt") {
        let res = parse_input(input.as_str());
        println!("{}", res)
    }
}

fn parse_input(input: &str) -> u32 {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            line.to_string()
                .replace("one", "o1e")
                .replace("two", "t2o")
                .replace("three", "t3e")
                .replace("four", "f4r")
                .replace("five", "f5e")
                .replace("six", "s6x")
                .replace("seven", "s7n")
                .replace("eight", "e8t")
                .replace("nine", "n9e")
        })
        .map(|line| {
            line.chars()
                .filter_map(|c| c.to_digit(10))
                .collect::<Vec<u32>>()
        })
        .map(|vec| 10 * vec.first().unwrap() + vec.last().unwrap())
        .sum()
}

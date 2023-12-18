use std::fs;

struct Race {
    duration: u64,
    record: u64,
}

fn main() {
    if let Ok(input) = fs::read_to_string("./input.txt") {
        let races = parse_input_part1(input.as_str());
        println!("{}", compute_ways_product(races));

        let long_race = parse_input_part2(input.as_str());
        println!("{}", compute_ways_product(long_race))
    } else {
        println!("Unable to open file input.txt")
    }
}

fn parse_input_part1(input: &str) -> Vec<Race> {
    let mut races: Vec<Race> = Vec::new();

    let lines: Vec<&str> = input.lines().collect();

    let durations: Vec<u64> = lines[0]
        .split_whitespace()
        .skip(1)
        .map(|s| s.parse().unwrap())
        .collect();

    let records: Vec<u64> = lines[1]
        .split_whitespace()
        .skip(1)
        .map(|s| s.parse().unwrap())
        .collect();

    for (&duration, &record) in durations.iter().zip(records.iter()) {
        let race = Race { duration, record };
        races.push(race);
    }

    races
}

fn parse_input_part2(input: &str) -> Vec<Race> {
    let lines: Vec<&str> = input.lines().collect();

    let durations: Vec<String> = lines[0]
        .split_whitespace()
        .skip(1)
        .map(|s| s.parse().unwrap())
        .collect();

    let records: Vec<String> = lines[1]
        .split_whitespace()
        .skip(1)
        .map(|s| s.parse().unwrap())
        .collect();

    vec![Race {
        duration: durations.concat().parse().unwrap(),
        record: records.concat().parse().unwrap(),
    }]
}

fn compute_ways_product(races: Vec<Race>) -> u32 {
    let mut ways: Vec<u32> = Vec::new();

    for race in races.iter() {
        let mut res: u32 = 0;

        for hold_time in 1..race.duration {
            if (hold_time * (race.duration - hold_time)) > race.record {
                res += 1;
            }
        }
        println!("Numbers of ways to beat record: {}", res);
        ways.push(res);
    }
    ways.iter().product()
}

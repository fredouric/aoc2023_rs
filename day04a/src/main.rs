use std::fs;

pub struct Card {
    id: u32,
    winning: Vec<u32>,
    numbers: Vec<u32>,
}

fn main() {
    if let Ok(input) = fs::read_to_string("./input.txt") {
        let cards = collect_cards(input.as_str());
        let res = compute_points(&cards);
        println!("{}", res)
    }
}

fn collect_cards(input: &str) -> Vec<Card> {
    let mut cards = Vec::new();

    for line in input.lines() {
        let (id_str, numbers_str) = line.split_once(":").unwrap();
        let id = id_str
            .trim_end_matches(":")
            .trim()
            .split_once(' ')
            .unwrap()
            .1
            .trim()
            .parse::<u32>()
            .unwrap();

        let (winning, number) = numbers_str.split_once('|').unwrap();

        let winnings = winning
            .split_whitespace()
            .map(|n| n.trim().parse::<u32>().unwrap())
            .collect();
        let numbers = number
            .split_whitespace()
            .map(|n| n.trim().parse::<u32>().unwrap())
            .collect();

        cards.push(Card {
            id,
            winning: winnings,
            numbers,
        })
    }
    cards
}

fn compute_points(cards: &Vec<Card>) -> u32 {
    cards.iter().fold(0, |acc, card| {
        let matching_numbers = card
            .winning
            .iter()
            .filter(|&winning_number| card.numbers.contains(winning_number))
            .count() as u32;

        acc + if matching_numbers > 0 {
            2u32.pow(matching_numbers - 1)
        } else {
            0
        }
    })
}

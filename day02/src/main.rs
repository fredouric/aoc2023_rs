use std::fs;

const MAX_RED: u32 = 12;
const MAX_GREEN: u32 = 13;
const MAX_BLUE: u32 = 14;

type Reveal = [u32; 3];

pub struct Game {
    id: u32,
    reveals: Vec<Reveal>,
}

fn main() {
    if let Ok(input) = fs::read_to_string("./input.txt") {
        let games = collect_games(input.as_str());
        let id_power = part2(games);
        println!("{}", id_power);
    }
}

fn collect_games(input: &str) -> Vec<Game> {
    let mut games = Vec::new();

    for line in input.lines() {
        let (id_str, scores) = line.split_once(':').unwrap();
        let id = id_str.split_once(' ').unwrap().1.parse().unwrap();
        let reveals = scores
            .split(';')
            .map(|r| {
                let mut reveal = [0, 0, 0];
                for (n, color) in r.split(',').map(|s| s.trim().split_once(' ').unwrap()) {
                    reveal[color.as_bytes()[0] as usize % 3] = n.parse().unwrap();
                }
                reveal
            })
            .collect();
        let game = Game { id, reveals };
        games.push(game);
    }
    games
}

fn part1(games: Vec<Game>) -> u32 {
    games
        .iter()
        .filter(|game| {
            game.reveals.iter().all(|reveal| {
                reveal[0] <= MAX_RED && reveal[1] <= MAX_GREEN && reveal[2] <= MAX_BLUE
            })
        })
        .map(|game| game.id)
        .sum()
}

fn part2(games: Vec<Game>) -> u32 {
    games
        .iter()
        .map(|g| {
            (0..3)
                .map(|i| g.reveals.iter().map(|r| r[i]).max().unwrap())
                .product::<u32>()
        })
        .sum()
}

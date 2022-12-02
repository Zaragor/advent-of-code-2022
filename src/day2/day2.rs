use std::fs;

pub fn part1() -> i32 {
    let file = "C:/Code/advent-of-code-2022/src/day2/input.txt";
    let contents = fs::read_to_string(file)
    .expect("File should exist and be readable");

    let rounds = contents.split("\r\n");
    return rounds.map(calculate_round_score).sum();
}

pub fn part2() -> i32 {
    let file = "C:/Code/advent-of-code-2022/src/day2/input.txt";
    let contents = fs::read_to_string(file)
    .expect("File should exist and be readable");

    let rounds = contents.split("\r\n");
    return rounds.map(calculate_updated_round_score).sum();
}

fn calculate_round_score(round: &str) -> i32 {
    match round {
        "A X" => return 4,
        "A Y" => return 8,
        "A Z" => return 3,
        "B X" => return 1,
        "B Y" => return 5,
        "B Z" => return 9,
        "C X" => return 7,
        "C Y" => return 2,
        "C Z" => return 6,
        _ => panic!("Unexpected combination")
    }
}

fn calculate_updated_round_score(round: &str) -> i32 {
    match round {
        "A X" => return 3,
        "A Y" => return 4,
        "A Z" => return 8,
        "B X" => return 1,
        "B Y" => return 5,
        "B Z" => return 9,
        "C X" => return 2,
        "C Y" => return 6,
        "C Z" => return 7,
        _ => panic!("Unexpected combination")
    }
}
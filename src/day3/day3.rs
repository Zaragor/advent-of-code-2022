use std::fs;
use std::collections::HashSet;
use std::iter::FromIterator;

pub fn part1() -> i32 {
    let file = "C:/Code/advent-of-code-2022/src/day3/input.txt";
    let contents = fs::read_to_string(file)
    .expect("File should exist and be readable");

    let backpacks = contents.split("\r\n");
    return backpacks
    .map(calculate_repeat)
    .map(calculate_score)
    .sum()
}

pub fn part2() -> i32 {
    let file = "C:/Code/advent-of-code-2022/src/day3/input.txt";
    let contents = fs::read_to_string(file)
    .expect("File should exist and be readable");

    let backpacks = contents.split("\r\n").collect::<Vec<&str>>();
    let group_count = backpacks.len() / 3;
    let mut total = 0;

    for group_index in 0..group_count {
        let first = String::from(backpacks[group_index *3]);
        let second = String::from(backpacks[group_index *3 + 1]);
        let third = String::from(backpacks[group_index *3 + 2]);
        let repeats: HashSet<char> = HashSet::from_iter(first.chars().filter(|char| second.contains(*char)).filter(|char| third.contains(*char)));
        if repeats.len() > 1 {
            panic!("multiple repeats found for set {} in strings {} \r\n{}\r\n{}. found {}", group_index, first, second, third, repeats.iter().collect::<String>());
        }
        let score = repeats.iter().map(calculate_score_pnt).collect::<Vec<i32>>();
        let total_score: i32 = score[0];
        total = total + total_score;
    }

    return total;
}

fn calculate_repeat(backpack: &str) -> char {
    let (first, second) = backpack.split_at(backpack.len()/2);
    for char in first.chars() {
        if second.contains(char) {
            return char;
        }
    }
    panic!("No duplicate found in string {}", backpack);
}

fn calculate_score(repeat: char) -> i32 {
    let score = "0abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    return score.find(repeat).expect("repeat character was not a-zA-Z") as i32;
}

fn calculate_score_pnt(repeat: &char) -> i32 {
    let score = "0abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    return score.find(*repeat).expect("repeat character was not a-zA-Z") as i32;
}
use std::fs;

pub fn part1() {
    let elves = parse_list();
    let largest = elves.max();

    Some(println!("{:?}", largest));
}

pub fn part2() {
    let elves = parse_list();
    let mut collected_elves = elves.collect::<Vec<i32>>();
    collected_elves.sort();
    collected_elves.reverse();
    let mut total = 0;
    for n in 0..3 {
        total = total + collected_elves[n];
    }
    println!("{total}");
}

fn parse_list() -> impl Iterator<Item=i32> {
    let file = "C:/Code/advent-of-code-2022/src/day1/input.txt";
    let contents = fs::read_to_string(file)
    .expect("File should exist and be readable");

    let elves = contents.split("\r\n\r\n").map(|s| s.to_string()).collect::<Vec<String>>();
    return elves.into_iter().map(|elf| {
        let items = elf.split("\r\n");
        let elf_total_calories = items.map(|amount| amount.parse::<i32>().expect("Should be numbers"));
        return elf_total_calories.sum::<i32>();
    });
}
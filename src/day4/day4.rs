use std::{fs, fmt::format};

pub fn part1() -> i32 {
    let file = "C:/Code/advent-of-code-2022/src/day4/input.txt";
    let contents = fs::read_to_string(file)
    .expect("File should exist and be readable");

    let assignments = contents.split("\r\n");
    let duplicate_assignments = assignments.filter(duplicate_assignment);
    return duplicate_assignments.count() as i32;
}

pub fn part2() -> i32 {
    let file = "C:/Code/advent-of-code-2022/src/day4/input.txt";
    let contents = fs::read_to_string(file)
    .expect("File should exist and be readable");

    let assignments = contents.split("\r\n");
    let overlapping_assignments = assignments.filter(overlapping_assignments);
    return overlapping_assignments.count() as i32;
}

fn duplicate_assignment<'a>(assignments: &'a &str) -> bool {
    let mut both_assignments = assignments.split(',');
    let first = both_assignments.next().expect("More than zero assignments");
    let first_lower = first.chars().take_while(|char| *char != '-').collect::<String>();
    let first_lower_int = first_lower.parse::<i32>().expect("First should parse to int");
    let first_higher = first.chars().skip_while(|char| *char != '-').skip(1).collect::<String>();
    let first_higher_int = first_higher.parse::<i32>().expect("Second should parse to int");
    println!("First parsed to {} and {}", first_lower_int, first_higher_int);

    let second = both_assignments.next().expect("More than zero assignments");
    let second_lower = second.chars().take_while(|char| *char != '-').collect::<String>();
    let second_lower_int = second_lower.parse::<i32>().expect("First should parse to int");
    let second_higher = second.chars().skip_while(|char| *char != '-').skip(1).collect::<String>();
    let second_higher_int = second_higher.parse::<i32>().expect("Second should parse to int");
    println!("Second parsed to {} and {}", second_lower_int, second_higher_int);

    let mut first_larger = false;
    let mut second_larger = false;
    if first_lower_int <= second_lower_int {
        first_larger = first_higher_int >= second_higher_int;
    }

    if second_lower_int <= first_lower_int {
        second_larger = second_higher_int >= first_higher_int;
    }

    return first_larger || second_larger;
}

fn overlapping_assignments<'a>(assignments: &'a &str) -> bool {
    let mut both_assignments = assignments.split(',');
    let first = both_assignments.next().expect("More than zero assignments");
    let first_lower = first.chars().take_while(|char| *char != '-').collect::<String>();
    let first_lower_int = first_lower.parse::<i32>().expect("First should parse to int");
    let first_higher = first.chars().skip_while(|char| *char != '-').skip(1).collect::<String>();
    let first_higher_int = first_higher.parse::<i32>().expect("Second should parse to int");
    println!("First parsed to {} and {}", first_lower_int, first_higher_int);

    let second = both_assignments.next().expect("More than zero assignments");
    let second_lower = second.chars().take_while(|char| *char != '-').collect::<String>();
    let second_lower_int = second_lower.parse::<i32>().expect("First should parse to int");
    let second_higher = second.chars().skip_while(|char| *char != '-').skip(1).collect::<String>();
    let second_higher_int = second_higher.parse::<i32>().expect("Second should parse to int");
    println!("Second parsed to {} and {}", second_lower_int, second_higher_int);

    let overlap = (first_higher_int >= second_lower_int && first_lower_int <= second_lower_int) || (second_higher_int >= first_lower_int && second_lower_int <= first_lower_int);
    println!("overlapping: {}", overlap);
    return overlap;
}
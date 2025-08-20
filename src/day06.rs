use std::{collections::HashMap, fs};

#[test]
fn test() {
    run();
}

pub fn run() {
    println!("------- DAY06 -------");
    let example = fs::read_to_string("inputs/example_day06").expect("Unable to read input!");
    let example = parse(&example);
    let input = fs::read_to_string("inputs/input_day06").expect("Unable to read input!");
    let input = parse(&input);

    day06_part1(&example, &input);
    day06_part2(&example, &input);
}

fn parse(raw_input: &str) -> Vec<&str> {
    let mut messages: Vec<&str> = vec![];
    for line in raw_input.lines() {
        messages.push(line);
    }
    messages
}

fn error_corrected_message(input: &Vec<&str>) -> String {
    let mut columns: Vec<HashMap<char, u32>> = vec![];
    // Init
    for _ in 0..input[0].len() {
        columns.push(HashMap::new());
    }

    // Count letters by position
    for &message in input {
        for (i, letter) in message.chars().enumerate() {
            let count = columns[i].entry(letter).or_default();
            *count += 1;
        }
    }

    // Find most common letter by position
    let mut corrected_message = String::new();
    for column in columns {
        let mut column_as_vec: Vec<(u32, char)> = vec![];
        for (letter, count) in column {
            column_as_vec.push((count, letter));
            column_as_vec.sort();
        }
        corrected_message.push(column_as_vec[column_as_vec.len() - 1].1);
    }
    corrected_message
}

fn day06_part1(example: &Vec<&str>, input: &Vec<&str>) {
    // Exemple tests
    let res = error_corrected_message(example);
    assert_eq!(res, "easter");

    // Solve puzzle
    let res = error_corrected_message(input);
    println!("Result part 1: {res}");
    // assert_eq!(res, );
    // println!("> DAY06 - part 1: OK!");
}

fn day06_part2(_example: &Vec<&str>, _input: &Vec<&str>) {
    println!("TODO - part2");
    // Exemple tests
    // assert_eq!(, 0);

    // Solve puzzle
    // let res =
    // println!("Result part 2: {res}");
    // assert_eq!(res, );
    // println!("> DAY06 - part 2: OK!");
}

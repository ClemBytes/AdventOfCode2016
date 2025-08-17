use std::fs;

use regex::Regex;

#[test]
fn test() {
    run();
}

pub fn run() {
    println!("------- DAY03 -------");
    let raw_input = fs::read_to_string("inputs/input_day03").expect("Unable to read input!");
    let input_part1 = parse_part1(&raw_input);
    let raw_example = fs::read_to_string("inputs/example_day03").expect("Unable to read input!");
    let example_part2 = parse_part2(&raw_example);
    let input_part2 = parse_part2(&raw_input);

    day03_part1(&input_part1);
    day03_part2(&example_part2, &input_part2);
}

fn parse_part1(raw_input: &str) -> Vec<Vec<u32>> {
    let mut triangles: Vec<Vec<u32>> = vec![];
    let re = Regex::new(r"^\ *([0-9]+)\ +([0-9]+)\ +([0-9]+)$").unwrap();
    for line in raw_input.lines() {
        let matches = re.captures(line).unwrap();
        let mut sides = vec![
            matches[1].parse::<u32>().unwrap(),
            matches[2].parse::<u32>().unwrap(),
            matches[3].parse::<u32>().unwrap(),
        ];
        sides.sort();
        triangles.push(sides);
    }
    triangles
}

fn parse_part2(raw_input: &str) -> Vec<Vec<u32>> {
    let mut triangles: Vec<Vec<u32>> = vec![];
    let re = Regex::new(r"^\ *([0-9]+)\ +([0-9]+)\ +([0-9]+)$").unwrap();
    let mut temp1: Vec<u32> = vec![];
    let mut temp2: Vec<u32> = vec![];
    let mut temp3: Vec<u32> = vec![];
    for (i, line) in raw_input.lines().enumerate() {
        let matches = re.captures(line).unwrap();
        temp1.push(matches[1].parse::<u32>().unwrap());
        temp2.push(matches[2].parse::<u32>().unwrap());
        temp3.push(matches[3].parse::<u32>().unwrap());
        if i % 3 == 2 {
            temp1.sort();
            temp2.sort();
            temp3.sort();

            triangles.push(temp1.clone());
            triangles.push(temp2.clone());
            triangles.push(temp3.clone());

            temp1.clear();
            temp2.clear();
            temp3.clear();
        }
    }
    triangles
}

fn constructible(triangle: &[u32]) -> bool {
    triangle[0] + triangle[1] > triangle[2]
}

fn day03_part1(input: &[Vec<u32>]) {
    // Exemple tests
    assert!(!constructible(&[5, 10, 25]));

    // Solve puzzle
    let mut res = 0;
    for triangle in input {
        if constructible(triangle) {
            res += 1;
        }
    }
    println!("Result part 1: {res}");
    assert_eq!(res, 1050);
    println!("> DAY03 - part 1: OK!");
}

fn day03_part2(example: &[Vec<u32>], input: &[Vec<u32>]) {
    // Exemple tests
    let mut res = 0;
    for triangle in example {
        if constructible(triangle) {
            res += 1;
        }
    }
    assert_eq!(res, 6);

    // Solve puzzle
    let mut res = 0;
    for triangle in input {
        if constructible(triangle) {
            res += 1;
        }
    }
    println!("Result part 2: {res}");
    assert_eq!(res, 1921);
    println!("> DAY03 - part 2: OK!");
}

use std::fs;

use regex::Regex;

#[test]
fn test() {
    run();
}

pub fn run() {
    println!("------- DAY03 -------");
    let input = fs::read_to_string("inputs/input_day03").expect("Unable to read input!");
    let input = parse(&input);

    day03_part1(&input);
    day03_part2(&input);
}

fn parse(raw_input: &str) -> Vec<Vec<u32>> {
    let mut triangles: Vec<Vec<u32>> = vec![];
    let re = Regex::new(r"^\ +([0-9]+)\ +([0-9]+)\ +([0-9]+)$").unwrap();
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

fn day03_part2(_input: &[Vec<u32>]) {
    println!("TODO - part2");
    // Exemple tests
    // assert_eq!(, 0);

    // Solve puzzle
    // let res =
    // println!("Result part 2: {res}");
    // assert_eq!(res, );
    // println!("> DAY03 - part 2: OK!");
}

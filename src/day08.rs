use std::fs;

use regex::Regex;

#[test]
fn test() {
    run();
}

pub fn run() {
    println!("------- DAY08 -------");
    let example = fs::read_to_string("inputs/example_day08").expect("Unable to read input!");
    let example = Instruction::parse(&example);
    let input = fs::read_to_string("inputs/input_day08").expect("Unable to read input!");
    let input = Instruction::parse(&input);

    day08_part1(&example, &input);
    day08_part2(&example, &input);
}

#[derive(Debug, Clone, Copy)]
enum RotationOn {
    Column,
    Row,
}

#[derive(Debug, Clone, Copy)]
struct Rotation {
    on: RotationOn,
    id: u32,
    shift: u32,
}

#[derive(Debug, Clone, Copy)]
enum Instruction {
    Rect(u32, u32),
    Rotate(Rotation),
}

impl Instruction {
    fn parse(raw_input: &str) -> Vec<Self> {
        let mut instructions: Vec<Instruction> = vec![];
        let re_rect = Regex::new(r"([0-9]+)x([0-9]+)").unwrap();
        let re_rotate_col = Regex::new(r"x=([0-9]+) by ([0-9]+)").unwrap();
        let re_rotate_row = Regex::new(r"y=([0-9]+) by ([0-9]+)").unwrap();
        for line in raw_input.lines() {
            let (kind, infos) = line.split_once(" ").unwrap();
            match kind {
                "rect" => {
                    let matches = re_rect.captures(infos).unwrap();
                    instructions.push(Instruction::Rect(
                        matches[1].parse().unwrap(),
                        matches[2].parse().unwrap(),
                    ));
                },
                "rotate" => {
                    let (on, infos_rotation) = infos.split_once(" ").unwrap();
                    match on {
                        "column" => {
                            let matches = re_rotate_col.captures(infos_rotation).unwrap();
                            instructions.push(Instruction::Rotate(Rotation {
                                on: RotationOn::Column,
                                id: matches[1].parse().unwrap(),
                                shift: matches[2].parse().unwrap(),
                            }));
                        },
                        "row" => {
                            let matches = re_rotate_row.captures(infos_rotation).unwrap();
                            instructions.push(Instruction::Rotate(Rotation {
                                on: RotationOn::Row,
                                id: matches[1].parse().unwrap(),
                                shift: matches[2].parse().unwrap(),
                            }));
                        },
                        _ => panic!("Unknown rotation: {on} ({infos_rotation})")
                    }
                },
                _ => panic!("Unknown instruction: {kind} ({infos})"),
            }
        }
        instructions
    }
}


fn day08_part1(example: &Vec<Instruction>, _input: &Vec<Instruction>) {
    println!("{example:#?}");
    // Exemple tests
    // assert_eq!(, 0);

    // Solve puzzle
    // let res =
    // println!("Result part 1: {res}");
    // assert_eq!(res, );
    // println!("> DAY08 - part 1: OK!");
}

fn day08_part2(_example: &Vec<Instruction>, _input: &Vec<Instruction>) {
    println!("TODO - part2");
    // Exemple tests
    // assert_eq!(, 0);

    // Solve puzzle
    // let res =
    // println!("Result part 2: {res}");
    // assert_eq!(res, );
    // println!("> DAY08 - part 2: OK!");
}

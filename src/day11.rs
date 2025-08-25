use std::{fs, vec};

#[test]
fn test() {
    run();
}

pub fn run() {
    println!("------- DAY11 -------");
    let example = Area {
        elevator_floor: 1,
        elevator_contains: vec![],
        floor1: vec![Object::Microchip('H'), Object::Microchip('L')],
        floor2: vec![Object::Generator('H')],
        floor3: vec![Object::Generator('L')],
        floor4: vec![],
    };
    let input = Area {
        elevator_floor: 1,
        elevator_contains: vec![],
        floor1: vec![
            Object::Generator('P'),
            Object::Generator('T'),
            Object::Microchip('T'),
            Object::Generator('p'),
            Object::Generator('R'),
            Object::Microchip('R'),
            Object::Generator('C'),
            Object::Microchip('C'),
        ],
        floor2: vec![
            Object::Microchip('P'),
            Object::Microchip('p')
        ],
        floor3: vec![],
        floor4: vec![],
    };

    day11_part1(example.clone(), input.clone());
    day11_part2(example, input);
}

#[derive(Debug, Clone)]
enum Object {
    Microchip(char),
    Generator(char),
}

#[derive(Debug, Clone)]
struct Area {
    elevator_floor: u32,
    elevator_contains: Vec<Object>,
    floor1: Vec<Object>,
    floor2: Vec<Object>,
    floor3: Vec<Object>,
    floor4: Vec<Object>,
}

fn day11_part1(_example: Area, _input: Area) {
    println!("TODO - part1");
    // Exemple tests
    // assert_eq!(, 0);

    // Solve puzzle
    // let res =
    // println!("Result part 1: {res}");
    // assert_eq!(res, );
    // println!("> DAY11 - part 1: OK!");
}

fn day11_part2(_example: Area, _input: Area) {
    println!("TODO - part2");
    // Exemple tests
    // assert_eq!(, 0);

    // Solve puzzle
    // let res =
    // println!("Result part 2: {res}");
    // assert_eq!(res, );
    // println!("> DAY11 - part 2: OK!");
}

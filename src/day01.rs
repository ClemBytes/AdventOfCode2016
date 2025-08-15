use std::fs;

#[test]
fn test() {
    run();
}

pub fn run() {
    println!("------- DAY01 -------");
    let input = fs::read_to_string("inputs/input_day01").expect("Unable to read input!");
    let input = parse(&input);

    day01_part1(&input);
    day01_part2(&input);
}

#[derive(Debug, Copy, Clone)]
enum Turn {
    R,
    L,
}

impl Turn {
    fn from_char(ch: char) -> Self {
        match ch {
            'R' => Turn::R,
            'L' => Turn::L,
            other => panic!("Unkwnow Turn '{other}'! Should be 'R' or 'L'."),
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct Instruction {
    turn: Turn,
    nb_blocks: i32,
}

fn parse(raw_input: &str) -> Vec<Instruction> {
    let raw_instructions: Vec<&str> = raw_input.trim().split(", ").collect();
    let mut instructions: Vec<Instruction> = vec![];
    for instruction in raw_instructions {
        let turn = instruction.chars().collect::<Vec<char>>()[0];
        let turn = Turn::from_char(turn);
        let nb_blocks = instruction[1..].to_string().parse().unwrap();
        instructions.push(Instruction { turn, nb_blocks });
    }
    instructions
}

enum Facing {
    North,
    South,
    East,
    West,
}

struct State {
    facing: Facing,
    n_s_coord: i32,
    e_w_coord: i32,
}

impl State {
    fn turn(&mut self, t: Turn) {
        match t {
            Turn::R => match self.facing {
                Facing::North => self.facing = Facing::East,
                Facing::East => self.facing = Facing::South,
                Facing::South => self.facing = Facing::West,
                Facing::West => self.facing = Facing::North,
            },
            Turn::L => match self.facing {
                Facing::North => self.facing = Facing::West,
                Facing::West => self.facing = Facing::South,
                Facing::South => self.facing = Facing::East,
                Facing::East => self.facing = Facing::North,
            },
        }
    }

    fn walk(&mut self, nb: i32) {
        match self.facing {
            Facing::North => self.n_s_coord += nb,
            Facing::South => self.n_s_coord -= nb,
            Facing::East => self.e_w_coord += nb,
            Facing::West => self.e_w_coord -= nb,
        }
    }
}

fn blocks_away(instructions: &Vec<Instruction>) -> i32 {
    let mut state = State {
        facing: Facing::North,
        n_s_coord: 0,
        e_w_coord: 0,
    };

    for instruction in instructions {
        state.turn(instruction.turn);
        state.walk(instruction.nb_blocks);
    }

    state.n_s_coord.abs() + state.e_w_coord.abs()
}

fn day01_part1(input: &Vec<Instruction>) {
    // Exemple tests
    let ex: Vec<Instruction> = vec![
        Instruction {
            turn: Turn::R,
            nb_blocks: 2,
        },
        Instruction {
            turn: Turn::L,
            nb_blocks: 3,
        },
    ];
    assert_eq!(blocks_away(&ex), 5);
    let ex: Vec<Instruction> = vec![
        Instruction {
            turn: Turn::R,
            nb_blocks: 2,
        },
        Instruction {
            turn: Turn::R,
            nb_blocks: 2,
        },
        Instruction {
            turn: Turn::R,
            nb_blocks: 2,
        },
    ];
    assert_eq!(blocks_away(&ex), 2);
    let ex: Vec<Instruction> = vec![
        Instruction {
            turn: Turn::R,
            nb_blocks: 5,
        },
        Instruction {
            turn: Turn::L,
            nb_blocks: 5,
        },
        Instruction {
            turn: Turn::R,
            nb_blocks: 5,
        },
        Instruction {
            turn: Turn::R,
            nb_blocks: 3,
        },
    ];
    assert_eq!(blocks_away(&ex), 12);

    // Solve puzzle
    let res = blocks_away(&input);
    println!("Result part 1: {res}");
    assert_eq!(res, 300);
    println!("> DAY01 - part 1: OK!");
}

fn day01_part2(_input: &Vec<Instruction>) {
    println!("TODO - part2");
    // Exemple tests
    // assert_eq!(, 0);

    // Solve puzzle
    // let res =
    // println!("Result part 2: {res}");
    // assert_eq!(res, );
    // println!("> DAY01 - part 2: OK!");
}

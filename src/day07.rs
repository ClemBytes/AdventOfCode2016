use std::fs;

#[test]
fn test() {
    run();
}

pub fn run() {
    println!("------- DAY07 -------");
    let example = fs::read_to_string("inputs/example_day07").expect("Unable to read input!");
    let example = parse(&example);
    let input = fs::read_to_string("inputs/input_day07").expect("Unable to read input!");
    let input = parse(&input);

    day07_part1(&example, &input);
    day07_part2(&example, &input);
}

#[derive(Debug, Clone)]
struct Block {
    within_brackets: bool,
    sequence: String,
}

impl Block {
    fn contains_abba(&self) -> bool {
        let mut first_char = self.sequence.chars().nth(0);
        let mut second_char = self.sequence.chars().nth(1);
        for i in 2..self.sequence.len() - 1 {
            if first_char != second_char
                && self.sequence.chars().nth(i) == second_char
                && self.sequence.chars().nth(i + 1) == first_char
            {
                return true;
            }
            first_char = second_char;
            second_char = self.sequence.chars().nth(i);
        }
        false
    }
}

#[derive(Debug, Clone)]
struct Address {
    nb_blocks: u32,
    blocks: Vec<Block>,
}

impl Address {
    fn supports_tls(&self) -> bool {
        let mut res = false;
        for block in self.blocks.iter() {
            if block.contains_abba() && !block.within_brackets {
                res = true;
            } else if block.contains_abba() && block.within_brackets {
                return false;
            }
        }
        res
    }
}

fn parse(raw_input: &str) -> Vec<Address> {
    let mut addresses: Vec<Address> = vec![];
    for line in raw_input.lines() {
        let mut address = Address {
            nb_blocks: 0,
            blocks: vec![],
        };
        let mut block = Block {
            within_brackets: false,
            sequence: String::new(),
        };
        for ch in line.chars() {
            if ch == '[' {
                address.nb_blocks += 1;
                address.blocks.push(block.clone());
                block.sequence.clear();
                block.within_brackets = true;
            } else if ch == ']' {
                address.nb_blocks += 1;
                address.blocks.push(block.clone());
                block.sequence.clear();
                block.within_brackets = false;
            } else {
                block.sequence.push(ch);
            }
        }
        address.nb_blocks += 1;
        address.blocks.push(block.clone());
        addresses.push(address);
    }
    addresses
}

fn day07_part1(example: &Vec<Address>, input: &Vec<Address>) {
    // Exemple tests
    let mut res = 0;
    for address in example {
        if address.supports_tls() {
            res += 1;
        }
    }
    assert_eq!(res, 2);

    // Solve puzzle
    let mut res = 0;
    for address in input {
        if address.supports_tls() {
            res += 1;
        }
    }
    println!("Result part 1: {res}");
    // assert_eq!(res, );
    // println!("> DAY07 - part 1: OK!");
}

fn day07_part2(_example: &Vec<Address>, _input: &Vec<Address>) {
    println!("TODO - part2");
    // Exemple tests
    // assert_eq!(, 0);

    // Solve puzzle
    // let res =
    // println!("Result part 2: {res}");
    // assert_eq!(res, );
    // println!("> DAY07 - part 2: OK!");
}

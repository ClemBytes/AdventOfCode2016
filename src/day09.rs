use std::fs;

#[test]
fn test() {
    run();
}

pub fn run() {
    println!("------- DAY09 -------");
    let input = fs::read_to_string("inputs/input_day09").expect("Unable to read input!");
    let input = input.trim();

    day09_part1(input);
    day09_part2(input);
}

fn decompress(input: &str, print: bool) -> usize {
    let mut decompressed_string = String::new();
    let mut i = 0;
    let input_chars: Vec<char> = input.chars().collect();
    while i < input_chars.len() {
        let mut ch = input_chars[i];
        if ch == '(' {
            let mut nb_chars = String::new();
            i += 1;
            ch = input_chars[i];
            while ch != 'x' {
                nb_chars.push(ch);
                i += 1;
                ch = input_chars[i];
            }
            assert_eq!(ch, 'x');
            let nb_chars: usize = nb_chars.parse().unwrap();
            let mut repeat = String::new();
            i += 1;
            ch = input_chars[i];
            while ch != ')' {
                repeat.push(ch);
                i += 1;
                ch = input_chars[i];
            }
            assert_eq!(ch, ')');
            let repeat: usize = repeat.parse().unwrap();
            let mut to_add = String::new();
            for _ in 0..nb_chars {
                i += 1;
                ch = input_chars[i];
                to_add.push(ch);
            }
            for _ in 0..repeat {
                for ch_add in to_add.chars() {
                    assert_ne!(ch, ' ');
                    if ch != ' ' {
                        decompressed_string.push(ch_add);
                    }
                }
            }
            i += 1;
        } else {
            assert_ne!(ch, ' ');
            if ch != ' ' {
                decompressed_string.push(ch);
            }
            i += 1;
        }
    }
    if print {
        println!("{input} => {decompressed_string}");
    }
    decompressed_string.len()
}

fn day09_part1(input: &str) {
    // Exemple tests
    let ex = "ADVENT";
    let res = decompress(ex, true);
    assert_eq!(res, 6);
    let ex = "A(1x5)BC";
    let res = decompress(ex, true);
    assert_eq!(res, 7);
    let ex = "(3x3)XYZ";
    let res = decompress(ex, true);
    assert_eq!(res, 9);
    let ex = "A(2x2)BCD(2x2)EFG";
    let res = decompress(ex, true);
    assert_eq!(res, 11);
    let ex = "(6x1)(1x3)A";
    let res = decompress(ex, true);
    assert_eq!(res, 6);
    let ex = "X(8x2)(3x3)ABCY";
    let res = decompress(ex, true);
    assert_eq!(res, 18);

    // Solve puzzle
    let res = decompress(input, false);
    println!("Result part 1: {res}");
    assert_eq!(res, 102239);
    println!("> DAY09 - part 1: OK!");
}

fn day09_part2(_input: &str) {
    println!("TODO - part2");
    // Exemple tests
    // assert_eq!(, 0);

    // Solve puzzle
    // let res =
    // println!("Result part 2: {res}");
    // assert_eq!(res, );
    // println!("> DAY09 - part 2: OK!");
}

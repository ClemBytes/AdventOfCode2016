use md5::Digest;

#[test]
fn test() {
    run();
}

pub fn run() {
    println!("------- DAY05 -------");
    let input = "ugkcyxxp";

    day05_part1(input);
    day05_part2(input);
}

fn has_five_hex_zeros(hash: Digest) -> bool {
    hash[0] == 0 && hash[1] == 0 && hash[2] & 0xF0 == 0
}

fn find_password(door_id: &str) -> String {
    let mut i = 0;
    let mut password = String::new();
    while password.len() < 8 {
        let s = format!("{door_id}{i}");
        let hash = md5::compute(s);
        if hash.len() > 5 && has_five_hex_zeros(hash) {
            password = format!("{password}{:x}", hash[2]);
        }
        i += 1
    }
    password
}

fn day05_part1(input: &str) {
    // Exemple tests
    let example = "abc";
    let res = find_password(example);
    assert_eq!(res, "18f47a30");
    println!();

    // Solve puzzle
    let res = find_password(input);
    println!("Result part 1: {res}");
    assert_eq!(res, "d4cd2ee1");
    println!("> DAY05 - part 1: OK!");
}

fn day05_part2(_input: &str) {
    println!("TODO - part2");
    // Exemple tests
    // assert_eq!(, 0);

    // Solve puzzle
    // let res =
    // println!("Result part 2: {res}");
    // assert_eq!(res, );
    // println!("> DAY05 - part 2: OK!");
}

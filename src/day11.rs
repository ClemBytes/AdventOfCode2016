use std::{
    collections::HashSet,
    vec,
};

#[test]
fn test() {
    run();
}

pub fn run() {
    /*
    0 H
    1 L
    2 P
    3 T
    4 p
    5 R
    6 C
     */
    println!("------- DAY11 -------");
    let mut example = Area {
        elevator: 0,
        floors: [
            vec![Object::Microchip(0), Object::Microchip(1)],
            vec![Object::Generator(0)],
            vec![Object::Generator(1)],
            vec![],
        ],
    };
    example.floors[0].sort();
    example.floors[1].sort();
    example.floors[2].sort();
    example.floors[3].sort();
    let mut input = Area {
        elevator: 0,
        floors: [
            vec![
                Object::Generator(2),
                Object::Generator(3),
                Object::Microchip(3),
                Object::Generator(4),
                Object::Generator(5),
                Object::Microchip(5),
                Object::Generator(6),
                Object::Microchip(6),
            ],
            vec![Object::Microchip(2), Object::Microchip(4)],
            vec![],
            vec![],
        ],
    };
    input.floors[0].sort();
    input.floors[1].sort();
    input.floors[2].sort();
    input.floors[3].sort();

    day11_part1(example.clone(), input.clone());
    day11_part2(example, input);
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Ord, PartialOrd)]
enum Object {
    Microchip(u8),
    Generator(u8),
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Area {
    elevator: usize,
    floors: [Vec<Object>; 4],
}

fn check_floor(floor: &Vec<Object>) -> bool {
    let mut microchips: u32 = 0;
    let mut generators: u32 = 0;
    for object in floor {
        match object {
            Object::Microchip(element) => {
                microchips |= 2_u32.pow((*element).into());
            }
            Object::Generator(element) => {
                generators |= 2_u32.pow((*element).into());
            }
        }
    }
    if microchips == generators {
        return true;
    }
    if microchips & !generators == 0 || generators & !microchips == 0
    {
        return true;
    }
    false
}

/*
fn temp() {
    let m = HashSet::from(['a', 'b']);
    let g = HashSet::from(['a', 'c']);
    let cond = m.difference(&g).next().is_none() || g.difference(&m).next().is_none();
    println!("m: {:?}\ng: {:?}\nm.diff(g): {:?}\ng.diff(m): {:?}\ncondition: {}\n", m, g, m.difference(&g), g.difference(&m), cond);
    let m = HashSet::from(['a', 'b']);
    let g = HashSet::from(['a', 'b', 'c']);
    let cond = m.difference(&g).next().is_none() || g.difference(&m).next().is_none();
    println!("m: {:?}\ng: {:?}\nm.diff(g): {:?}\ng.diff(m): {:?}\ncondition: {}\n", m, g, m.difference(&g), g.difference(&m), cond);
    let m = HashSet::from(['a', 'b', 'c']);
    let g = HashSet::from(['a', 'b']);
    let cond = m.difference(&g).next().is_none() || g.difference(&m).next().is_none();
    println!("m: {:?}\ng: {:?}\nm.diff(g): {:?}\ng.diff(m): {:?}\ncondition: {}\n", m, g, m.difference(&g), g.difference(&m), cond);
}
*/

fn move_floor(
    current_area: &Area,
    q: &mut Vec<Area>,
    from: usize,
    to: usize,
) {
    // Max elevator capacity: 2 objects
    // I can move :
    // - 1 object
    // - 2 microchips
    // - 2 generator
    // - 1 microchip + 1 generator ONLY if they are compatible

    // Try moving only one object:
    for (i, &object) in current_area.floors[from].iter().enumerate() {
        let mut next_area = current_area.clone();
        next_area.floors[from].remove(i);
        next_area.floors[to].push(object);
        if check_floor(&current_area.floors[from]) && check_floor(&current_area.floors[to]) {
            next_area.floors[to].sort();
            next_area.elevator = to;
            q.push(next_area);
        }
    }

    // Try moving 2 objects:
    for (i, &object) in current_area.floors[from].iter().enumerate() {
        for j in i + 1..current_area.floors[from].len() {
            let mut next_area = current_area.clone();
            next_area.floors[from].remove(j);
            let second_object = next_area.floors[from].remove(i);
            let mut can_move = match (object, second_object) {
                (Object::Microchip(_), Object::Microchip(_)) => true,
                (Object::Generator(_), Object::Generator(_)) => true,
                (Object::Microchip(e1), Object::Generator(e2)) => e1 == e2,
                (Object::Generator(e1), Object::Microchip(e2)) => e1 == e2,
            };
            can_move = can_move
                && check_floor(&current_area.floors[from])
                && check_floor(&current_area.floors[to]);
            if can_move {
                next_area.floors[to].push(object);
                next_area.floors[to].push(second_object);
                next_area.floors[to].sort();
                if check_floor(&next_area.floors[from]) && check_floor(&next_area.floors[to]) {
                    next_area.elevator = to;
                    q.push(next_area);
                }
            }
        }
    }
}

fn find_minimum_steps(input: Area) -> u32 {
    // Init
    let mut visited_from_start: HashSet<Area> = HashSet::new();
    let mut front_from_start = Vec::new();
    let mut end_area = Area {
        elevator: 3,
        floors: [vec![], vec![], vec![], vec![]],
    };
    for f in &input.floors {
        for object in f {
            end_area.floors[3].push(object.clone());
        }
    }
    let mut visited_from_end: HashSet<Area> = HashSet::new();
    let mut front_from_end = Vec::new();

    // Start
    front_from_start.push(input.clone());
    front_from_end.push(end_area.clone());
    let mut nb_steps_from_start = 0;
    let mut nb_steps_from_end = 0;
    loop {
        // Go one step further from start
        let mut next_front = vec![];
        while let Some(current_from_start) = front_from_start.pop()
        {
            // Check already visited areas from start
            if visited_from_start.contains(&current_from_start) {
                continue;
            }
            visited_from_start.insert(current_from_start.clone());

            // Check if current area from start has been seen from end
            if visited_from_end.contains(&current_from_start) {
                println!("A: nb_steps_from_start: {nb_steps_from_start} | nb_steps_from_end: {nb_steps_from_end}");
                return nb_steps_from_start + nb_steps_from_end - 1;
            }

            // Check if find end
            assert_ne!(current_from_start, end_area);

            // If no object on elevator floor => impossible, continue
            for f in 0..4 {
                assert!(!(current_from_start.elevator == f && current_from_start.floors[f].is_empty()));
            }

            // For each microchip at a given floor, I need to check if there is no generator that can fry it
            if !check_floor(&current_from_start.floors[0])
                || !check_floor(&current_from_start.floors[1])
                || !check_floor(&current_from_start.floors[2])
                || !check_floor(&current_from_start.floors[3])
            {
                continue;
            }

            // Create next front
            match current_from_start.elevator {
                0 => {
                    // Move from for floor 0 to 1:
                    move_floor(
                        &current_from_start,
                        &mut next_front,
                        0,
                        1,
                    );
                }
                1 => {
                    // Move from for floor 1 to 0:
                    move_floor(
                        &current_from_start,
                        &mut next_front,
                        1,
                        0,
                    );
                    // Move from for floor 1 to 2:
                    move_floor(
                        &current_from_start,
                        &mut next_front,
                        1,
                        2,
                    );
                }
                2 => {
                    // Move from for floor 2 to 1:
                    move_floor(
                        &current_from_start,
                        &mut next_front,
                        2,
                        1,
                    );
                    // Move from for floor 2 to 3:
                    move_floor(
                        &current_from_start,
                        &mut next_front,
                        2,
                        3,
                    );
                }
                3 => {
                    // Move from for floor 3 to 2:
                    move_floor(
                        &current_from_start,
                        &mut next_front,
                        3,
                        2,
                    );
                }
                other => panic!("Unknown elevator floor: {other}"),
            }
        }
        nb_steps_from_start += 1;
        front_from_start = next_front;

        // Go one step before from end
        let mut next_front = vec![];
        while let Some(current_from_end) = front_from_end.pop()
        {
            // Check already visited areas from start
            if visited_from_end.contains(&current_from_end) {
                continue;
            }
            visited_from_end.insert(current_from_end.clone());

            // Check if current area from start has been seen from end
            if visited_from_start.contains(&current_from_end) {
                println!("C: nb_steps_from_start: {nb_steps_from_start} | nb_steps_from_end: {nb_steps_from_end}");
                return nb_steps_from_start + nb_steps_from_end - 1;
            }

            // Check if find start
            assert_ne!(current_from_end, input);

            // If no object on elevator floor => impossible, continue
            for f in 0..4 {
                assert!(!(current_from_end.elevator == f && current_from_end.floors[f].is_empty()));
            }

            // For each microchip at a given floor, I need to check if there is no generator that can fry it
            if !check_floor(&current_from_end.floors[0])
                || !check_floor(&current_from_end.floors[1])
                || !check_floor(&current_from_end.floors[2])
                || !check_floor(&current_from_end.floors[3])
            {
                continue;
            }

            // Create next front
            match current_from_end.elevator {
                0 => {
                    // Move from for floor 0 to 1:
                    move_floor(
                        &current_from_end,
                        &mut next_front,
                        0,
                        1,
                    );
                }
                1 => {
                    // Move from for floor 1 to 0:
                    move_floor(
                        &current_from_end,
                        &mut next_front,
                        1,
                        0,
                    );
                    // Move from for floor 1 to 2:
                    move_floor(
                        &current_from_end,
                        &mut next_front,
                        1,
                        2,
                    );
                }
                2 => {
                    // Move from for floor 2 to 1:
                    move_floor(
                        &current_from_end,
                        &mut next_front,
                        2,
                        1,
                    );
                    // Move from for floor 2 to 3:
                    move_floor(
                        &current_from_end,
                        &mut next_front,
                        2,
                        3,
                    );
                }
                3 => {
                    // Move from for floor 3 to 2:
                    move_floor(
                        &current_from_end,
                        &mut next_front,
                        3,
                        2,
                    );
                }
                other => panic!("Unknown elevator floor: {other}"),
            }
        }
        nb_steps_from_end += 1;
        front_from_end = next_front;
    }
}

fn day11_part1(example: Area, input: Area) {
    // Exemple tests
    // temp();
    let res = find_minimum_steps(example);
    assert_eq!(res, 11);

    // Solve puzzle
    let res = find_minimum_steps(input);
    println!("Result part 1: {res}");
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

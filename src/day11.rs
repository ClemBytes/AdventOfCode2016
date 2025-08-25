use std::{
    collections::{HashSet, VecDeque},
    vec,
};

#[test]
fn test() {
    run();
}

pub fn run() {
    println!("------- DAY11 -------");
    let mut example = Area {
        elevator: 0,
        floors: [
            vec![Object::Microchip('H'), Object::Microchip('L')],
            vec![Object::Generator('H')],
            vec![Object::Generator('L')],
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
                Object::Generator('P'),
                Object::Generator('T'),
                Object::Microchip('T'),
                Object::Generator('p'),
                Object::Generator('R'),
                Object::Microchip('R'),
                Object::Generator('C'),
                Object::Microchip('C'),
            ],
            vec![Object::Microchip('P'), Object::Microchip('p')],
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
    Microchip(char),
    Generator(char),
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Area {
    elevator: usize,
    floors: [Vec<Object>; 4],
}

fn check_floor(floor: &Vec<Object>) -> bool {
    let mut microchips: HashSet<char> = HashSet::new();
    let mut generators: HashSet<char> = HashSet::new();
    for object in floor {
        match object {
            Object::Microchip(element) => {
                microchips.insert(*element);
            }
            Object::Generator(element) => {
                generators.insert(*element);
            }
        }
    }
    if microchips == generators {
        return true;
    }
    if microchips.difference(&generators).next().is_none()
        || generators.difference(&microchips).next().is_none()
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
    current_nb_steps: u32,
    current_area: &Area,
    q: &mut VecDeque<(u32, Area)>,
    from: usize,
    to: usize,
) {
    // Try moving only one object:
    for (i, &object) in current_area.floors[from].iter().enumerate() {
        let mut next_area = current_area.clone();
        next_area.floors[from].remove(i);
        next_area.floors[to].push(object);
        if check_floor(&current_area.floors[from]) && check_floor(&current_area.floors[to]) {
            next_area.floors[to].sort();
            next_area.elevator = to;
            q.push_back((current_nb_steps + 1, next_area));
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
                    q.push_back((current_nb_steps + 1, next_area));
                }
            }
        }
    }
}

fn find_minimum_steps(input: Area) -> u32 {
    let mut visited: HashSet<Area> = HashSet::new();
    let mut q = VecDeque::new();
    q.push_back((0, input));
    'main_loop: while let Some(current) = q.pop_front() {
        let (current_nb_steps, current_area) = current;
        if visited.contains(&current_area) {
            continue;
        }
        visited.insert(current_area.clone());

        if current_area.floors[0].is_empty()
            && current_area.floors[1].is_empty()
            && current_area.floors[2].is_empty()
        {
            return current_nb_steps;
        }
        // If no object on elevator floor => impossible, continue
        for f in 0..4 {
            if current_area.elevator == f && current_area.floors[f].is_empty() {
                continue 'main_loop;
            }
        }
        // For each microchip at a given floor, I need to check if there is no generator that can fry it
        if !check_floor(&current_area.floors[0])
            || !check_floor(&current_area.floors[1])
            || !check_floor(&current_area.floors[2])
            || !check_floor(&current_area.floors[3])
        {
            continue;
        }
        // Max elevator capacity: 2 objects
        // I can move :
        // - 1 object
        // - 2 microchips
        // - 2 generator
        // - 1 microchip + 1 generator ONLY if they are compatible
        match current_area.elevator {
            0 => {
                // Move from for floor 0 to 1:
                move_floor(current_nb_steps, &current_area, &mut q, 0, 1);
            }
            1 => {
                // Move from for floor 1 to 0:
                move_floor(current_nb_steps, &current_area, &mut q, 1, 0);
                // Move from for floor 1 to 2:
                move_floor(current_nb_steps, &current_area, &mut q, 1, 2);
            }
            2 => {
                // Move from for floor 2 to 1:
                move_floor(current_nb_steps, &current_area, &mut q, 2, 1);
                // Move from for floor 2 to 3:
                move_floor(current_nb_steps, &current_area, &mut q, 2, 3);
            }
            3 => {
                // Move from for floor 3 to 2:
                move_floor(current_nb_steps, &current_area, &mut q, 3, 2);
            }
            other => panic!("Unknown elevator floor: {other}"),
        }
    }
    unreachable!();
}

fn day11_part1(example: Area, input: Area) {
    // Exemple tests
    // temp();
    let res = find_minimum_steps(example);
    println!("Example OK");
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

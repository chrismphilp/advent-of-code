use std::{cmp, fs};

use crate::y2023::day_10::day_10::Direction::{E, N, S, W};
use crate::y2023::day_10::day_10::Pipe::{GROUND, HORIZONTAL, NEBend, NWBend, SEBend, START, SWBend, VERTICAL};

pub fn process_answer() {
    println!("The sum for day tens loop path count is: {}", part_1());
}

fn part_1() -> usize {
    let input = fs::read_to_string("src/y2023/day_10/input.txt").unwrap();
    let diagram = input.lines()
        .map(|v| v.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    find_number_of_steps_in_loop(&diagram)
}

fn find_number_of_steps_in_loop(diagram: &Vec<Vec<char>>) -> usize {
    let grid_width = diagram.len() as i32 - 1;
    let start_x = diagram.iter().position(|v| v.contains(&'S')).unwrap() as i32;
    let start_y = diagram[start_x as usize].iter().position(|v| *v == 'S').unwrap() as i32;

    let mut count: usize = 1;

    let mut prev: (i32, i32) = (start_x, start_y);
    let mut curr: (i32, i32) = *calculate_next_possible_moves(diagram, start_x, start_y, grid_width).iter()
        .filter(|v| **v != prev)
        .next().unwrap();

    while curr != (start_x, start_y) {
        let tmp = curr;
        let possible_values = calculate_next_possible_moves(diagram, curr.0, curr.1, grid_width);

        curr = *possible_values.iter()
            .filter(|v| **v != prev)
            .next()
            .unwrap();

        prev = tmp;
        count += 1;
    }
    count / 2
}

fn calculate_next_possible_moves(diagram: &Vec<Vec<char>>, curr_x: i32, curr_y: i32, max: i32) -> Vec<(i32, i32)> {
    let mut found = vec!();

    for (next_x, next_y, from_direction, to_direction) in vec!(
        (cmp::min(cmp::max(0, curr_x), max), cmp::max(0, cmp::min(curr_y - 1, max)), W, E), // Left
        (cmp::min(cmp::max(0, curr_x + 1), max), cmp::max(0, cmp::min(curr_y, max)), S, N), // Below
        (cmp::min(cmp::max(0, curr_x), max), cmp::max(0, cmp::min(curr_y + 1, max)), E, W), // Right
        (cmp::min(cmp::max(0, curr_x - 1), max), cmp::max(0, cmp::min(curr_y, max)), N, S), // Above
    ) {
        let curr = Pipe::from_char(diagram[curr_x as usize][curr_y as usize]);
        let next = Pipe::from_char(diagram[next_x as usize][next_y as usize]);

        if is_valid_next_tile(curr, &from_direction) && is_valid_next_tile(next, &to_direction) {
            found.push((next_x, next_y));
        }
    }
    found
}

fn is_valid_next_tile(curr: Pipe, direction: &Direction) -> bool {
    Pipe::available_directions(curr).map_or(false, |v| v.contains(direction))
}

#[derive(PartialEq)]
enum Direction {
    N,
    E,
    S,
    W,
}

#[derive(PartialEq)]
enum Pipe {
    START,
    VERTICAL,
    HORIZONTAL,
    NEBend,
    NWBend,
    SWBend,
    SEBend,
    GROUND,
}

impl Pipe {
    pub const fn from_char(c: char) -> Pipe {
        match c {
            '|' => VERTICAL,
            '-' => HORIZONTAL,
            'L' => NEBend,
            'J' => NWBend,
            '7' => SWBend,
            'F' => SEBend,
            '.' => GROUND,
            'S' => START,
            _ => panic!(),
        }
    }

    pub fn available_directions(pipe: Pipe) -> Option<Vec<Direction>> {
        match pipe {
            START => Some(vec!(N, E, S, W)),
            VERTICAL => Some(vec!(N, S)),
            HORIZONTAL => Some(vec!(E, W)),
            NEBend => Some(vec!(N, E)),
            NWBend => Some(vec!(N, W)),
            SWBend => Some(vec!(S, W)),
            SEBend => Some(vec!(S, E)),
            GROUND => None,
        }
    }
}

#[cfg(test)]
mod test {
    use super::find_number_of_steps_in_loop;

    #[test]
    fn should_find_correct_number_of_forward_steps_for_simple_diagram() {
        let simple_diagram = vec!(
            vec!('.', '.', '.', '.', '.'),
            vec!('.', 'S', '-', '7', '.'),
            vec!('.', '|', '.', '|', '.'),
            vec!('.', 'L', '-', 'J', '.'),
            vec!('.', '.', '.', '.', '.'),
        );
        assert_eq!(find_number_of_steps_in_loop(&simple_diagram), 4);
    }

    #[test]
    fn should_find_correct_number_of_forward_steps_for_complex_diagram() {
        let complex_diagram = vec!(
            vec!('.', '.', 'F', '7', '.'),
            vec!('.', 'F', 'J', '|', '.'),
            vec!('S', 'J', '.', 'L', '7'),
            vec!('|', 'F', '-', '-', 'J'),
            vec!('L', 'J', '.', '.', '.'),
        );
        assert_eq!(find_number_of_steps_in_loop(&complex_diagram), 8);
    }
}

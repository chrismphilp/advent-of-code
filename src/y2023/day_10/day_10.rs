use std::{cmp, fs};
use std::collections::HashSet;

use crate::y2023::day_10::day_10::Direction::{E, N, S, W};
use crate::y2023::day_10::day_10::Pipe::{GROUND, HORIZONTAL, IGNORED, NEBend, NWBend, SEBend, START, SWBend, VERTICAL};

pub fn process_answer() {
    println!("The sum for day tens loop path count is: {}", part_1());
    println!("The sum for day tens loop fill count is: {}", part_2());
}

fn part_1() -> usize {
    let input = fs::read_to_string("src/y2023/day_10/input.txt").unwrap();
    let diagram = input.lines()
        .map(|v| v.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let path = find_loop_path(&diagram);

    (path.len() / 2) + 1
}

fn part_2() -> usize {
    let input = fs::read_to_string("src/y2023/day_10/input.txt").unwrap();
    let diagram = input.lines()
        .map(|v| v.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let path = find_loop_path(&diagram);
    let path_only_diagram = create_path_only_diagram(&diagram, &path);

    find_tiles_inside_loop(&path_only_diagram)
}

fn find_loop_path(diagram: &Vec<Vec<char>>) -> HashSet<(i32, i32)> {
    let grid_width = diagram.len() as i32 - 1;
    let grid_height = diagram[0].len() as i32 - 1;
    let start_x = diagram.iter().position(|v| v.contains(&'S')).unwrap() as i32;
    let start_y = diagram[start_x as usize].iter().position(|v| *v == 'S').unwrap() as i32;

    let mut path: HashSet<(i32, i32)> = HashSet::new();

    let mut prev: (i32, i32) = (start_x, start_y);
    let mut curr: (i32, i32) = *calculate_next_possible_moves(diagram, start_x, start_y, grid_width, grid_height).iter()
        .filter(|v| **v != prev)
        .next().unwrap();
    path.insert(curr);

    while curr != (start_x, start_y) {
        let tmp = curr;
        let possible_values = calculate_next_possible_moves(diagram, curr.0, curr.1, grid_width, grid_height);

        curr = *possible_values.iter()
            .filter(|v| **v != prev)
            .next()
            .unwrap();

        path.insert(curr);
        prev = tmp;
    }
    path
}

fn calculate_next_possible_moves(diagram: &Vec<Vec<char>>, curr_x: i32, curr_y: i32, max_x: i32, max_y: i32) -> Vec<(i32, i32)> {
    let mut found = vec!();

    for (next_x, next_y, from_direction, to_direction) in vec!(
        (cmp::min(cmp::max(0, curr_x), max_x), cmp::max(0, cmp::min(curr_y - 1, max_y)), W, E), // Left
        (cmp::min(cmp::max(0, curr_x + 1), max_x), cmp::max(0, cmp::min(curr_y, max_y)), S, N), // Below
        (cmp::min(cmp::max(0, curr_x), max_x), cmp::max(0, cmp::min(curr_y + 1, max_y)), E, W), // Right
        (cmp::min(cmp::max(0, curr_x - 1), max_x), cmp::max(0, cmp::min(curr_y, max_y)), N, S), // Above
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

fn create_path_only_diagram(diagram: &Vec<Vec<char>>, path: &HashSet<(i32, i32)>) -> Vec<Vec<char>> {
    let mut diagram_copy = diagram.clone();

    for (x, xv) in diagram.iter().enumerate() {
        for (y, _) in xv.iter().enumerate() {
            if !path.contains(&(x as i32, y as i32)) {
                diagram_copy[x][y] = '?';
            }
        }
    }
    diagram_copy
}

fn find_tiles_inside_loop(path_only_diagram: &Vec<Vec<char>>) -> usize {
    let mut count: usize = 0;
    let vertical_pipes = vec!(VERTICAL, NEBend, NWBend);

    for xv in path_only_diagram {
        let mut intersecting = false;

        for yv in xv {
            let curr = Pipe::from_char(*yv);

            if curr == IGNORED && intersecting {
                count += 1;
            } else if vertical_pipes.contains(&curr) {
                intersecting = !intersecting;
            }
        }
    }
    count
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
    IGNORED,
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
            _ => IGNORED,
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
            IGNORED => None,
        }
    }
}

#[cfg(test)]
mod test {
    use super::{create_path_only_diagram, find_loop_path, find_tiles_inside_loop};

    #[test]
    fn should_find_correct_number_of_forward_steps_for_simple_diagram() {
        let simple_diagram = vec!(
            vec!('.', '.', '.', '.', '.'),
            vec!('.', 'S', '-', '7', '.'),
            vec!('.', '|', '.', '|', '.'),
            vec!('.', 'L', '-', 'J', '.'),
            vec!('.', '.', '.', '.', '.'),
        );
        assert_eq!(find_loop_path(&simple_diagram).len() / 2, 4);
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
        assert_eq!(find_loop_path(&complex_diagram).len() / 2, 8);
    }

    #[test]
    fn should_find_correct_number_of_tiles_inside_loop_for_simple_diagram_1() {
        let simple_diagram = vec!(
            vec!('.', '.', '.', '.', '.'),
            vec!('.', 'S', '-', '7', '.'),
            vec!('.', '|', '.', '|', '.'),
            vec!('.', 'L', '-', 'J', '.'),
            vec!('.', '.', '.', '.', '.'),
        );
        let path = find_loop_path(&simple_diagram);
        let path_only_diagram = create_path_only_diagram(&simple_diagram, &path);

        assert_eq!(find_tiles_inside_loop(&path_only_diagram), 1);
    }

    #[test]
    fn should_find_correct_number_of_tiles_inside_loop_for_simple_diagram_2() {
        let complex_diagram = vec!(
            vec!('.', '.', 'F', '7', '.'),
            vec!('.', 'F', 'J', '|', '.'),
            vec!('S', 'J', '.', 'L', '7'),
            vec!('|', 'F', '-', '-', 'J'),
            vec!('L', 'J', '.', '.', '.'),
        );
        let path = find_loop_path(&complex_diagram);
        let path_only_diagram = create_path_only_diagram(&complex_diagram, &path);

        assert_eq!(find_tiles_inside_loop(&path_only_diagram), 1);
    }

    #[test]
    fn should_find_correct_number_of_tiles_inside_loop_for_complex_diagram_1() {
        let complex_diagram = vec!(
            vec!('.', '.', '.', '.', '.', '.', '.', '.', '.', '.'),
            vec!('.', 'S', '-', '-', '-', '-', '-', '-', '7', '.'),
            vec!('.', '|', 'F', '-', '-', '-', '-', '7', '|', '.'),
            vec!('.', '|', '|', '.', '.', '.', '.', '|', '|', '.'),
            vec!('.', '|', '|', '.', '.', '.', '.', '|', '|', '.'),
            vec!('.', '|', 'L', '-', '7', 'F', '-', 'J', '|', '.'),
            vec!('.', '|', '.', '.', '|', '|', '.', '.', '|', '.'),
            vec!('.', 'L', '-', '-', 'J', 'L', '-', '-', 'J', '.'),
            vec!('.', '.', '.', '.', '.', '.', '.', '.', '.', '.'),
        );
        let path = find_loop_path(&complex_diagram);
        let path_only_diagram = create_path_only_diagram(&complex_diagram, &path);

        assert_eq!(find_tiles_inside_loop(&path_only_diagram), 4);
    }

    #[test]
    fn should_find_correct_number_of_tiles_inside_loop_for_complex_diagram_2() {
        let complex_diagram = vec!(
            vec!('.', 'F', '-', '-', '-', '-', '7', 'F', '7', 'F', '7', 'F', '7', 'F', '-', '7', '.', '.', '.', '.'),
            vec!('.', '|', 'F', '-', '-', '7', '|', '|', '|', '|', '|', '|', '|', '|', 'F', 'J', '.', '.', '.', '.'),
            vec!('.', '|', '|', '.', 'F', 'J', '|', '|', '|', '|', '|', '|', '|', '|', 'L', '7', '.', '.', '.', '.'),
            vec!('F', 'J', 'L', '7', 'L', '7', 'L', 'J', 'L', 'J', '|', '|', 'L', 'J', '.', 'L', '-', '7', '.', '.'),
            vec!('L', '-', '-', 'J', '.', 'L', '7', '.', '.', '.', 'L', 'J', 'S', '7', 'F', '-', '7', 'L', '7', '.'),
            vec!('.', '.', '.', '.', 'F', '-', 'J', '.', '.', 'F', '7', 'F', 'J', '|', 'L', '7', 'L', '7', 'L', '7'),
            vec!('.', '.', '.', '.', 'L', '7', '.', 'F', '7', '|', '|', 'L', '7', '|', '.', 'L', '7', 'L', '7', '|'),
            vec!('.', '.', '.', '.', '.', '|', 'F', 'J', 'L', 'J', '|', 'F', 'J', '|', 'F', '7', '|', '.', 'L', 'J'),
            vec!('.', '.', '.', '.', 'F', 'J', 'L', '-', '7', '.', '|', '|', '.', '|', '|', '|', '|', '.', '.', '.'),
            vec!('.', '.', '.', '.', 'L', '-', '-', '-', 'J', '.', 'L', 'J', '.', 'L', 'J', 'L', 'J', '.', '.', '.'),
        );
        let path = find_loop_path(&complex_diagram);
        let path_only_diagram = create_path_only_diagram(&complex_diagram, &path);

        assert_eq!(find_tiles_inside_loop(&path_only_diagram), 8);
    }
}

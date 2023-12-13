use std::fs;

pub fn process_answer() {
    println!("The sum for day nines difference is: {}", part_1());
    println!("The sum for day nines backward difference is: {}", part_2());
}

fn part_1() -> i32 {
    let input = fs::read_to_string("src/y2023/day_9/input.txt").unwrap();
    let lines = input.lines()
        .map(|v| v.split_whitespace().map(|v| v.parse::<i32>().unwrap()).collect::<Vec<i32>>())
        .collect::<Vec<Vec<i32>>>();

    lines.iter()
        .map(|v| find_extrapolated_values(v))
        .sum()
}

fn find_extrapolated_values(values: &Vec<i32>) -> i32 {
    let mut grid: Vec<Vec<i32>> = vec!(vec!(0; values.len()); values.len());
    let grid_len = grid.len();

    values.iter().enumerate()
        .for_each(|(i, v)| grid[0][i] = *v);

    for ix in 0..grid_len {
        for iy in 1..grid[ix].len() - ix {
            let a = &grid[ix][iy - 1];
            let b = &grid[ix][iy];

            grid[ix + 1][iy - 1] = b - a;
        }
    }

    let mut sum = 0;

    for ix in 0..grid_len {
        sum += grid[ix][grid_len - ix - 1];
    }

    sum
}

fn part_2() -> i32 {
    let input = fs::read_to_string("src/y2023/day_9/input.txt").unwrap();
    let lines = input.lines()
        .map(|v| v.split_whitespace().map(|v| v.parse::<i32>().unwrap()).collect::<Vec<i32>>())
        .collect::<Vec<Vec<i32>>>();

    lines.iter()
        .map(|v| find_backward_extrapolated_values(v))
        .sum()
}

fn find_backward_extrapolated_values(values: &Vec<i32>) -> i32 {
    let rev_values = values.iter().rev().map(|v| *v).collect::<Vec<i32>>();
    find_extrapolated_values(&rev_values)
}

#[cfg(test)]
mod test {
    use super::{find_backward_extrapolated_values, find_extrapolated_values};

    #[test]
    fn should_find_correct_number_of_forward_steps_for_standard_ending() {
        assert_eq!(find_extrapolated_values(&vec!(0, 3, 6, 9, 12, 15)), 18);
        assert_eq!(find_extrapolated_values(&vec!(1, 3, 6, 10, 15, 21)), 28);
        assert_eq!(find_extrapolated_values(&vec!(10, 13, 16, 21, 30, 45)), 68);
    }

    #[test]
    fn should_find_correct_number_of_backward_steps_for_standard_ending() {
        assert_eq!(find_backward_extrapolated_values(&vec!(0, 3, 6, 9, 12, 15)), -3);
        assert_eq!(find_backward_extrapolated_values(&vec!(1, 3, 6, 10, 15, 21)), 0);
        assert_eq!(find_backward_extrapolated_values(&vec!(10, 13, 16, 21, 30, 45)), 5);
    }
}

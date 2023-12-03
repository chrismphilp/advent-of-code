use std::{cmp, fs};
use regex::Regex;

pub fn process_answer() {
    println!("The sum for day threes simple engine calibration is: {}", part_1());
    println!("The sum for day threes gear calibration is: {}", part_2());
}

fn part_1() -> i32 {
    let input = fs::read_to_string("src/day_3/input.txt").unwrap();
    let vec = input.lines().collect::<Vec<&str>>();
    process_engine_calibration(vec)
}

fn process_engine_calibration(lines: Vec<&str>) -> i32 {
    let mut sum: i32 = 0;
    let line_length = lines.len();
    let full_vec = lines.into_iter()
        .map(|v| v.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    for (i, line) in full_vec.clone().into_iter().enumerate() {
        let mut row_x = 0;

        while row_x < line.len() {
            let lh = line[row_x..line.len()].iter()
                .position(|v| v.is_digit(10))
                .map(|v| v + row_x)
                .unwrap_or(line.len());

            let rh = (line[lh..line.len()].iter()
                .position(|v| !v.is_digit(10))
                .map(|v| v + lh)
                .unwrap_or(line.len())) - 1;

            let actual_number = find_actual_numeric_value(&line, lh, rh);

            if actual_number > 0 {
                let contains_symbol = full_vec[cmp::max(i as i32 - 1, 0) as usize..=cmp::min(line_length - 1, i + 1)].iter()
                    .map(|v| v[cmp::max(lh as i32 - 1, 0) as usize..=cmp::min(rh + 1, line.len() - 1)].iter().collect::<Vec<&char>>())
                    .any(|v| find_row_value(v));

                if contains_symbol {
                    sum += actual_number;
                }
            }

            row_x = rh + 1
        }
    }

    sum
}

fn find_row_value(row: Vec<&char>) -> bool {
    row.iter().any(|v| !v.is_digit(10) && **v != '.')
}

fn part_2() -> i32 {
    let input = fs::read_to_string("src/day_3/input.txt").unwrap();
    let lines_vec = input.lines().collect::<Vec<&str>>();
    process_gear_value(&lines_vec)
}

fn process_gear_value(lines: &Vec<&str>) -> i32 {
    let re: Regex = Regex::new("[0-9]+").unwrap();
    let mut sum: i32 = 0;
    let full_vec = lines.into_iter()
        .map(|v| v.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    for (i, line) in full_vec.clone().into_iter().enumerate() {
        let asterix_indexes = line.iter()
            .enumerate()
            .filter(|(_, &v)| v == '*')
            .map(|(index, _)| index)
            .collect::<Vec<usize>>();

        for asterix_position in asterix_indexes {
            let mut count = 0;
            let mut product: i32 = 1;

            let mut calculated_indexes: Vec<Vec<i32>> = (0..140).into_iter()
                .map(|_| vec!(-1; 140))
                .collect();

            for x in cmp::max(i as i32 - 1, 0) as usize..=cmp::min(i + 1, full_vec.len() - 1) {
                for y in cmp::max(asterix_position as i32 - 1, 0) as usize..=cmp::min(asterix_position + 1, full_vec.len() - 1) {
                    for number in re.find_iter(lines[x]) {
                        let start_index = number.start() as i32;

                        if count > 2 {
                            break;
                        }
                        if !calculated_indexes[x].contains(&start_index) && number.range().contains(&y) {
                            let actual_number = find_actual_numeric_value(&full_vec[x], number.start(), number.end() - 1);
                            product *= actual_number;
                            count += 1;
                            calculated_indexes[x].push(start_index);
                        }
                    }
                }
            }

            if count == 2 {
                sum += product;
            }
        }
    }
    sum
}

fn find_actual_numeric_value(line: &Vec<char>, min: usize, max: usize) -> i32 {
    let mut actual_number: i32 = 0;
    let mut power = 1;

    for c in (min..=max).rev() {
        actual_number += (line[c].to_digit(10).unwrap() * power) as i32;
        power = power * 10;
    }
    actual_number
}

#[cfg(test)]
mod test {
    use std::fs;
    use crate::day_3::day_3::{process_engine_calibration, process_gear_value};

    #[test]
    fn should_find_correct_engine_calibration_value() {
        let lines: Vec<&str> = vec!(
            "467..114..",
            "...*......",
            "..35..633.",
            "21....#...",
            "617*......",
            "617*..@874",
            "....5.....",
        );
        assert_eq!(process_engine_calibration(lines), 467 + 35 + 633 + 617 + 617 + 874 + 5);
    }

    #[test]
    fn should_find_correct_engine_gear_value() {
        let lines: Vec<&str> = vec!(
            "467..114..",
            "...*.....*",
            "..35.*633.",
            "21....#*..",
            "617*......",
            "617*..@874",
            "....5.....",
            "35..5.....",
            "*35.5.....",
            "..5.5..150",
            "*...5...2*"
        );
        assert_eq!(process_gear_value(&lines), (467 * 35) + (617 * 617) + (35 * 35) + (150 * 2));

        let input = fs::read_to_string("src/day_3/input.txt").unwrap();
        let lines_vec = input.lines().collect::<Vec<&str>>();
        assert_eq!(process_gear_value(&lines_vec), 84266818);
    }
}

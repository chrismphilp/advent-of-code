use std::{cmp, fs};
use std::collections::BTreeMap;

pub fn process_answer() {
    println!("The sum for day fours scratchcard winning values are: {}", part_1());
    println!("The sum for day fours scratchcard copies are: {}", part_2());
}

fn part_1() -> i32 {
    fs::read_to_string("src/day_4/input.txt")
        .unwrap()
        .lines()
        .map(|v| process_number_of_winning_values(v))
        .filter(|v| *v > 0)
        .map(|v| 2i32.pow(v as u32 - 1))
        .sum()
}

fn process_number_of_winning_values(line: &str) -> i32 {
    let split_line = line.split('|')
        .map(|v| v.trim())
        .collect::<Vec<&str>>();

    let winning_numbers = split_line[0]
        .split(':')
        .map(|v| v.trim())
        .collect::<Vec<&str>>()[1]
        .split_whitespace()
        .map(|x| (x, 1))
        .collect::<BTreeMap<_, _>>();

    let chosen_numbers = split_line[1]
        .split_whitespace()
        .collect::<Vec<&str>>();

    chosen_numbers.into_iter()
        .filter(|v| winning_numbers.contains_key(v))
        .map(|_| 1)
        .sum()
}

fn part_2() -> i32 {
    let input = fs::read_to_string("src/day_4/input.txt").unwrap();
    let lines = input.lines().collect::<Vec<&str>>();
    process_number_of_scratchcard_copies(lines)
}

fn process_number_of_scratchcard_copies(lines: Vec<&str>) -> i32 {
    let mut tracker: Vec<i32> = vec![0; lines.len() + 1];

    for (i, line) in lines.iter().enumerate() {
        let wins = process_number_of_winning_values(line);
        tracker[i] += 1;

        for x in i..cmp::min(i + wins as usize, lines.len()) {
            tracker[x + 1] += tracker[i];
        }
    }
    tracker.iter().sum()
}

#[cfg(test)]
mod test {
    use crate::day_4::day_4::{process_number_of_scratchcard_copies, process_number_of_winning_values};

    #[test]
    fn should_find_correct_scratchcard_winning_values() {
        assert_eq!(process_number_of_winning_values("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53"), 4);
        assert_eq!(process_number_of_winning_values("Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19"), 2);
        assert_eq!(process_number_of_winning_values("Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1"), 2);
        assert_eq!(process_number_of_winning_values("Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83"), 1);
        assert_eq!(process_number_of_winning_values("Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36"), 0);
        assert_eq!(process_number_of_winning_values("Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"), 0);
    }

    #[test]
    fn should_find_correct_scratchcard_copy_values() {
        let input = vec!(
            "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53",
            "Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19",
            "Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1",
            "Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83",
            "Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36",
            "Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"
        );

        assert_eq!(process_number_of_scratchcard_copies(input), 30)
    }
}

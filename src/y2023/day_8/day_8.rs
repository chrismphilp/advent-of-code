use std::collections::HashMap;
use std::fs;
use std::hash::Hash;
use regex::Regex;

pub fn process_answer() {
    println!("The sum for day eights step count is: {}", part_1());
}

fn part_1() -> i32 {
    let re: Regex = Regex::new("[A-Z]+").unwrap();

    let input = fs::read_to_string("src/y2023/day_8/input.txt").unwrap();
    let lines = input.lines().collect::<Vec<&str>>();
    let instructions = lines[0].trim().chars().collect::<Vec<char>>();
    let directions = lines[2..].iter()
        .map(|v| {
            let matches = re.find_iter(*v).map(|v| v.as_str()).collect::<Vec<&str>>();
            (matches[0], matches[1], matches[2])
        })
        .fold(HashMap::new(), |mut acc: HashMap<&str, (&str, &str)>, c| {
            acc.insert(c.0, (c.1, c.2));
            acc
        });

    find_number_of_steps(instructions, "AAA", directions)
}

fn find_number_of_steps(instructions: Vec<char>, start: &str, directions: HashMap<&str, (&str, &str)>) -> i32 {
    let mut index: usize = 0;
    let mut curr: &str = start;
    let mut step_count: i32 = 0;

    loop {
        if index == instructions.len() {
            index = 0;
        }

        if curr == "ZZZ" {
            return step_count;
        }

        let direction = instructions[index];
        let options = *directions.get(curr).unwrap();

        if direction == 'L' {
            curr = options.0;
        } else {
            curr = options.1;
        }

        step_count += 1;
        index += 1;
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;
    use super::find_number_of_steps;

    #[test]
    fn should_find_correct_number_of_steps() {
        let instructions1 = vec!('R', 'L');
        let directions1: HashMap<&str, (&str, &str)> = HashMap::from([
            ("AAA", ("BBB", "CCC")),
            ("BBB", ("DDD", "EEE")),
            ("CCC", ("ZZZ", "GGG")),
            ("DDD", ("DDD", "DDD")),
            ("EEE", ("EEE", "EEE")),
            ("GGG", ("GGG", "GGG")),
            ("ZZZ", ("ZZZ", "ZZZ"))
        ]);
        assert_eq!(find_number_of_steps(instructions1, "AAA", directions1), 2);

        let instructions2 = vec!('L', 'L', 'R');
        let directions2: HashMap<&str, (&str, &str)> = HashMap::from([
            ("AAA", ("BBB", "BBB")),
            ("BBB", ("AAA", "ZZZ")),
            ("ZZZ", ("ZZZ", "ZZZ"))
        ]);
        assert_eq!(find_number_of_steps(instructions2, "AAA", directions2), 6);
    }
}

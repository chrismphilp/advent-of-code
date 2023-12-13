use std::collections::HashMap;
use std::fs;

use regex::Regex;

pub fn process_answer() {
    println!("The sum for day eights step count is: {}", part_1());
    println!("The sum for day eights LCM count is: {}", part_2());
}

fn part_1() -> i32 {
    let re: Regex = Regex::new("[A-Z0-9]+").unwrap();

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

    let matcher: Regex = Regex::new("ZZZ").unwrap();
    find_number_of_steps(&instructions, "AAA", &directions, &matcher)
}

fn part_2() -> i64 {
    let re: Regex = Regex::new("[A-Z0-9]+").unwrap();

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

    let matcher: Regex = Regex::new("[A-Z0-9][A-Z0-9]Z").unwrap();
    let starting_positions = find_starting_nodes(&directions);

    let steps_to_find_z = starting_positions.iter()
        .map(|v| find_number_of_steps(&instructions, v.0, &directions, &matcher) as i64)
        .collect::<Vec<i64>>();

    return lcm(steps_to_find_z);
}

fn find_starting_nodes<'a>(directions: &HashMap<&'a str, (&str, &str)>) -> Vec<(&'a str, i32)> {
    directions.iter().enumerate()
        .filter(|(_, (k, _))| k.ends_with('A'))
        .map(|(_, (k, _))| (*k, 0))
        .collect::<Vec<(&'a str, i32)>>()
}

fn find_number_of_steps(
    instructions: &Vec<char>,
    start: &str,
    directions: &HashMap<&str, (&str, &str)>,
    matcher: &Regex
) -> i32 {
    let mut index = 0;
    let mut curr: &str = start;
    let mut step_count = 0;

    loop {
        if index == instructions.len() {
            index = 0;
        }

        if matcher.is_match(curr) {
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

pub fn lcm(nums: Vec<i64>) -> i64 {
    if nums.len() == 1 {
        return nums[0];
    }
    let a = nums[0];
    let b = lcm(nums[1..].iter().map(|v| *v).collect::<Vec<i64>>());
    a * b / gcd_of_two_numbers(a, b)
}

fn gcd_of_two_numbers(a: i64, b: i64) -> i64 {
    if b == 0 {
        return a;
    }
    gcd_of_two_numbers(b, a % b)
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use regex::Regex;

    use super::find_number_of_steps;

    #[test]
    fn should_find_correct_number_of_steps_for_standard_ending() {
        let instructions1 = vec!('R', 'L');
        let directions1: HashMap<&str, (&str, &str)> = HashMap::from([
            ("AAA", ("BBB", "CCC")),
            ("BBB", ("DDD", "EEE")),
            ("CCC", ("ZZZ", "GGG")),
            ("DDD", ("DDD", "DDD")),
            ("EEE", ("EEE", "EEE")),
            ("GGG", ("GGG", "GGG")),
            ("ZZZ", ("ZZZ", "ZZZ")),
        ]);
        let matcher: Regex = Regex::new("ZZZ").unwrap();
        assert_eq!(find_number_of_steps(&instructions1, "AAA", &directions1, &matcher), 2);

        let instructions2 = vec!('L', 'L', 'R');
        let directions2: HashMap<&str, (&str, &str)> = HashMap::from([
            ("AAA", ("BBB", "BBB")),
            ("BBB", ("AAA", "ZZZ")),
            ("ZZZ", ("ZZZ", "ZZZ")),
        ]);
        let matcher: Regex = Regex::new("ZZZ").unwrap();
        assert_eq!(find_number_of_steps(&instructions2, "AAA", &directions2, &matcher), 6);
    }

    #[test]
    fn should_find_correct_number_of_steps_for_advanced_ending() {
        let instructions1 = vec!('L', 'R');
        let directions1: HashMap<&str, (&str, &str)> = HashMap::from([
            ("11A", ("11B", "XXX")),
            ("11B", ("XXX", "11Z")),
            ("11Z", ("11B", "XXX")),
            ("22A", ("22B", "XXX")),
            ("22B", ("22C", "22C")),
            ("22C", ("22Z", "22Z")),
            ("22Z", ("22B", "22B")),
            ("XXX", ("XXX", "XXX")),
        ]);
        let matcher: Regex = Regex::new("[A-Z0-9][A-Z0-9]Z").unwrap();

        assert_eq!(find_number_of_steps(&instructions1, "11A", &directions1, &matcher), 2);
        assert_eq!(find_number_of_steps(&instructions1, "22A", &directions1, &matcher), 3);
    }
}

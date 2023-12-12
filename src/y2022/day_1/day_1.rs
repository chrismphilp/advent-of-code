use std::fs;

pub fn process_answer() {
    println!("The sum for day ones max calories is: {}", part_1());
    println!("The sum for day ones top three calories is: {}", part_2());
}

fn part_1() -> i32 {
    let input = fs::read_to_string("src/y2022/day_1/input.txt").unwrap();
    let lines = input.split("\n\n").collect::<Vec<&str>>();
    lines.iter().map(|v| find_total_calories(*v)).max().unwrap()
}

fn find_total_calories(group: &str) -> i32 {
    group.lines()
        .map(|v| v.trim())
        .map(|v| v.lines().map(|v| v.parse::<i32>().unwrap()).sum::<i32>())
        .sum()
}

fn part_2() -> i32 {
    let input = fs::read_to_string("src/y2022/day_1/input.txt").unwrap();
    let lines = input.split("\n\n").collect::<Vec<&str>>();
    let mut vals = lines.iter().map(|v| find_total_calories(*v)).collect::<Vec<i32>>();
    vals.sort();
    vals[vals.len() - 1] + vals[vals.len() - 2] + vals[vals.len() - 3]
}

#[cfg(test)]
mod test {
    use super::find_total_calories;

    #[test]
    fn should_find_correct_number_of_calories() {
        assert_eq!(find_total_calories("\
        1000
        2000
        3000
        "), 6000);
    }
}
